use std::hint::black_box;

use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BenchmarkId;
use criterion::Criterion;
use matrix_match::matrix_match_double;
use matrix_match::matrix_match_single;

fn big_single(a: u32, b: u32) -> u32 {
    matrix_match_single!(
        (a, b) ;   0,  1,  2,  3,  4,  5,  6,  7,  8,  9, _ =>
        0     =>   0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 0  ;
        1     =>   0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 0  ;
        2     =>   0,  2,  4,  6,  8, 10, 12, 14, 16, 18, 0  ;
        3     =>   0,  3,  6,  9, 12, 15, 18, 21, 24, 27, 0  ;
        4     =>   0,  4,  8, 12, 16, 20, 24, 28, 32, 36, 0  ;
        5     =>   0,  5, 10, 15, 20, 25, 30, 35, 40, 45, 0  ;
        6     =>   0,  6, 12, 18, 24, 30, 36, 42, 48, 54, 0  ;
        7     =>   0,  7, 14, 21, 28, 35, 42, 49, 56, 63, 0  ;
        8     =>   0,  8, 16, 24, 32, 40, 48, 56, 64, 72, 0  ;
        _     =>   0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 0  ;
    )
}

fn big_double(a: u32, b: u32) -> u32 {
    matrix_match_double!(
        (a, b) ;   0,  1,  2,  3,  4,  5,  6,  7,  8,  9, _ =>
        0     =>   0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 0  ;
        1     =>   0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 0  ;
        2     =>   0,  2,  4,  6,  8, 10, 12, 14, 16, 18, 0  ;
        3     =>   0,  3,  6,  9, 12, 15, 18, 21, 24, 27, 0  ;
        4     =>   0,  4,  8, 12, 16, 20, 24, 28, 32, 36, 0  ;
        5     =>   0,  5, 10, 15, 20, 25, 30, 35, 40, 45, 0  ;
        6     =>   0,  6, 12, 18, 24, 30, 36, 42, 48, 54, 0  ;
        7     =>   0,  7, 14, 21, 28, 35, 42, 49, 56, 63, 0  ;
        8     =>   0,  8, 16, 24, 32, 40, 48, 56, 64, 72, 0  ;
        _     =>   0,  0,  0,  0,  0,  0,  0,  0,  0,  0, 0  ;
    )
}

#[allow(dead_code)]
pub enum Ea {
    A(bool),
    C,
}

#[allow(dead_code)]
pub enum Eb {
    A(bool, bool),
    B(u32),
}

fn complex_match_single(a: Ea, b: Eb) -> String {
    matrix_match_single!(
        (b, a)           ; Ea::A(true)        , Ea::A(false)       , Ea::C           =>
        Eb::A(true,  b) => "aa".to_string()   , b.to_string()      , (!b).to_string() ;
        Eb::A(false, b) => "afat".to_string() , "afaf".to_string() , (b as u8 + 4).to_string()  ;
        Eb::B(i)        => (i+4).to_string()  , (i*2).to_string()  , match i { 3 => "abcd".to_string(), _ => "cdef".to_string() }  ;
    )
}

fn complex_match_double(a: Ea, b: Eb) -> String {
    matrix_match_double!(
        (b, a)           ; Ea::A(true)        , Ea::A(false)       , Ea::C           =>
        Eb::A(true,  b) => "aa".to_string()   , b.to_string()      , (!b).to_string() ;
        Eb::A(false, b) => "afat".to_string() , "afaf".to_string() , (b as u8 + 4).to_string()  ;
        Eb::B(i)        => (i+4).to_string()  , (i*2).to_string()  , match i { 3 => "abcd".to_string(), _ => "cdef".to_string() }  ;
    )
}

pub fn bench_big(c: &mut Criterion) {
    let mut group = c.benchmark_group("big");

    group.bench_function("single", |b| {
        b.iter(|| big_single(black_box(5), black_box(3)))
    });

    group.bench_function("double", |b| {
        b.iter(|| big_double(black_box(5), black_box(3)))
    });

    group.finish();
}

pub fn bench_complex(c: &mut Criterion) {
    let mut group = c.benchmark_group("complex_match");

    group.bench_function("single", |b| {
        b.iter(|| complex_match_single(black_box(Ea::C), black_box(Eb::A(false, true))))
    });

    group.bench_function("double", |b| {
        b.iter(|| complex_match_double(black_box(Ea::C), black_box(Eb::A(false, true))))
    });

    group.finish();
}

criterion_group!(benches, bench_big, bench_complex);
criterion_main!(benches);
