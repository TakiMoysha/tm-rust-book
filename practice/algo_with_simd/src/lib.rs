#![feature(portable_simd)]

use std::simd::prelude::*;

type u64s = u64x8;
type u32s = u32x8;
type f64s = f64x8;

#[derive(Clone)]
struct Complex {
    real: f64s,
    imag: f64s,
}

fn get_count(start: &Complex) -> u32s {
    let mut current = start.clone();
    let mut count = u64s::splat(0);
    let threshold_mask = f64s::splat(THRESHOLD);

    for _ in 0..ITER_LIMIT {
        let rr = current.real * current.real;
        let ii = current.imag * current.imag;

        let udiverged_mask = (rr + ii).simd_le(threshold_mask);

        if !udiverged_mask.any() {
            break;
        };

        count += udiverged_mask.select(u64s::splat(1), u64s::splat(0));

        let ri = current.real * current.imag;

        current.real = start.real + (rr - ii);
        current.imag = start.imag + (ri + ri);
    }

    count.cast()
}
