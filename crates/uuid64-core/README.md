# uuid64-core

Core encoding and decoding logic for compact, URL-safe, base64-encoded UUIDs.

A standard UUID like `550e8400-e29b-41d4-a716-446655440000`
becomes `VgBAAOKbQdSnFkRmVUQAAA`
— 22 characters instead of 36, safe to use in URLs, filenames, and database keys
without escaping or encoding.

This crate is the pure Rust building block.
If you need to call the library from another language, see
[`uuid64-ffi`](../uuid64-ffi).

## Usage

```toml
[dependencies]
uuid64-core = "0.1"
```

### Generating UUIDs

```rust,no_run
use uuid64_core::Uuid64;

// Random (v4)
let encoded = Uuid64::new_v4();
println!("{encoded}"); // e.g. "3mJK9vQw2xRnP8tL4hGcY1"

// Time-ordered (v7) — preferable for database primary keys
let encoded = Uuid64::new_v7();
println!("{encoded}"); // e.g. "-OtvDca6Rk1CFnKiuqty0k"
```

### Encoding and decoding

```rust,no_run
use uuid64_core::Uuid64;

// Encode a standard UUID string
let encoded = Uuid64::encode_uuid("550e8400-e29b-41d4-a716-446655440000")?;
assert_eq!(encoded.len(), 22);

// Decode back to standard form
let decoded = Uuid64::decode_uuid(&encoded)?;
assert_eq!(decoded, "550e8400-e29b-41d4-a716-446655440000");

// Encode/decode using the uuid::Uuid type directly
use uuid::Uuid;
let uuid   = Uuid::parse_str("550e8400-e29b-41d4-a716-446655440000")?;
let encoded = Uuid64::encode(uuid);
let decoded: Uuid = Uuid64::decode(&encoded)?;
```

## API

### Generating

| Function           | Returns  | Description                |
|--------------------|----------|----------------------------|
| `Uuid64::new_v4()` | `String` | Random UUID, encoded       |
| `Uuid64::new_v7()` | `String` | Time-ordered UUID, encoded |

### Encoding

| Function                 | Input        | Returns                     | Description                     |
|--------------------------|--------------|-----------------------------|---------------------------------|
| `Uuid64::encode(uuid)`   | `uuid::Uuid` | `String`                    | Encode a `Uuid` value           |
| `Uuid64::encode_uuid(s)` | `&str`       | `Result<String, UuidError>` | Encode a hyphenated UUID string |

### Decoding

| Function                 | Input  | Returns                     | Description                        |
|--------------------------|--------|-----------------------------|------------------------------------|
| `Uuid64::decode(s)`      | `&str` | `Result<Uuid, UuidError>`   | Decode to a `Uuid` value           |
| `Uuid64::decode_uuid(s)` | `&str` | `Result<String, UuidError>` | Decode to a hyphenated UUID string |

### Errors

`UuidError` is returned when encoding or decoding fails:

| Variant                        | Cause                                                    |
|--------------------------------|----------------------------------------------------------|
| `InvalidBase64Length(usize)`   | Encoded string is not exactly 22 characters              |
| `InvalidBase64Character(char)` | Encoded string contains a character outside the alphabet |
| `InvalidUuid(uuid::Error)`     | Input string is not a valid UUID                         |

## Alphabet

The 64-character URL-safe alphabet used, in index order:

```
- 0 1 2 3 4 5 6 7 8 9
A B C D E F G H I J K L M N O P Q R S T U V W X Y Z
_ a b c d e f g h i j k l m n o p q r s t u v w x y z
```

This matches the RFC 4648 URL-safe base64 character set, making encoded strings
safe for use in URLs, filenames, and HTTP headers without percent-encoding.

## License

Licensed under your choice of:

- [BSD 3-Clause License](../../LICENSE-BSD.md)
- [GNU Lesser General Public License v3.0 or later](../../LICENSE-LGPL.md)
