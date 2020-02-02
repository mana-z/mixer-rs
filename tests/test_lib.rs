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
use mixer_rs::{
    *,
    io::*,
    audiosample::AudioSample
};

struct Gain {
    g: f32,
}

impl<T: AudioSample> SoundPassthrough<T> for Gain {
    fn pass(&mut self, input: &[T], output: &mut [T]) {
        for i in 0..input.len() {
            output[i] = input[i].audio_scale(self.g);
        }
    }
}


struct ImpulseEachFrame<T: AudioSample> {
    one: T,
    nul: T
}

impl<T: AudioSample> ImpulseEachFrame<T> {
    fn new(one: T, nul: T) -> Self {
        ImpulseEachFrame { one: one, nul: nul }
    }
}

impl<T: AudioSample> SoundSource<T> for ImpulseEachFrame<T> {
    fn get_out_channel_count(&self) -> usize {1}

    fn load_into(&mut self, result: &mut [T]) {
        if result.len() == 0 { return; }
        result[0] = self.one;
        for i in 1..result.len() {
            result[i] = self.nul
        }
    }
}


struct Buffer<T: AudioSample> {
    b : Vec<T>
}

impl<T: AudioSample> Buffer<T> {
    fn new() -> Self {
        Self { b: Vec::new() }
    }
}


impl<T: AudioSample> SoundSink<T> for Buffer<T> {
    fn put(&mut self, buf: &[T]) {
        self.b.extend(buf);
    }
    fn get_in_channel_count(&self) -> usize {1}
}


#[test]
fn effect_stack() {
    let mut e = EffectStack::new();
    let newgain = || Box::new(Gain{g: 2f32});
    e.effects.insert(0, newgain());
    e.effects.insert(1, newgain());
    assert_eq!(vec!(132u8, 136u8), e.get(&[129u8, 130u8]));
    assert_eq!(vec!(124u8, 120u8), e.get(&[127u8, 126u8]));
}


#[test]
fn track() {
    let mut t = Track::new(
        Box::new(ImpulseEachFrame::new(129u8, 128u8)),
        1f32);

    {
        let ref mut e = t.effects.effects;
        let newgain = || Box::new(Gain{g: 2f32});
        e.insert(0, newgain());
        e.insert(1, newgain());
    }
    assert_eq!(vec!(132u8, 128u8, 128u8), t.get(3));
}

#[test]
fn mixer() {
    let impulse_track = || {
        Track::new(Box::new(ImpulseEachFrame::new(129u8, 128u8)), 1f32)
    };

    let mut m = Mixer::new(Buffer::new());
    m.tracks.insert(0, impulse_track());
    m.tracks.insert(1, impulse_track());
    m.do_frame(3);
    assert_eq!(vec!(130u8, 128u8, 128u8), m.sink.b);
}
