use super::audiosample::AudioSample;
use std::iter::repeat;

/// A base interface for sinks/entities/passtroughs
///
/// This contains samplerate getters/setters, as many apps may want to test it always, even if it is
/// not applicable for the entity itself
pub trait SoundEntity {
    fn set_samplerate(&mut self, rate: u32);
    fn samplerate(&self) -> Option<u32>;
}

#[macro_export]
macro_rules! no_samplerate {
    () => {
        fn set_samplerate(&mut self, _: u32) {}
        fn samplerate(&self) -> Option<u32> { None }
    }
}

/// A source of sound of X channels
///
/// A trait which should be implemented by sources, e.g. input, synths, ...
pub trait SoundSource<T: AudioSample> : SoundEntity {
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
pub trait SoundSink<T: AudioSample> : SoundEntity {
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
    impl SoundEntity for Copier { no_samplerate!(); }

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
