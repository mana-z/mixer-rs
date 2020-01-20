use super::audiosample::AudioSample;

/// A base interface for sinks/entities/passtroughs
///
/// This contains samplerate getters/setters, as many apps may want to test it always, even if it is
/// not applicable for the entity itself
/// Note: Ther
pub trait SoundEntity {
    fn set_samplerate(&mut self, rate: u32);
    fn samplerate(&self) -> Option<u32>;
}

pub trait SoundSource<T: AudioSample> : SoundEntity {
    fn get_out_channel_count(&self) -> usize;

    fn load_into(&mut self, result: &mut [T]);

    fn get(&mut self, frame_size: usize) -> Vec<T> {
        let mut result : Vec<T> = std::iter::repeat(Default::default())
            .take(frame_size)
            .collect();

        self.load_into(&mut result);
        result
    }

}

pub trait SoundSink<T: AudioSample> : SoundEntity {
    fn get_in_channel_count(&self) -> usize;
    fn put(&mut self, data: &[T]);
}

pub trait SoundPassthrough<T>
where
    T: AudioSample {
    fn pass(&mut self, input: &[T]) -> Vec<T>;
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Copier{}

    impl SoundEntity for Copier {
        fn set_samplerate(&mut self, _: u32) {}
        fn samplerate(&self) -> Option<u32> {None}
    }

    impl<T: AudioSample> SoundPassthrough<T> for Copier {
        fn pass(&mut self, input: &[T]) -> Vec<T> {
            Vec::from(input)
        }
    }

    #[test]
    fn passtrough() {
        let mut dummy = Copier{};
        let vec1 : Vec<u8> = vec!{1, 2, 3};
        let vec2 : Vec<u8> = dummy.pass(&vec1);
        assert_eq!(vec1, vec2);

    }
}
