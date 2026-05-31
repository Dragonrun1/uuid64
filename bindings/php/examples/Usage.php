// SPDX-FileCopyrightText: 2026 Michael Cummings <mgcummings@yahoo.com>
// SPDX-License-Identifier: BSD-3-Clause OR LGPL-3.0-or-later
<?php

declare(strict_types=1);

require_once __DIR__ . '/../Uuid64.php';

// Adjust this path to where you placed the compiled shared library.
// On macOS use libuuid64_ffi.dylib, on Windows use uuid64_ffi.dll.
$libPath = __DIR__ . '/../libuuid64_ffi.so';

$uuid64 = new Uuid64($libPath);

// --- Generating UUIDs ---

$v4 = $uuid64->newV4();
echo "v4 (random):       {$v4}" . PHP_EOL;

$v7 = $uuid64->newV7();
echo "v7 (time-ordered): {$v7}" . PHP_EOL;

// --- Encoding a standard UUID ---

$standard = '550e8400-e29b-41d4-a716-446655440000';
$encoded  = $uuid64->encodeUuid($standard);
echo PHP_EOL;
echo "Standard UUID: {$standard}" . PHP_EOL;
echo "Encoded:       {$encoded}" . PHP_EOL;

// --- Decoding back to standard form ---

$decoded = $uuid64->decodeUuid($encoded);
echo "Decoded:       {$decoded}" . PHP_EOL;

// --- Round-trip check ---

assert($decoded === $standard, 'Round-trip encode → decode must produce the original UUID');
echo PHP_EOL . 'Round-trip assertion passed.' . PHP_EOL;

// --- Error handling: invalid input returns null ---

$badEncode = $uuid64->encodeUuid('not-a-uuid');
assert($badEncode === null, 'Invalid UUID input should return null');
echo PHP_EOL . "encodeUuid('not-a-uuid'): " . var_export($badEncode, true) . PHP_EOL;

$badDecode = $uuid64->decodeUuid('tooshort');
assert($badDecode === null, 'Invalid encoded input should return null');
echo "decodeUuid('tooshort'):   " . var_export($badDecode, true) . PHP_EOL;
