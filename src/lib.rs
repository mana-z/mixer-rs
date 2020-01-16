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
    pub fn new(sink: S) -> Self {
        Self { tracks: Vec::new(), sink: sink }
    }

    pub fn do_frame(&mut self, size: usize)
    {
        let mut result : Vec<T> = std::iter::repeat(T::audio_default())
                                   .take(size)
                                   .collect();
        let tracks_count = self.tracks.len();
        for i in 0..tracks_count {
            let buf = self.tracks[i].get(size);
            for j in 0..size {
                result[j] = result[j].audio_add(buf[j]);
            }
        }
        self.sink.put(&result);
    }
}

