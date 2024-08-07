use ark_bls12_381::{G1Affine, G2Affine};
use ark_ff::BigInteger256;
use criterion::{criterion_group, criterion_main, Criterion};

use std::str::FromStr;

use cuda_msm::*;

fn criterion_benchmark(c: &mut Criterion) {
    let bench_npow = std::env::var("BENCH_NPOW").unwrap_or("23".to_string());
    let npoints_npow = i32::from_str(&bench_npow).unwrap();

    let (points, scalars) = util::generate_points_scalars::<G1Affine>(1usize << npoints_npow);

    let mut group = c.benchmark_group("CUDA");
    group.sample_size(20);

    let name = format!("2**{}", npoints_npow);
    group.bench_function(name, |b| {
        b.iter(|| {
            let _ = multi_scalar_mult_arkworks(&points.as_slice(), unsafe {
                std::mem::transmute::<&[_], &[BigInteger256]>(scalars.as_slice())
            });
        })
    });

    group.finish();
}

fn criterion_benchmark_fp2(c: &mut Criterion) {
    let bench_npow = std::env::var("BENCH_NPOW").unwrap_or("23".to_string());
    let npoints_npow = i32::from_str(&bench_npow).unwrap();

    let (points, scalars) = util::generate_points_scalars::<G2Affine>(1usize << npoints_npow);

    let mut group = c.benchmark_group("CUDA");
    group.sample_size(10);

    let name = format!("2**{}", npoints_npow);
    group.bench_function(name, |b| {
        b.iter(|| {
            let _ = multi_scalar_mult_fp2_arkworks(&points.as_slice(), unsafe {
                std::mem::transmute::<&[_], &[BigInteger256]>(scalars.as_slice())
            });
        })
    });

    group.finish();
}

criterion_group!(benches, criterion_benchmark, criterion_benchmark_fp2);

criterion_main!(benches);
