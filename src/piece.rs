#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color {
    White,
    Black,
    None,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type{
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
    Empty,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Piece {

    x: i32,
    y: i32,
    color: Color,
    peice_type: Type,

}

impl Piece{

    pub fn new(x: i32, y: i32, color: Color, peice_type: Type) -> Self {
        Self {
            x,
            y,
            color,
            peice_type,
        }
    }

    pub fn get_pos(&self) -> Vec<i32>{
        vec![self.x, self.y]
    }

    pub fn set_pos(&mut self, x: i32, y: i32){
        self.x = x;
        self.y = y;
    }

    pub fn get_color(&self) -> &Color{
        &self.color
    }

    pub fn get_type(&self) -> &Type{
        &self.peice_type
    }

}