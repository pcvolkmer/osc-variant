/*
 * This file is part of osc-variant
 *
 * Copyright (C) 2023-2025 the original author or authors.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; either version 2 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along
 * with this program; if not, write to the Free Software Foundation, Inc.,
 * 51 Franklin Street, Fifth Floor, Boston, MA 02110-1301 USA.
 */

#ifndef OSC_VARIANT_DEOB_H
#define OSC_VARIANT_DEOB_H

#include <stdio.h>
#include <string.h>
#include <stdlib.h>

#define S0 8373972096940928081
#define S1 7378413942531504450
#define CS (char*)&s
#define I2 i*2
#define INZ(var) in[var] = 0
#define LD 16
#define DLS(idx) in[idx] = (d[LD+1]<<4)|d[LD]
#define DLT(idx) d[LD+idx] = (in[I2+idx] == d[j]) ? (char)j : d[LD+idx]
#define F(var) free(var)

void deob(char *in);

#endif //OSC_VARIANT_DEOB_H
