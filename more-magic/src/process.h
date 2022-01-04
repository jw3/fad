/*
 * process.h - Header file for process.c
 * Copyright (c) 2016,2019-21 Red Hat Inc.
 * All Rights Reserved.
 *
 * This software may be freely redistributed and/or modified under the
 * terms of the GNU General Public License as published by the Free
 * Software Foundation; either version 2, or (at your option) any
 * later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program; see the file COPYING. If not, write to the
 * Free Software Foundation, Inc., 51 Franklin Street, Fifth Floor
 * Boston, MA 02110-1335, USA.
 *
 * Authors:
 *   Steve Grubb <sgrubb@redhat.com>
 */

#ifndef PROCESS_HEADER
#define PROCESS_HEADER

#include <sys/types.h>
#include <stdint.h>
#include "gcc-attributes.h"

// This is used to determine what kind of elf file we are looking at.
// HAS_LOAD but no HAS_DYNAMIC is staticly linked app. Normally you see both.
#define IS_ELF		0x00001
#define HAS_ERROR	0x00002
// #define HAS_RPATH	0x00004
#define HAS_DYNAMIC	0x00008
#define HAS_LOAD	0x00010
#define HAS_INTERP	0x00020
#define HAS_BAD_INTERP	0x00040
#define HAS_EXEC	0x00080
#define HAS_CORE	0x00100
#define HAS_REL		0x00200
#define HAS_DEBUG	0x00400
#define HAS_RWE_LOAD	0x00800
#define HAS_PHDR	0x01000
#define HAS_EXE_STACK	0x02000

#endif
