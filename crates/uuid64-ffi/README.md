# uuid64-ffi

C FFI bindings for [uuid64-core](../uuid64-core), exposing the UUID encoding
and decoding API as a C-compatible shared library.

Use this crate when integrating uuid64 with a language that can load native
libraries via FFI — PHP, Python, Ruby, Node.js, and so on. If you are writing
Rust, use `uuid64-core` directly instead.

## Using a pre-built release

Pre-compiled binaries for Linux, macOS, and Windows are available on the
[releases page](https://github.com/Dragonrun1/uuid64/releases).
Each release includes:

- The shared library for your platform
- `uuid64.h` — the C header
- Language-specific archives that bundle both together with a wrapper

## Building from source

```bash
cargo build --release --package uuid64-ffi
```

Output location:

| Platform | File                                 |
|----------|--------------------------------------|
| Linux    | `target/release/libuuid64_ffi.so`    |
| macOS    | `target/release/libuuid64_ffi.dylib` |
| Windows  | `target/release/uuid64_ffi.dll`      |

## C API

```c
// Generate a new random (v4) UUID, returned as a 22-character encoded string.
char* uuid64_new_v4(void);

// Generate a new time-ordered (v7) UUID, returned as a 22-character encoded string.
char* uuid64_new_v7(void);

// Encode a standard hyphenated UUID string to its 22-character compact form.
// Returns NULL if the input is not a valid UUID.
char* uuid64_encode_uuid(const char* ptr);

// Decode a 22-character compact UUID back to standard hyphenated form.
// Returns NULL if the input is not a valid encoded UUID.
char* uuid64_decode_uuid(const char* ptr);

// Free a string returned by any of the functions above.
// Must be called exactly once per non-NULL pointer. Safe to call with NULL.
void uuid64_free(char* ptr);
```

### Memory ownership

Every non-NULL string returned by this library is allocated by Rust and must
be freed by calling `uuid64_free`. Do not pass these pointers to the C
standard library `free()` — the memory is managed by Rust's allocator.

```c
char* encoded = uuid64_new_v7();
if (encoded) {
    printf("%s\n", encoded);
    uuid64_free(encoded);
}

char* decoded = uuid64_decode_uuid(encoded); // NULL on invalid input
if (decoded) {
    printf("%s\n", decoded);
    uuid64_free(decoded);
}
```

## Language bindings

Ready-made wrappers that handle memory management for you:

| Language | Location                             |
|----------|--------------------------------------|
| PHP 7.4+ | [`bindings/php`](../../bindings/php) |

## License

Licensed under your choice of:

- [BSD 3-Clause License](../../LICENSE-BSD.md)
- [GNU Lesser General Public License v3.0 or later](../../LICENSE-LGPL.md)
