
pub trait SoundEntity {
    fn set_samplerate(&mut self, rate: u32);
    fn samplerate(&self) -> u32;
}

pub trait SoundSource<T: Copy + Default> : SoundEntity {
    fn get_out_channel_count(&self) -> usize;

    fn load_into(&mut self, result: &mut [T]);
    fn get(&mut self, frame_size: usize) -> Vec<T> {
        let mut result = Vec::with_capacity(frame_size);
        for _ in 0..frame_size {
            result.push(T::default());
        }
        self.load_into(&mut result);
        result
    }

}

pub trait SoundSink<T: Copy + Default> : SoundEntity {
    fn put(&mut self, data: &[T]);
}

pub trait SoundPassthrough<T: Copy + Default> {
    fn pass(&mut self, input: &[T]) -> Vec<T>;
}

pub struct EffectStack<T: Copy + Default> {
    pub effects : Vec<Box<dyn SoundPassthrough<T>>>
}


impl<T: Copy + Default> SoundPassthrough<T> for EffectStack<T> {
    fn pass(&mut self, input: &[T]) -> Vec<T> {
        let len = self.effects.len();
        match len {
            0 => Vec::from(input),
            1 => self.effects[0].pass(input),
            _ => {
                let mut buf = self.effects[0].pass(input);
                for i in 1..len {
                    let mut buf2 = self.effects[i].pass(&buf);
                    std::mem::swap(&mut buf, &mut buf2);
                }
                buf
            }
        }

    }
}





#[cfg(test)]
mod tests {
    use super::*;
    
    struct Copier{}

    impl SoundEntity for Copier {
        fn set_samplerate(&mut self, _: u32) {}
        fn samplerate(&self) -> u32 {0}
    }

    impl<T: Copy + Default> SoundPassthrough<T> for Copier {
        fn pass(&mut self, input: &[T]) -> Vec<T> {
            Vec::from(input)
        }
    }

    #[test]
    fn passtrough() {
        let mut dummy = Copier{};
        let vec1 : Vec<i32> = vec!{1, 2, 3};
        let vec2 : Vec<i32> = dummy.pass(&vec1);
        assert_eq!(vec1, vec2);

    }
}
