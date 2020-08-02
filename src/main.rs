use computing::ComputeShader;


fn main() {
    let cs = ComputeShader::from_source(include_str!("shader.comp"));
    
    let mut x : [u32; 32] = [1; 32];
    cs.start()
        .bind_mut_buffer(&mut x)
        .compute(32,1,1);
    println!("{:#?}", x);

}
