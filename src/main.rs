use crate::board::Board;
use crate::piece::{Piece, Color, Type};
use crate::piece::Type::Empty;

mod board;
mod piece;

fn main() {

    let mut board:board::Board = board::Board::new();

    while !board.is_game_over() {

        board.print_board();

        let mut player_move:Vec<i32> = Vec::new();

        if board.get_turn() == true {
            player_move = player_turn(&board, Color::White);
        }else{
            player_move = player_turn(&board, Color::Black);
        }

        board.set_piece(player_move[2] as usize, player_move[3] as usize, board.get_piece(player_move[0] as usize, player_move[1] as usize));
        board.set_piece(player_move[0] as usize, player_move[1] as usize, piece::Piece::new(player_move[0], player_move[1], Color::None, Type::Empty));

        board.set_turn(!board.get_turn());
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

    //println!("{} {} {} {}", origin_x, origin_y, dest_x, dest_y);

    let piece = board.get_piece(origin_x as usize, origin_y as usize);

    if is_valid_move(&board, &piece, &board.get_piece(dest_x as usize, dest_y as usize), color, dest_x, dest_y) {
        return vec![origin_x, origin_y, dest_x, dest_y];
    }else{
        println!("Invalid move");
        return player_turn(board, color);
    }

}

fn is_valid_move(board: &Board, move_piece: &Piece, dest_piece: &Piece, color: Color, x: i32, y: i32) -> bool {

    if *move_piece.get_type() == Empty {
        println!("No piece at this location");
        return false;
    }

    if *move_piece.get_color() != color {
        println!("This is not your piece");
        return false;
    }

    true
}
