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
use super::audiosample::AudioSample;
use std::iter::repeat;


/// A source of sound of X channels
///
/// A trait which should be implemented by sources, e.g. input, synths, ...
pub trait SoundSource<T: AudioSample> {
    /// Get number of emmiting channels channels
    ///
    /// How are the channels represented in input is user-defined
    fn get_out_channel_count(&self) -> usize;

    /// Load source data into a container
    ///
    /// This will provide better performance over `get` as it saves an allocation
    fn load_into(&mut self, result: &mut [T]);

    /// Allocate a vector and load new data into it
    fn get(&mut self, frame_size: usize) -> Vec<T> {
        let mut result : Vec<T> = repeat(T::audio_default())
            .take(frame_size)
            .collect();

        self.load_into(&mut result);
        result
    }

}

/// A sink of sound of X channels
///
/// A trait which should be implemented by output endpoints, e.g. outputs, file writers...
pub trait SoundSink<T: AudioSample> {
    /// Get number of consuming channels
    ///
    /// How should channels be represented in output is user-defined
    fn get_in_channel_count(&self) -> usize;

    /// Put data into the sink
    fn put(&mut self, data: &[T]);
}

/// A pass-through sound element
///
/// A trait which should be implemented by an intermediate sound element, e.g. sound effect, filter,
/// ...
/// Inputs and outputs are to be considered of a same length
pub trait SoundPassthrough<T>
where T: AudioSample {
    /// pass sound data through the element and get the result
    fn pass(&mut self, input: &[T], output: &mut [T]);

    /// Allocate a vector and pass data into it
    fn get(&mut self, input: &[T]) -> Vec<T> {
        let mut result : Vec<T> = repeat(T::audio_default())
            .take(input.len())
            .collect();
        self.pass(input, &mut result);
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Copier{}

    impl<T: AudioSample> SoundPassthrough<T> for Copier {
        fn pass(&mut self, input: &[T], output: &mut [T]) {
            output.copy_from_slice(input);
        }
    }

    #[test]
    fn passtrough() {
        let mut dummy = Copier{};
        let vec1 : Vec<u8> = vec!{1, 2, 3};
        let vec2 : Vec<u8> = dummy.get(&vec1);
        assert_eq!(vec1, vec2);

    }
}
