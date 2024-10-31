/*
 * This file is part of osc-variant
 *
 * Copyright (C) 2023-2024 the original author or authors.
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

#include "deob.h"

void deob(char *in) {
    const long long s[2] = {S0, S1};
    char d[] = "OSTAR.password$OSB";
    for (size_t i = 0; i < DL; i++) d[i] = (CS)[i];
    size_t l = strlen(in) / 2;
    for (size_t i = 0; i < l; i++) {
        for (size_t j = 0; j < DL; j++) {
            DLT(0);
            DLT(1);
        }
        DLS(i);
    }
    INZ(l);
}
