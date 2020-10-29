
const SIZE : u32 = 1000000;


fn gpu() {

    let nums: Vec<u32> = (1..SIZE+1).collect(); 
    let mut output: Vec<u32> = vec![0;SIZE as usize];

    let shader = shadez::ComputeShader::from_source(include_str!("collatz.comp"));

    shader.pass()
        .bind_buffer(&nums)
        .bind_mut_buffer(&mut output)
        .compute(SIZE, 1, 1);

    println!("{:#?}", output);

}

fn main() {
    gpu();

}
