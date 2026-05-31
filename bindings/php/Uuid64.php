// SPDX-FileCopyrightText: 2026 Michael Cummings <mgcummings@yahoo.com>
// SPDX-License-Identifier: BSD-3-Clause OR LGPL-3.0-or-later
<?php

class Uuid64
{
    private \FFI $ffi;

    public function __construct(string $libPath)
    {
        $this->ffi = \FFI::cdef(
            file_get_contents(__DIR__ . '/uuid64.h'),
            $libPath
        );
    }

    public function newV4(): string
    {
        return $this->call('uuid64_new_v4');
    }

    public function newV7(): string
    {
        return $this->call('uuid64_new_v7');
    }

    public function encodeUuid(string $uuid): ?string
    {
        $input = $this->ffi->new('char[37]');
        \FFI::memcpy($input, $uuid, strlen($uuid));
        $ptr = $this->ffi->uuid64_encode_uuid($input);
        if (\FFI::isNull($ptr)) {
            return null;
        }
        $result = \FFI::string($ptr);
        $this->ffi->uuid64_free($ptr);
        return $result;
    }

    public function decodeUuid(string $encoded): ?string
    {
        $input = $this->ffi->new('char[23]');
        \FFI::memcpy($input, $encoded, strlen($encoded));
        $ptr = $this->ffi->uuid64_decode_uuid($input);
        if (\FFI::isNull($ptr)) {
            return null;
        }
        $result = \FFI::string($ptr);
        $this->ffi->uuid64_free($ptr);
        return $result;
    }

    private function call(string $fn): string
    {
        $ptr = $this->ffi->$fn();
        $result = \FFI::string($ptr);
        $this->ffi->uuid64_free($ptr);
        return $result;
    }
}
