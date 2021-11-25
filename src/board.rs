use std::borrow::Borrow;
use crate::piece::{Piece, Type, Color};
use std::collections::HashMap;
use std::collections::hash_map::RandomState;


#[derive(Debug)]
pub(crate) struct Board {

    board: [[Piece; 8]; 8],
    turn: bool,
    white_king_moved: bool,
    black_king_moved: bool,
    white_queen_rook_moved: bool,
    black_queen_rook_moved: bool,
    white_king_rook_moved: bool,
    black_king_rook_moved: bool,
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
            white_king_moved: false,
            black_king_moved: false,
            white_queen_rook_moved: false,
            black_queen_rook_moved: false,
            white_king_rook_moved: false,
            black_king_rook_moved: false,
            is_game_over: false
        }

    }

    pub fn get_board(&self) -> &[[Piece; 8]; 8]{
        self.board.borrow()
    }

    pub fn get_turn(&self) -> bool{
        self.turn
    }


    pub fn get_white_king_moved(&self) -> bool{
        self.white_king_moved
    }

    pub fn get_black_king_moved(&self) -> bool{
        self.black_king_moved
    }

    pub fn get_white_queen_rook_moved(&self) -> bool{
        self.white_queen_rook_moved
    }

    pub fn get_black_queen_rook_moved(&self) -> bool{
        self.black_queen_rook_moved
    }

    pub fn get_white_king_rook_moved(&self) -> bool{
        self.white_king_rook_moved
    }

    pub fn get_black_king_rook_moved(&self) -> bool{
        self.black_king_rook_moved
    }

    pub fn set_white_king_moved(&mut self, moved: bool){
        self.white_king_moved = moved;
    }

    pub fn set_black_king_moved(&mut self, moved: bool){
        self.black_king_moved = moved;
    }

    pub fn set_white_queen_rook_moved(&mut self, moved: bool){
        self.white_queen_rook_moved = moved;
    }

    pub fn set_black_queen_rook_moved(&mut self, moved: bool){
        self.black_queen_rook_moved = moved;
    }

    pub fn set_white_king_rook_moved(&mut self, moved: bool){
        self.white_king_rook_moved = moved;
    }

    pub fn set_black_king_rook_moved(&mut self, moved: bool){
        self.black_king_rook_moved = moved;
    }

    pub fn set_turn(&mut self, turn: bool){
        self.turn = turn;
    }

    pub fn set_board(&mut self, board: [[Piece; 8]; 8]){
        self.board = board;
    }

    pub fn get_piece(&self, x: usize, y: usize) -> Piece{
        return self.board[y][x].clone();
    }

    pub fn set_piece(&mut self, x: usize, y: usize, piece: Piece) {
        self.board[y][x] = piece;
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
