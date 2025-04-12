<div align="center">

# Ghx Grid

[![ghx_grid on crates.io](https://img.shields.io/crates/v/ghx_grid)](https://crates.io/crates/ghx_grid)
[![ghx_grid on doc.rs](https://docs.rs/ghx_grid/badge.svg)](https://docs.rs/ghx_grid)

</div>

Structures and utilities to manipulate 2d & 3d grid data.

## Cargo features

*Find the list and description in [Cargo.toml](Cargo.toml)*

- `bevy`: Disabled by default, enabling it simply derives `Component` on common structs of the crate.
- `reflect`: Disabled by default, enabling it simply derives `Reflect` on common structs of the crate.

## For Bevy users

#### Compatible Bevy versions

 | **ghx_grid** | **bevy** |
 | :----------- | :------- |
 | 0.6          | 0.16     |
 | 0.5          | 0.15     |
 | 0.4          | 0.14     |
 | 0.2-0.3      | 0.13     |
 | 0.1          | 0.12     |

See the [`bevy_ghx_grid`](https://github.com/Henauxg/bevy_ghx_grid) crate which uses and exposes `ghx_grid`, as well as additional plugins and utilities dedicated to [`Bevy`](https://github.com/bevyengine/bevy).

## Misc

#### Why "ghx" ?
- It serves as a namespace to avoid picking cargo names such as `grid` or `bevy_grid`

## License

### Code

ghx_grid is free and open source. All code in this repository is dual-licensed under either:

* MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
