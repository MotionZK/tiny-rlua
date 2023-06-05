# tiny-rlua -- High level Rust <-> Lua bindings for embedded-ish 32-bit RISC-V devices

[Guided Tour](examples/guided_tour.rs)

This library is a high level interface between Rust and Lua.  Its goal is to be
an easy to use, practical, flexible, and *safe* API between Rust and Lua.

## Safety + API stability

This crate is not in any way stable and is only made
available as a proof of concept. There may be breaking changes as
these issues are dealt with on the way.

## Lua versions supported

Only Lua 5.1 is supported at the moment.

Alternative Lua implementations (such as Luajit) are not currently supported, 
but are expected in the near future.

## License

Amethyst's [rlua](https://github.com/amethyst/rlua), which tiny-rlua is based on, is licensed under the [MIT license](https://github.com/amethyst/rlua/)
