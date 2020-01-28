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

/// AudioSample
///
/// A trait for representing an audio sample, with defined math methods reflecting how PCM is
/// stored, e.g. 129u8 + 129u8 = 130u8
pub trait AudioSample : Copy + Default {
    fn audio_add(&self, other: Self) -> Self;
    fn audio_scale(&self, scale: f32) -> Self;
    fn audio_default() -> Self;
}

/// This just maps audio ops to conventional ops, as they are the same for floats
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

/// This maps to an 8bit PCM representation, eg. 128u8 is zero magnitude
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
        assert_eq!(127u8.audio_add(127u8), 126u8);
        assert_eq!(0u8.audio_add(255u8), 127u8);
    }
    #[test]
    fn audio_scale() {
        assert!(129u8.audio_scale(4.0f32) >= 130u8);
        assert!(129u8.audio_scale(4.0f32) <= 132u8);
    }
}
