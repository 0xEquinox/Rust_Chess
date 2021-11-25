use crate::board::Board;
use crate::piece::{Piece, Color, Type};

mod board;
mod piece;

fn main() {

    let mut board:board::Board = board::Board::new();

    while !board.is_game_over() {

        board.print_board();

        let player_move:Vec<i32> = if board.turn() == &true{
            player_turn(&board, Color::White)
        }else{
            player_turn(&board, Color::Black)
        };

        let original_piece:Piece = *board.piece_at(player_move[0] as usize, player_move[1] as usize);
        let empty_piece:Piece = Piece::new(player_move[0], player_move[1], Color::None, Type::Empty);

        *board.piece_at_mut(player_move[2] as usize, player_move[3] as usize) = original_piece;
        *board.piece_at_mut(player_move[0] as usize, player_move[1] as usize) = empty_piece;

        let current_turn:bool = *board.turn();
        *board.turn_mut() = !current_turn;
    }

}

fn player_turn(board: &Board, color: Color) -> Vec<i32>{

    println!("Player {:?} turn", color);

    let mut input = String::new();

    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    if input.trim() == "exit" {
        std::process::exit(0);
    }

    let input: Vec<&str> = input.trim().split(" ").collect();

    let mut origin_x:i32 = 0;
    let mut origin_y:i32 = 0;
    let mut dest_x:i32= 0;
    let mut dest_y:i32 = 0;

    if input.len() == 4 {
        origin_x = input[0].parse::<i32>().unwrap() - 1;
        origin_y = input[1].parse::<i32>().unwrap() - 1;
        dest_x = input[2].parse::<i32>().unwrap() - 1;
        dest_y = input[3].parse::<i32>().unwrap() - 1;
    }else {
        println!("Invalid input");
        return player_turn(board, color);
    }

    let original_piece = board.piece_at(origin_x as usize, origin_y as usize);
    let dest_piece = board.piece_at(dest_x as usize, dest_y as usize);

    return if is_valid_move(&board, original_piece, dest_piece, &color, dest_x, dest_y){
        vec![origin_x, origin_y, dest_x, dest_y]
    }else{
        println!("Invalid move");
        player_turn(board, color)
    };

}

fn is_valid_move(board: &Board, move_piece: &Piece, dest_piece: &Piece, color: &Color, x: i32, y: i32) -> bool {

    return if move_piece.get_type() == &Type::Empty && move_piece.get_color() == color{
        false
    }else{
        true
    };

}
