
const SIZE : u32 = 1000000;

fn collatz(mut nr: u64) -> u64 {
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

fn cpu() {
    let nums: Vec<u64> = (1..(SIZE+1) as u64).collect();
    let mut output: Vec<u64> = vec![0;SIZE as usize];
    
    for (idx, nr) in nums.iter().enumerate() {
        output[idx] = collatz(*nr); 
    }


    println!("{:#?}", output);

}

fn gpu() {

    let nums: Vec<u32> = (1..SIZE+1).collect(); 
    let mut output: Vec<u32> = vec![0;SIZE as usize];

    let mut shader = computing::ComputeShader::from_source(include_str!("collatz.comp"));

    shader.bind_buffer(bytemuck::cast_slice(&nums))
        .bind_mut_buffer(bytemuck::cast_slice_mut(&mut output))
        .compute(SIZE, 1, 1);

    println!("{:#?}", output);

}

fn main() {
    gpu();

}
