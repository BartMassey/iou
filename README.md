# iou: Initialize-on-use struct for Rust
Bart Massey 2023

This branch contains a version of `Iou` that allows direct
refs to the initialized value. The code uses
`UnsafeCell`. Miri doesn't like it, so beware.

Please see the main branch of this repo for information
about `Iou` and its license.
