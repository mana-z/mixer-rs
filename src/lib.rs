use std::iter::FromIterator;

pub trait SoundEntity {
    fn set_samplerate(&mut self, rate: u32);
    fn samplerate(&self) -> u32;
}

pub trait SoundSource<T: Copy + Default> : SoundEntity {
    fn get_out_channel_count(&self) -> usize;

    fn load_into(&mut self, result: &mut [T]);
    fn get<B>(&mut self, frame_size: usize) -> B where B: FromIterator<T> {
        let mut result = Vec::with_capacity(frame_size);
        for _ in 0..frame_size {
            result.push(T::default());
        }
        self.load_into(&mut result);
        B::from_iter(result)
    }

}

pub trait SoundEffect<T: Copy> : SoundEntity {
    fn pass(input: &[T], out: &mut [T]);
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
