# tiny-rlua -- High level bindings between Rust and Lua for embedded-ish 32-bit RISC-V targets

[Guided Tour](examples/guided_tour.rs)

This library is a high level interface between Rust and Lua.  Its goal is to be
an easy to use, practical, flexible, and *safe* API between Rust and Lua.

## API stability

Currently, this library follows a pre-1.0 semver, so all API changes should be
accompanied by 0.x version bumps.  See the [Version 1.0
milestone](https://github.com/amethyst/rlua/milestone/1) for the work planned
to be done before a more stable 1.0 release.  There may be breaking changes as
these issues are dealt with on the way (the version number will be bumped as
needed).

## Lua versions supported

Only Lua 5.1 is supported at the moment.

Alternative Lua implementations (such as Luajit) are not currently supported, 
but are expected in the near future.

## Safety and Panics

This Library is not in any way safe or secure at the moment. It is only made
available as a proof of concept.

## License

rlua is licensed under the [MIT license](LICENSE)
