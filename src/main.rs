use computing::ComputeShader;


fn main() {
    let mut cs =ComputeShader::from_source(include_str!("shader.comp"));
    
    let mut x : [u32; 32] = [1; 32];
    cs.bind_mut_buffer(bytemuck::bytes_of_mut(&mut x));
    cs.compute(32,1,1);
    println!("{:#?}", x);

}
