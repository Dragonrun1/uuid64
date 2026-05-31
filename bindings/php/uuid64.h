// SPDX-FileCopyrightText: 2026 Michael Cummings <mgcummings@yahoo.com>
// SPDX-License-Identifier: BSD-3-Clause OR LGPL-3.0-or-later

#ifndef UUID64_H
#define UUID64_H

char* uuid64_new_v4(void);
char* uuid64_new_v7(void);
char* uuid64_encode_uuid(const char* ptr);
char* uuid64_decode_uuid(const char* ptr);
void  uuid64_free(char* ptr);

#endif
