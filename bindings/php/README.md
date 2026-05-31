# uuid64-php

PHP bindings for
[uuid64](https://github.com/Dragonrun1/uuid64),
a Rust library that generates and encodes UUIDs as compact 22-character URL-safe
base64 strings.

A standard UUID like `550e8400-e29b-41d4-a716-446655440000` becomes
`VgBAAOKbQdSnFkRmVUQAAA` — 22 characters instead of 36, with no hyphens, safe to
use in URLs, filenames, and database keys without escaping.

## Requirements

- PHP 7.4 or later
- The `ffi` extension enabled (`extension=ffi`, `ffi.enable=true` in `php.ini`)
- The compiled `libuuid64_ffi` shared library for your platform

## Installation

Download the latest release for your platform from the
[releases page](https://github.com/Dragonrun1/uuid64/releases).
Each release archive contains:

- `libuuid64_ffi.so` / `libuuid64_ffi.dylib` / `uuid64_ffi.dll` (platform shared library)
- `uuid64.h` (C header)
- `Uuid64.php` (this wrapper)

Extract the archive and place the files somewhere accessible to your PHP application.

## Usage

See [`examples/Usage.php`](examples/Usage.php) for a runnable example.

```php
<?php

require_once 'Uuid64.php';

$uuid64 = new Uuid64('/path/to/libuuid64_ffi.so');

// Generate encoded UUIDs directly
$v4 = $uuid64->newV4(); // e.g. "3mJK9vQw2xRnP8tL4hGcY1"
$v7 = $uuid64->newV7(); // e.g. "-OtvDca6Rk1CFnKiuqty0k" (time-sortable)

// Encode a standard UUID string to its compact form
$encoded = $uuid64->encodeUuid('550e8400-e29b-41d4-a716-446655440000');

// Decode a compact UUID back to standard form
$decoded = $uuid64->decodeUuid($encoded);

// Invalid input returns null rather than throwing
$result = $uuid64->encodeUuid('not-a-uuid'); // null
```

## API

### `new Uuid64(string $libPath)`

Loads the shared library from the given path and initializes the FFI bindings.

### `newV4(): string`

Generates a new random (v4) UUID and returns it in compact base64 form.

### `newV7(): string`

Generates a new time-ordered (v7) UUID and returns it in compact base64 form.
v7 UUIDs sort chronologically, making them preferable to v4 for database primary
keys.

### `encodeUuid(string $uuid): ?string`

Encodes a standard hyphenated UUID string
(e.g. `550e8400-e29b-41d4-a716-446655440000`)
into its 22-character compact form.
Returns `null` if the input is not a valid UUID.

### `decodeUuid(string $encoded): ?string`

Decodes a 22-character compact UUID back to standard hyphenated form.
Returns `null` if the input is not a valid encoded UUID.

## Memory management

The shared library allocates the strings it returns.
The `Uuid64` wrapper handles freeing that memory internally — callers never need
to manage it explicitly.

## Building from source

If you need to compile the library yourself:

```bash
git clone https://github.com/Dragonrun1/uuid64.git
cd uuid64
cargo build --release --package uuid64-ffi
# Output: target/release/libuuid64_ffi.so (Linux)
#         target/release/libuuid64_ffi.dylib (macOS)
#         target/release/uuid64_ffi.dll (Windows)
```

Rust 1.75 or later is required. Install it via [rustup](https://rustup.rs).

## License

Licensed under your choice of:

- [BSD 3-Clause License](../../LICENSE-BSD.md)
- [GNU Lesser General Public License v3.0 or later](../../LICENSE-LGPL.md)
