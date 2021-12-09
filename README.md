# serde-window-handle

Serde-compatible window handle types, inter-convertible with `raw-window-handle`.

## Usage

This crate pretty much just replaces the pointer types with `usize`s and derives
`Serialize` and `Deserialize` traits. My use case was passing these window handles
through `deno`'s `serde_v8` layer.

Naming is also same except structs are prefixed with `Serde`, and the
`RawWindowHandle` struct becomes `SerdeWindowHandle`.

All the structs and enums implement `Into<T>` and `From<T>` for conversion
between `raw-window-handle` types.

## License

Check [LICENSE](./LICENSE) for more info.

Copyright 2021 @ DjDeveloperr
