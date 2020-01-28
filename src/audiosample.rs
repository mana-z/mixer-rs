////////////////////////////////////////////////////////////////////////////////
//
// Copyright 2020 Martin Zalabak
//
// Permission to use, copy, modify, and/or distribute this software for
// any purpose with or without fee is hereby granted, provided that the
// above copyright notice and this permission notice appear in all
// copies.
//
// THE SOFTWARE IS PROVIDED "AS IS" AND THE AUTHOR DISCLAIMS ALL
// WARRANTIES WITH REGARD TO THIS SOFTWARE INCLUDING ALL IMPLIED
// WARRANTIES OF MERCHANTABILITY AND FITNESS. IN NO EVENT SHALL THE
// AUTHOR BE LIABLE FOR ANY SPECIAL, DIRECT, INDIRECT, OR CONSEQUENTIAL
// DAMAGES OR ANY DAMAGES WHATSOEVER RESULTING FROM LOSS OF USE, DATA OR
// PROFITS, WHETHER IN AN ACTION OF CONTRACT, NEGLIGENCE OR OTHER
// TORTIOUS ACTION, ARISING OUT OF OR IN CONNECTION WITH THE USE OR
// PERFORMANCE OF THIS SOFTWARE.
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
