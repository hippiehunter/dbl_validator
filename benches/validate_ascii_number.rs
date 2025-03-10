use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::Rng;
use std::ffi::c_int;

// Import the validate_ascii_number function from your library
// Assuming your library is named "my_lib"
use dbl_validator::other_validate_ascii_number as validate_ascii_number;

fn generate_test_case(length: usize, allow_negative: bool) -> Vec<u8> {
    let mut rng = rand::thread_rng();
    let mut number = Vec::with_capacity(length);

    if allow_negative && rng.gen_bool(0.5) {
        number.push(b'-');
    }

    for _ in 0..length - number.len() {
        number.push(rng.gen_range(b'0'..=b'9'));
    }

    number
}

fn bench_validate_ascii_number(c: &mut Criterion) {
    let mut group = c.benchmark_group("validate_ascii_number");

    for &length in &[4, 8, 16, 32] {
        for &allow_negative in &[false, true] {
            let test_case = generate_test_case(length, allow_negative);

            group.bench_function(format!("len={}, negative={}", length, allow_negative), |b| {
                b.iter(|| {
                    let result = unsafe {
                        validate_ascii_number(
                            black_box(test_case.as_ptr() as *const std::ffi::c_void),
                            black_box(test_case.len() as c_int),
                            black_box(allow_negative as c_int),
                        )
                    };
                    black_box(result);
                })
            });
        }
    }

    group.finish();
}

criterion_group!(benches, bench_validate_ascii_number);
criterion_main!(benches);