# Bit Array

An efficient library for working with many bit-values.

## Motivation

Although there is a ton of built-in support for manipulating bytes (`u8`'s), working with the bits that compose them is not as easy.
Most of the time byte-alignment will be the preferred default, however there are cases where this is not true.

For example, two I have come across recently are:

1. Calculating hashes, which require bit-level precision
1. Manipulating network packets, which require a very specific format

