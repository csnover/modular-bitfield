//! In this benchmark we compare our `modular_bitfield` crate generated bitfields
//! with the ones generated by the popular `bitfield` crate.
//!
//! We want to find out which crate produces the more efficient code for different
//! use cases and scenarios.

mod handwritten;
mod utils;

use bitfield::bitfield as bitfield_crate;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use modular_bitfield::prelude::*;
use utils::repeat;

criterion_group!(
    bench_get,
    bench_get_a,
    bench_get_b,
    bench_get_c,
    bench_get_d,
    bench_get_e
);
criterion_group!(
    bench_set,
    bench_set_a,
    bench_set_b,
    bench_set_c,
    bench_set_d,
    bench_set_e
);
criterion_main!(bench_get, bench_set);

bitfield_crate! {
    pub struct OtherBitfield(u32);

    #[inline]
    pub a, set_a: 8, 0;
    #[inline]
    pub b, set_b: 14, 9;
    #[inline]
    pub c, set_c: 27, 15;
    #[inline]
    pub d, set_d: 28, 28;
    #[inline]
    pub e, set_e: 31, 29;
}

#[bitfield]
pub struct ModularBitfield {
    pub a: B9,
    pub b: B6,
    pub c: B13,
    pub d: B1,
    pub e: B3,
}

macro_rules! generate_cmp_benchmark_for {
    (
        test($test_name_get:ident, $test_name_set:ident) {
            fn $fn_get:ident($name_get:literal);
            fn $fn_set:ident($name_set:literal);
        }
    ) => {
        fn $test_name_get(c: &mut Criterion) {
            let mut g = c.benchmark_group($name_get);
            g.bench_function("other_bitfield", |b| {
                let input = black_box(OtherBitfield(0x00));
                assert_eq!(input.$fn_get(), 0);
                b.iter(|| {
                    repeat(|| {
                        black_box(input.$fn_get());
                    })
                });
            });
            g.bench_function("modular_bitfield", |b| {
                let input = ModularBitfield::new();
                assert_eq!(input.$fn_get(), 0);
                b.iter(|| {
                    repeat(|| {
                        black_box(input.$fn_get());
                    })
                });
            });
        }

        fn $test_name_set(c: &mut Criterion) {
            let mut g = c.benchmark_group($name_set);
            g.bench_function("other_bitfield", |b| {
                let mut input = OtherBitfield(0x00);
                b.iter(|| {
                    repeat(|| {
                        black_box(black_box(&mut input).$fn_set(1));
                    })
                });
                assert_eq!(input.$fn_get(), 1);
            });
            g.bench_function("modular_bitfield", |b| {
                let mut input = ModularBitfield::new();
                b.iter(|| {
                    repeat(|| {
                        black_box(black_box(&mut input).$fn_set(1));
                    })
                });
                assert_eq!(input.$fn_get(), 1);
            });
        }
    };
}
generate_cmp_benchmark_for!(
    test(bench_get_a, bench_set_a) {
        fn a("compare_crates/get_a");
        fn set_a("compare_crates/set_a");
    }
);
generate_cmp_benchmark_for!(
    test(bench_get_b, bench_set_b) {
        fn b("compare_crates/get_b");
        fn set_b("compare_crates/set_b");
    }
);
generate_cmp_benchmark_for!(
    test(bench_get_c, bench_set_c) {
        fn c("compare_crates/get_c");
        fn set_c("compare_crates/set_c");
    }
);
generate_cmp_benchmark_for!(
    test(bench_get_d, bench_set_d) {
        fn d("compare_crates/get_d");
        fn set_d("compare_crates/set_d");
    }
);
generate_cmp_benchmark_for!(
    test(bench_get_e, bench_set_e) {
        fn e("compare_crates/get_e");
        fn set_e("compare_crates/set_e");
    }
);
