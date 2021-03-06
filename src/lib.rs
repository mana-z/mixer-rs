////////////////////////////////////////////////////////////////////////////////
//
// Copyright 2020 Martin Zalabak
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.
//
////////////////////////////////////////////////////////////////////////////////
pub mod audiosample;
pub mod io;
use audiosample::AudioSample;
use io::*;
use std::collections::BTreeMap;

/// Effects Stack
///
/// Contains a dynamic cascade of "effects", trait objects able to pass through. Also implements
/// pass through as well for easy traversing through the cascade
#[derive(Default)]
pub struct EffectStack<T: AudioSample> {
    pub effects : BTreeMap<usize, Box<dyn SoundPassthrough<T>>>,
    buffer_flip: Vec<T>,
    buffer_flop: Vec<T>,
}

impl<T: AudioSample> EffectStack<T> {
    pub fn new() -> Self {
        EffectStack {
            effects: BTreeMap::new(),
            buffer_flip: Vec::new(),
            buffer_flop: Vec::new(),
        }
    }
}


impl<T: AudioSample> SoundPassthrough<T> for EffectStack<T> {
    fn pass(&mut self, input: &[T], output: &mut [T]){
        let len = self.effects.len();
        match len {
            0 => output.copy_from_slice(input),
            1 => self.effects.iter_mut().next().unwrap().1.pass(input, output),
            _ => {
                prepare_vec_for_slicing(&mut self.buffer_flip, input.len());
                prepare_vec_for_slicing(&mut self.buffer_flop, input.len());
                let mut iter = self.effects.iter_mut();
                iter.next().unwrap().1.pass(input, &mut self.buffer_flip);
                for _ in 1..(len - 1) {
                    iter.next().unwrap().1.pass(&self.buffer_flip, &mut self.buffer_flop);
                    std::mem::swap(&mut self.buffer_flip, &mut self.buffer_flop);
                }
                iter.next().unwrap().1.pass(&self.buffer_flip, output);
            }
        }

    }
}


/// Track
///
/// Contains a source trait object, an effects stack and controls for adjusting.
/// Works as sound source as well
pub struct Track<T: AudioSample>
{
    pub source: Box<dyn SoundSource<T>>,
    pub effects: EffectStack<T>,
    pub volume: f32,
    buffer_flip: Vec<T>,
    buffer_flop: Vec<T>,
    //pub pan: Vec<f32>,
}

impl<T: AudioSample> Track<T> {
    pub fn new(source: Box<dyn SoundSource<T>>, volume: f32) -> Self {
        Track {
            source : source,
            effects: Default::default(),
            volume: volume,
            buffer_flip: Vec::new(),
            buffer_flop: Vec::new(),
        }
    }
}

impl<T: AudioSample> SoundSource<T> for Track<T>
{
    fn get_out_channel_count(&self) -> usize {
        self.source.get_out_channel_count()
    }

    fn load_into(&mut self, result: &mut [T])
    {
        let len = result.len();
        prepare_vec_for_slicing(&mut self.buffer_flip, len);
        prepare_vec_for_slicing(&mut self.buffer_flop, len);
        self.source.load_into(&mut self.buffer_flip);
        self.effects.pass(&self.buffer_flip, &mut self.buffer_flop);
        if self.buffer_flop.len() < len {
            panic!();
        }
        for i in 0..len {
            result[i] = self.buffer_flop[i].audio_scale(self.volume);
        }
    }
}


/// Mixer
///
/// A collection of tracks collected into a single sink
pub struct Mixer<T: AudioSample, S: SoundSink<T>>
{
    pub tracks: BTreeMap<usize, Track<T>>,
    pub sink : S,
    out_buffer: Vec<T>,
    track_buffer: Vec<T>,
}

impl<T: AudioSample, S: SoundSink<T>> Mixer<T, S>
{
    pub fn new(sink: S) -> Self {
        Self {
            tracks: BTreeMap::new(),
            sink: sink,
            out_buffer: Vec::new(),
            track_buffer: Vec::new(),
        }
    }

    pub fn do_frame(&mut self, size: usize)
    {
        // we need track buffer preloaded for having a writable slice of the desired length
        // we also need default values in output buffer as the results are accumulated
        prepare_vec_for_slicing(&mut self.track_buffer, size);
        prepare_vec_for_slicing(&mut self.out_buffer, size);
        for i in self.tracks.iter_mut() {
            i.1.load_into(&mut self.track_buffer);
            for j in 0..size {
                self.out_buffer[j] = self.out_buffer[j].audio_add(self.track_buffer[j]);
            }
        }
        self.sink.put(&self.out_buffer);
    }
}

fn prepare_vec_for_slicing<T: AudioSample>(vec: &mut Vec<T>, size: usize) {
    vec.clear();
    vec.extend(std::iter::repeat(T::audio_default()).take(size));
}
