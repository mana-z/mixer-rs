pub mod audiosample;
pub mod io;
use audiosample::AudioSample;
use io::*;

#[derive(Default)]
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
impl<T: AudioSample> Track<T> {
    pub fn new(source: Box<dyn SoundSource<T>>, volume: f32) -> Self {
        Track {
            source : source,
            effects: Default::default(),
            volume: volume
        }
    }
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
    pub sink : S,
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
    struct Gain {
        g: f32,
    }

    impl<T: AudioSample> SoundPassthrough<T> for Gain {
        fn pass(&mut self, input: &[T]) -> Vec<T> {
            input.iter().map(|i| i.audio_scale(self.g)).collect()
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
        fn set_samplerate(&mut self, _: u32) {}
        fn samplerate(&self) -> u32 {0}
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



    #[test]
    fn effect_stack() {
        let mut e = EffectStack{ effects: Vec::new() };
        let newgain = || Box::new(Gain{g: 2f32});
        e.effects.push(newgain());
        e.effects.push(newgain());
        assert_eq!(vec!(132u8, 136u8), e.pass(&[129u8, 130u8]));
        assert_eq!(vec!(124u8, 120u8), e.pass(&[127u8, 126u8]));
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

    //TODO mixer tests
}


