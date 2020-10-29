# shadez
Easy to use compute shaders for Rust using wgpu.

# Example

It's really this easy:
```rust
let nums: Vec<u32> = (1..SIZE+1).collect(); 
let mut output: Vec<u32> = vec![0;SIZE as usize];

let shader = shadez::ComputeShader::from_source(include_str!("collatz.comp"));
shader.pass()
      .bind_buffer(&nums)
      .bind_mut_buffer(&mut output)
      .compute(nums.len(), 1, 1);
```

And here is the shader:
```glsl
#version 450

layout(set=0 , binding = 0) buffer Input {
    uint[] numbers;
};

layout(set=0 , binding = 1) buffer Output {
    uint[] collatz;
};

uint collatz_iterations(uint n) {
    uint i = 0;
    while(n != 1) {
        if (mod(n, 2) == 0) {
            n = n / 2;
        }
        else {
            n = (3 * n) + 1;
        }
        i++;
    }
    return i;
}


void main() {
    uint index = gl_GlobalInvocationID.x;
    collatz[index] = collatz_iterations(numbers[index]);
}
```
