# ucl-sys

A low-level FFI bindings for UCL Library.

## Abstract

UCL is a portable lossless data compression library written in ANSI C.

UCL implements a number of compression algorithms that achieve an
excellent compression ratio while allowing *very* fast decompression.
Decompression requires no additional memory.

UCL is distributed under the terms of the GNU General Public License (GPL).

## Overview

UCL implements a number of algorithms with the following features:

- Decompression is simple and *very* fast.
- Requires no memory for decompression.
- The decompressors can be squeezed into less than 200 bytes of code.
- Includes compression levels for generating pre-compressed
  data which achieve an excellent compression ratio.
- Allows you to dial up extra compression at a speed cost in the
  compressor. The speed of the decompressor is not reduced.
- Algorithm is thread safe.
- Algorithm is lossless.

UCL supports in-place decompression.

## Design criteria

UCL's main design goal was a very high decompression speed while
achieving an excellent compression ratio. Real-time decompression should
be possible for virtually any application. The implementation of the
NRV2B decompressor in optimized i386 assembler code runs about at
the fifth of the speed of a memcpy() - and even faster for many files.

## Portability

UCL's decompressors should work on any system around - they could even
get ported to 8-bit processors such as the Z-80 or 6502.

UCL's compressors currently require at least 32-bit integers. While
porting them to more restricted environments (such as 16-bit DOS)
should be possible without too much effort this is not considered
important at this time.

## COPYRIGHT

The UCL library is Copyright (C) 1996, 1997, 1998, 1999, 2000, 2001, 2002,
2003 by Markus Franz Xaver Johannes Oberhumer <markus@oberhumer.com>.

The UCL library is distributed under the terms of the GNU General Public
License (GPL). See the file COPYING.

Special licenses for commercial and other applications which
are not willing to accept the GNU General Public License
are available by contacting the author.
