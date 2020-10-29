use criterion::{black_box, criterion_group, criterion_main, Criterion};
use shadez::ComputeShader;

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
        let mut output: Vec<u32> = vec![0; SIZE as usize];
        


        shader.pass()
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
        


        shader.pass()
            .bind_buffer(&nums)
            .bind_mut_buffer(&mut output)
            .compute(SIZE as u32, 1, 1);

    }));
}


fn collatz(mut nr: u32) -> u32 {
    let mut i = 0;
    while nr != 1 {
        if nr%2 == 0 {
            nr = nr/2;
        } else {
            nr = nr*3 + 1;
        }
        i = i+1;
    }
    i
}


pub fn compare(c: &mut Criterion) {
    const SIZE: u32 = 100000; 
    let mut group = c.benchmark_group("CPU vs GPU");

    group.bench_function("cpu", |b| b.iter(|| { 
        let nums: Vec<u32> = (1..(SIZE+1) as u32).collect();
        let mut output: Vec<u32> = vec![0;SIZE as usize];

        for (idx, nr) in nums.iter().enumerate() {
            output[idx] = collatz(*nr); 
        }
        black_box(output);
    }));
    
    let shader = ComputeShader::from_source(include_str!("collatz.comp"));

    group.bench_function("gpu", |b| b.iter(|| { 
        let nums: Vec<u32> = (1..SIZE+1).collect(); 
        let mut output: Vec<u32> = vec![0; SIZE as usize];
        


        shader.pass()
            .bind_buffer(&nums)
            .bind_mut_buffer(&mut output)
            .compute(SIZE, 1, 1);

    }));
}

criterion_group!(benches, collatz1, collatz2, compare);
criterion_main!(benches);

