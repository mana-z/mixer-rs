# mixer-rs

A Rust implementation of usual sound patchwork needed for projects needing more than "play one
sound". It should be generic enough for common use cases.

There is of course, the typical Rust stuff, like traits, zero-cost abstractions, taking over the
world, etc. etc..

## What is in the box...um, crate?

- An `AudioSample` trait, to cover different math concerning e.g. common PCM sample representation
  and classic IEEE-754 floats.
- `SoundSource`, `SoundSink` and `SoundPassthrough` traits as a generic slots for sound
  sources/sinks/effects given by user's choice of sound system.

- `EffectStack`, `Track` and `Mixer` structs for doing the magic using the user implementations of
  above traits. 

## Status

> It may work somehow

### Known limitations

- only u8 (8-bit PCM) and f32 (IEEE 754 float) supported as audiosamples as default

You should rarely use something different than float for DSP. The 8-bit PCM was added for
having easier testing and for potential possibility of an easy and effective chiptune stack

If you _have_ to use a different sample format, you still have two major possibilities:
    - Have a sinks and sources mapping the inputs and outputs to float (recommended)
    - implement an AudioSample trait


- limited multichannel support

The library is agnostic to how is the data structured in the buffers, so you may pass any number of
channels you want. However, this also means that it will not help you with panning and other
channel-wise operations.
    - You may use a passthrough effect for that

The library also requires that the input and output buffers for passthroughs are of same length, so
you cannot replicate or downmix channels using it.
    - You can use a struct implementing both Sink and Source for that, but you still cannot put it
      into an effects stack.


## License

MIT, see COPYING for details
