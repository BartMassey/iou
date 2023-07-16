![Maintenance](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

# iou: Initialize-on-use struct for Rust
Bart Massey 2023

Type for lazily initializing a value on use. [Iou] is
similar to `std::borrow::Cow`: instead of taking
ownership when first written, [Iou] initializes into a
owned value when first referenced.

[Iou] is useful in cases where initialization is
expensive or time consuming, and the price is better
paid later.

An [Iou] will have a "corrupted cell" if its initialization
function panics during initialization. Operations on an [Iou]
with a corrupted cell will themselves panic.

# License

This work is licensed under the "MIT License". Please see the file
`LICENSE.txt` in this distribution for license terms.
