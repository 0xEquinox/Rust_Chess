use std::borrow::Borrow;
use crate::piece::{Piece, Type, Color};


#[derive(Debug, Copy, Clone)]
pub(crate) struct Board {

    board: [[Piece; 8]; 8],
    turn: bool,
    can_castle_kingside: bool,
    can_castle_queenside: bool,
    is_game_over: bool,
}

impl Board{

    pub fn new() -> Self{

        Self{

            board: [ [Piece::new(0, 0, Color::White, Type::Rook), Piece::new(1, 0, Color::White, Type::Knight), Piece::new(2, 0, Color::White, Type::Bishop), Piece::new(4, 0, Color::White, Type::Queen), Piece::new(5, 0, Color::White, Type::King), Piece::new(6, 0, Color::White, Type::Bishop), Piece::new(7, 0, Color::White, Type::Knight), Piece::new(8, 0, Color::White, Type::Rook)],
                          [Piece::new(0, 1, Color::White, Type::Pawn), Piece::new(1, 1, Color::White, Type::Pawn), Piece::new(2, 1, Color::White, Type::Pawn), Piece::new(3, 1, Color::White, Type::Pawn), Piece::new(4, 1, Color::White, Type::Pawn), Piece::new(5, 1, Color::White, Type::Pawn), Piece::new(6, 1, Color::White, Type::Pawn), Piece::new(7, 1, Color::White, Type::Pawn)],
                          [Piece::new(0, 2, Color::None, Type::Empty), Piece::new(1, 2, Color::None, Type::Empty), Piece::new(2, 2, Color::None, Type::Empty), Piece::new(3, 2, Color::None, Type::Empty), Piece::new(4, 2, Color::None, Type::Empty), Piece::new(5, 2, Color::None, Type::Empty), Piece::new(6, 2, Color::None, Type::Empty), Piece::new(7, 2, Color::None, Type::Empty)],
                          [Piece::new(0, 3, Color::None, Type::Empty), Piece::new(1, 3, Color::None, Type::Empty), Piece::new(2, 3, Color::None, Type::Empty), Piece::new(3, 3, Color::None, Type::Empty), Piece::new(4, 3, Color::None, Type::Empty), Piece::new(5, 3, Color::None, Type::Empty), Piece::new(6, 3, Color::None, Type::Empty), Piece::new(7, 3, Color::None, Type::Empty)],
                          [Piece::new(0, 4, Color::None, Type::Empty), Piece::new(1, 4, Color::None, Type::Empty), Piece::new(2, 4, Color::None, Type::Empty), Piece::new(3, 4, Color::None, Type::Empty), Piece::new(4, 4, Color::None, Type::Empty), Piece::new(5, 4, Color::None, Type::Empty), Piece::new(6, 4, Color::None, Type::Empty), Piece::new(7, 4, Color::None, Type::Empty)],
                          [Piece::new(0, 5, Color::None, Type::Empty), Piece::new(1, 5, Color::None, Type::Empty), Piece::new(2, 5, Color::None, Type::Empty), Piece::new(3, 5, Color::None, Type::Empty), Piece::new(4, 5, Color::None, Type::Empty), Piece::new(5, 5, Color::None, Type::Empty), Piece::new(6, 5, Color::None, Type::Empty), Piece::new(7, 5, Color::None, Type::Empty)],
                          [Piece::new(0, 6, Color::Black, Type::Pawn), Piece::new(1, 6, Color::Black, Type::Pawn), Piece::new(2, 6, Color::Black, Type::Pawn), Piece::new(3, 6, Color::Black, Type::Pawn), Piece::new(4, 6, Color::Black, Type::Pawn), Piece::new(5, 6, Color::Black, Type::Pawn), Piece::new(6, 6, Color::Black, Type::Pawn), Piece::new(7, 6, Color::Black, Type::Pawn)],
                          [Piece::new(0, 7, Color::Black, Type::Rook), Piece::new(1, 7, Color::Black, Type::Knight), Piece::new(2, 7, Color::Black, Type::Bishop), Piece::new(4, 7, Color::Black, Type::Queen), Piece::new(5, 7, Color::Black, Type::King), Piece::new(6, 7, Color::Black, Type::Bishop), Piece::new(7, 7, Color::Black, Type::Knight), Piece::new(8, 7, Color::Black, Type::Rook)]
            ],
            turn: true,
            can_castle_kingside: false,
            can_castle_queenside: false,
            is_game_over: false
        }

    }


    //check the turn and print the board on the side that the turn is on
    pub fn print_board(&mut self){

        if !self.turn {
            for i in 0..8 {
                print!("{} ", i + 1);
                for j in 0..8 {
                    if *self.board[i][j].get_type() == Type::Rook {
                        print!("| R |");
                    } else if *self.board[i][j].get_type() == Type::Knight {
                        print!("| N |");
                    } else if *self.board[i][j].get_type() == Type::Bishop {
                        print!("| B |");
                    } else if *self.board[i][j].get_type() == Type::Queen {
                        print!("| Q |");
                    } else if *self.board[i][j].get_type() == Type::King {
                        print!("| K |");
                    } else if *self.board[i][j].get_type() == Type::Pawn {
                        print!("| P |");
                    } else {
                        print!("|   |");
                    }
                }
                println!();
            }
        }else{
            for i in (0..8).rev() {
                print!("{}", i + 1);
                for j in 0..8 {
                    if *self.board[i][j].get_type() == Type::Rook {
                        print!("| R |");
                    } else if *self.board[i][j].get_type() == Type::Knight {
                        print!("| N |");
                    } else if *self.board[i][j].get_type() == Type::Bishop {
                        print!("| B |");
                    } else if *self.board[i][j].get_type() == Type::Queen {
                        print!("| Q |");
                    } else if *self.board[i][j].get_type() == Type::King {
                        print!("| K |");
                    } else if *self.board[i][j].get_type() == Type::Pawn {
                        print!("| P |");
                    } else {
                        print!("|   |");
                    }
                }
                println!();
            }
        }
    }

    pub fn is_game_over(&self) -> bool{
        return self.is_game_over;
    }

}


//Getters and Setters
impl Board {

    fn can_castle_kingside(&self) -> &bool{ &self.can_castle_kingside }

    fn can_castle_queenside(&self) -> &bool{ &self.can_castle_queenside }

    fn can_castle_kingside_mut(&mut self) -> &mut bool{ &mut self.can_castle_kingside }

    fn can_castle_queenside_mut(&mut self) -> &mut bool{ &mut self.can_castle_queenside }

    pub fn board(&self) -> &[[Piece; 8]; 8]{ &self.board }

    pub fn turn(&self) -> &bool{ &self.turn }

    pub fn turn_mut(&mut self) -> &mut bool{ &mut self.turn }

    pub fn board_mut(&mut self) -> &mut [[Piece; 8]; 8]{ &mut self.board }

    pub fn piece_at(&self, x: usize, y: usize) -> &Piece{ &self.board[y][x] }

    pub fn piece_at_mut(&mut self, x: usize, y: usize) -> &mut Piece{ &mut self.board[y][x] }

}
