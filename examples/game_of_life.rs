use computing::ComputeShader;


#[repr(u8)]
#[derive(Copy,Clone,Debug)]
enum Cell {
    Dead,
    Alive,
}


struct Game<'a> {
    rows: u32,
    cols: u32,

    old_board: Vec<Cell>,
    new_board: Vec<Cell>,

    shader: ComputeShader<'a>,
}

impl<'a> Game<'a> {

    fn new(rows: u32, cols: u32) -> Self {
        let shader = ComputeShader::from_source(include_str!("game_of_life.comp")); 
        Game {
            rows,
            cols,
            old_board: 
        }
    }




}


fn main() {
    
    
    

}
