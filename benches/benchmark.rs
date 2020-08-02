use criterion::{black_box, criterion_group, criterion_main, Criterion};
use computing::ComputeShader;

pub fn compile(c: &mut Criterion) {
    c.bench_function("shader from source", |b| b.iter(|| {
        let shader = ComputeShader::from_source(include_str!("collatz.comp"));
        black_box(shader)
    }));
}

pub fn collatz1(c: &mut Criterion) {

    let shader = ComputeShader::from_source(include_str!("collatz.comp")); 

    c.bench_function("collatz 10 000", |b| b.iter(|| {
        const SIZE: u32 = 10000;
        let nums: Vec<u32> = (1..SIZE+1).collect(); 
        let mut output: Vec<u32> = vec![0;SIZE as usize];
        


        shader.start()
            .bind_buffer(&nums)
            .bind_mut_buffer(&mut output)
            .compute(SIZE, 1, 1);

    }));
}


pub fn collatz2(c: &mut Criterion) {

    let shader = ComputeShader::from_source(include_str!("collatz.comp")); 

    c.bench_function("collatz 100 000", |b| b.iter(|| {
        const SIZE: u32 = 100000;
        let nums: Vec<u32> = (1..SIZE+1).collect(); 
        let mut output: Vec<u32> = vec![0;SIZE as usize];
        


        shader.start()
            .bind_buffer(&nums)
            .bind_mut_buffer(&mut output)
            .compute(SIZE as u32, 1, 1);

    }));
}

criterion_group!(benches, collatz1, collatz2);
criterion_main!(benches);

