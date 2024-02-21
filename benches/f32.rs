use std::hint::black_box;

use criterion::Criterion;

#[inline(never)]
pub fn midpoint_upcast(x: f32, y: f32) -> f32 {
    ((f64::from(x) + f64::from(y)) / 2.0) as f32
}

#[inline(never)]
pub fn midpoint_std(a: f32, b: f32) -> f32 {
    const LO: f32 = f32::MIN_POSITIVE * 2.;
    const HI: f32 = f32::MAX / 2.;

    let abs_a = abs_private(a);
    let abs_b = abs_private(b);

    if abs_a <= HI && abs_b <= HI {
        // Overflow is impossible
        (a + b) / 2.
    } else if abs_a < LO {
        // Not safe to halve a
        a + (b / 2.)
    } else if abs_b < LO {
        // Not safe to halve b
        (a / 2.) + b
    } else {
        // Not safe to halve a and b
        (a / 2.) + (b / 2.)
    }
}

const fn abs_private(x: f32) -> f32 {
    use core::mem;

    // SAFETY: This transmutation is fine. Probably. For the reasons std is using it.
    unsafe { mem::transmute::<u32, f32>(mem::transmute::<f32, u32>(x) & 0x7fff_ffff) }
}

criterion::criterion_group!(cmain, cgroup);
criterion::criterion_main!(cmain);

fn cgroup(c: &mut Criterion) {
    let config = [
        ("special 1 (both <= HI)", 0.0, 100.0, 50.0),
        ("special 2 (a < LO, b > HI)", 0.0, f32::MAX, f32::MAX / 2.0),
        ("special 3 (b < LO, a > HI)", f32::MAX, 0.0, f32::MAX / 2.0),
        ("special 4 (a > HI, b > HI)", f32::MAX, f32::MAX, f32::MAX),
    ];

    for (name, a, b, res) in config {
        c.bench_function(&format!("f64::from {name}"), |bench| {
            bench.iter(|| {
                let a = black_box(a);
                let b = black_box(b);
                assert_eq!(midpoint_upcast(a, b), res);
            });
        });
        c.bench_function(&format!("std {name}"), |bench| {
            bench.iter(|| {
                let a = black_box(a);
                let b = black_box(b);
                assert_eq!(midpoint_std(a, b), res);
            });
        });
    }

    for (name, a, b, res) in config {
        c.bench_function(&format!("f64::from {name} no black box"), |bench| {
            bench.iter(|| {
                assert_eq!(midpoint_upcast(a, b), res);
            });
        });
        c.bench_function(&format!("std {name} no black box"), |bench| {
            bench.iter(|| {
                assert_eq!(midpoint_std(a, b), res);
            });
        });
    }

    let small_numbers: Vec<(f32, f32)> = (0..10_000_000)
        .map(|_| {
            let x = rand::random::<f32>();
            let y = rand::random::<f32>();
            (x * 100.0, y * 100.0)
        })
        .collect();

    c.bench_function("f64::from 10M small float pairs", |b| {
        b.iter(|| {
            for &(x, y) in small_numbers.iter() {
                black_box(midpoint_upcast(x, y));
            }
        })
    });

    c.bench_function("std 10M small float pairs", |b| {
        b.iter(|| {
            for &(x, y) in small_numbers.iter() {
                black_box(midpoint_std(x, y));
            }
        })
    });

    drop(small_numbers);

    let weird_numbers: Vec<(f32, f32)> = (0..10_000_000)
        .map(|_| {
            let x = rand::random::<f32>();
            let y = rand::random::<f32>();
            (x * f32::MAX, y * f32::MAX)
        })
        .collect();

    c.bench_function("f64::from 10M weird float pairs", |b| {
        b.iter(|| {
            for &(x, y) in weird_numbers.iter() {
                black_box(midpoint_upcast(x, y));
            }
        })
    });

    c.bench_function("std 10M weird float pairs", |b| {
        b.iter(|| {
            for &(x, y) in weird_numbers.iter() {
                black_box(midpoint_std(x, y));
            }
        })
    });
}
