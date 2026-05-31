# uuid64

Compact, URL-safe, base64-encoded UUIDs.

A standard UUID like `550e8400-e29b-41d4-a716-446655440000` becomes `VgBAAOKbQdSnFkRmVUQAAA`
— 22 characters instead of 36, safe to use in URLs, filenames, and database keys
without escaping or encoding.

Both UUID v4 (random) and v7 (time-ordered, suitable for database primary keys)
are supported.

## Repository layout

```
uuid64/
├── crates/
│   ├── uuid64-core/        Core encoding and decoding logic (pure Rust)
│   └── uuid64-ffi/         C FFI bindings (cdylib + header)
└── bindings/
    └── php/                PHP wrapper around the FFI library
```

## Crates

### `uuid64-core`

The pure Rust core. Encodes and decodes UUIDs to and from a 22-character
URL-safe base64 alphabet (`-`, `0–9`, `A–Z`, `_`, `a–z`). No unsafe code.
Add it as a dependency if you are writing Rust:

```toml
[dependencies]
uuid64-core = "0.1"
```

```rust,no_run
use uuid64_core::Uuid64;

let encoded = Uuid64::new_v7();               // "3mJK9vQw2xRnP8tL4hGcY1"
let decoded = Uuid64::decode_uuid(&encoded)?; // "550e8400-e29b-41d4-a716-446655440000"
```

### `uuid64-ffi`

Exposes the core API as a C-compatible shared library (`cdylib`). Use this
when integrating with any language that can load a native library via FFI.
The compiled output and C header are distributed as part of each
[GitHub release](#releases).

## Language bindings

| Language | Location                       | Install                             |
|----------|--------------------------------|-------------------------------------|
| PHP 7.4+ | [`bindings/php`](bindings/php) | Download from [releases](#releases) |

Additional language bindings are planned. Contributions welcome — see
[Contributing](#contributing).

## Releases

Each tagged release publishes pre-compiled binaries for Linux (x86\_64),
macOS (x86\_64), and Windows (x86\_64), along with language-specific archives
that bundle the library, header, and wrapper together.

Download from the [releases page](https://github.com/Dragonrun1/uuid64/releases).

## Building from source

Rust 1.75 or later is required. Install it via [rustup](https://rustup.rs).

```bash
git clone https://github.com/Dragonrun1/uuid64.git
cd uuid64

# Build everything
cargo build --release

# Run all tests
cargo test

# Check test coverage (requires cargo-llvm-cov)
cargo llvm-cov --lcov --output-path lcov.info
```

The compiled shared library is written to `target/release/`:

| Platform | File                  |
|----------|-----------------------|
| Linux    | `libuuid64_ffi.so`    |
| macOS    | `libuuid64_ffi.dylib` |
| Windows  | `uuid64_ffi.dll`      |

## Contributing

Bug reports and pull requests are welcome on
[GitHub](https://github.com/Dragonrun1/uuid64).

When adding a new language binding, place it under `bindings/<language>/` and
include a `README.md` that covers installation, basic usage, and how to obtain
the compiled library for each supported platform.

## License

Licensed under your choice of:

- [BSD 3-Clause License](LICENSE-BSD.md)
- [GNU Lesser General Public License v3.0 or later](LICENSE-LGPL.md)
