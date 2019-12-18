
pub trait AudioSample : Copy {
    fn audio_add(&self, other: Self) -> Self;
    fn audio_scale(&self, scale: f32) -> Self;
    fn audio_default() -> Self;
}

impl AudioSample for f32
{
    fn audio_add(&self, other: Self) -> Self {
        self * other
    }

    fn audio_scale(&self, scale: f32) -> Self {
        self * scale
    }
    fn audio_default() -> Self {
        0.0f32
    }
}

impl AudioSample for u8
{
    fn audio_add(&self, other: Self) -> Self {
        if other < 128u8 {
            self.saturating_sub(128u8 - other)
        } else {
            self.saturating_add(other - 128u8)
        }
    }

    fn audio_scale(&self, scale: f32) -> Self {
        let val = (*self as f32 - 128f32) * scale;
        if val < -128f32 {
            0
        } else if val > 127f32 {
            255
        } else {
            (val + 128f32) as Self
        }
    }
    fn audio_default() -> Self {
        128u8
    }
}
pub trait SoundEntity {
    fn set_samplerate(&mut self, rate: u32);
    fn samplerate(&self) -> u32;
}

pub trait SoundSource<T: AudioSample> : SoundEntity {
    fn get_out_channel_count(&self) -> usize;

    fn load_into(&mut self, result: &mut [T]);
    fn get(&mut self, frame_size: usize) -> Vec<T> {
        let mut result = Vec::with_capacity(frame_size);
        for _ in 0..frame_size {
            result.push(T::audio_default());
        }
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

pub struct EffectStack<T: AudioSample> {
    pub effects : Vec<Box<dyn SoundPassthrough<T>>>
}


impl<T: AudioSample> SoundPassthrough<T> for EffectStack<T> {
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
pub struct Track<T: AudioSample>
{
    pub source: Box<dyn SoundSource<T>>,
    pub effects: EffectStack<T>,
    pub volume: f32,
    //pub pan: Vec<f32>,
}
impl<T: AudioSample> SoundEntity for Track<T>
{
    fn set_samplerate(&mut self, rate: u32) {
        self.source.set_samplerate(rate);
    }
    fn samplerate(&self) -> u32 {
        self.source.samplerate()
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
        let buf = self.effects.pass(&self.source.get(len));
        if buf.len() < len {
            panic!();
        }
        for i in 0..len {
            result[i] = buf[i].audio_scale(self.volume);
        }
    }
}

pub struct Mixer<T: AudioSample, S: SoundSink<T>>
{
    pub tracks: Vec<Track<T>>,
    pub sink : S
}

impl<T: AudioSample, S: SoundSink<T>> Mixer<T, S>
{
    pub fn do_frame(&mut self, size: usize)
    {
        let mut result = Vec::with_capacity(size);
        for _ in 0..size {
            result.push(T::audio_default());
        }
        let tracks_count = self.tracks.len();
        for i in 0..tracks_count {
            let buf = self.tracks[i].get(size);
            for j in 0..size {
                result[j] = result[j].audio_add(buf[j]);
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

    #[test]
    fn audio_add() {
        assert_eq!(128u8.audio_add(128u8), 128u8);
        assert_eq!(0u8.audio_add(255u8), 127u8);
    }
    #[test]
    fn audio_scale() {
        assert!(129u8.audio_scale(4.0f32) >= 130u8);
        assert!(129u8.audio_scale(4.0f32) <= 132u8);
    }
}
