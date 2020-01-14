pub trait AudioSample : Copy + Default {
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
#[cfg(test)]
mod tests {
    use super::*;
    

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
