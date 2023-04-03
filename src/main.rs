use pacosakors::*;
use regex::Regex;
use std::io;

fn ask_input(board: &mut Board, mut playing: Color) -> Option<(usize, usize)> {
    loop {
        let mut input = String::new();
        println!("ENTER MOVES:");
        if let Err(err) = io::stdin().read_line(&mut input) {
            println!("invalid input somehow: {}", err);
            continue;
        } else {
            let validation = Regex::new(r"^[a-hA-H][1-8]([a-hA-H][1-8])*$").unwrap();
            let input = input.trim();
            if validation.is_match(input) {
                if input.len() == 2 {
                    let pos: (usize, usize) = get_coords(input.chars().nth(0).unwrap().to_ascii_lowercase(), input.chars().nth(1).unwrap());
                    if let Some(piece) = board.get_piece(pos) {
                        for (k, v) in piece.valid_moves(pos, board) {
                            if v.len() != 0 {
                                println!("{}: {:?}", k, get_notation(v));
                            }
                        }
                    } else {
                        println!("no piece at that position");
                    }
                } else {
                    let moves = input.chars().collect::<Vec<char>>().chunks(2).map(|pair| get_coords(pair[0].to_ascii_lowercase(), pair[1])).collect::<Vec<(usize,usize)>>();
                    if board.try_update(moves[0], moves[1..].to_vec(), playing) {
                        board.print();
                        playing = match playing {
                            Color::White => Color::Black,
                            Color::Black => Color::White,
                        };
                    } else {
                        println!("invalid move!")
                    }
                }
            } else {
                println!("Invalid input");
            }
        }
    }
}

fn get_notation(pos: Vec<(usize, usize)>) -> Vec<String> {
    let mut moves: Vec<String> = vec![];
    for (x,y) in pos {
        let letter = (x as u8 + b'a') as char;
        let number = (y as u8 + b'1') as char;
        moves.push(format!("{}{}", letter, number));
    }
    return moves
}

fn get_coords(letter: char, number: char) -> (usize, usize) {
    let col = (letter as u8 - b'a') as usize;
    let row = (number as u8 - b'1') as usize;
    return (col, row)
}

fn main() {
    let mut board = Board::new();
    board.print();

    let playing = Color::White;
    ask_input(&mut board, playing);
}
