////////////////////////////////////////////////////////////////////////////////
//
// Copyright 2020 Martin Zalabak
//
// Permission to use, copy, modify, and/or distribute this software for
// any purpose with or without fee is hereby granted, provided that the
// above copyright notice and this permission notice appear in all
// copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL
// WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE
// AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL
// DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR
// PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
// TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
// PERFORMANCE OF THIS SOFTWARE.
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

impl<T: AudioSample> SoundEntity for ImpulseEachFrame<T> {
    // dummy as is irrelevant for tests
    no_samplerate!();
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

impl<T: AudioSample> SoundEntity for Buffer<T> {
    // dummy as is irrelevant for tests
    no_samplerate!();
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
    e.effects.push(newgain());
    e.effects.push(newgain());
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
        e.push(newgain());
        e.push(newgain());
    }
    assert_eq!(vec!(132u8, 128u8, 128u8), t.get(3));
}

#[test]
fn mixer() {
    let impulse_track = || {
        Track::new(Box::new(ImpulseEachFrame::new(129u8, 128u8)), 1f32)
    };

    let mut m = Mixer::new(Buffer::new());
    m.tracks.push(impulse_track());
    m.tracks.push(impulse_track());
    m.do_frame(3);
    assert_eq!(vec!(130u8, 128u8, 128u8), m.sink.b);
}
