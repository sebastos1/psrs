use crate::board::{Board, Tile};
use std::collections::HashMap;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum PieceKind {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: Color,
    pub has_moved: bool,
}

impl Piece {
    pub fn new(kind: PieceKind, color: Color) -> Piece {
        Piece {
            kind: kind,
            color: color,
            has_moved: false,
        }
    }

    pub fn valid_moves<'a>(&'a self, pos: (usize, usize), board: &'a Board) -> HashMap<&str, Vec<(usize, usize)>> {
        let mut moves = HashMap::from([
            ("empty", vec![]),
            ("enemy", vec![]),
            ("union", vec![])
        ]);
 
        match self.kind {
            PieceKind::Pawn => { 
                let start_row: i8;
                let dir: i8;
                match self.color {
                    Color::White => {
                       start_row = 1;
                       dir = 1;
                    }
                    Color::Black => {
                        start_row = 6;
                        dir = -1;
                    }
                }
                if let Some(next) = add(pos.1, dir) {
                    if board.tiles[pos.0][next] == Tile::Empty {
                        moves.get_mut("empty").unwrap().push((pos.0, next));
                        if let Some(next) = add(pos.1, 2*dir) {
                            if board.tiles[pos.0][next] == Tile::Empty && start_row == pos.1 as i8 {
                                moves.get_mut("empty").unwrap().push((pos.0, next));
                            }
                        }
                    }
                }
                
                let offsets = if pos.0 == 0 {
                    vec![1]
                } else if pos.0 == 7 {
                    vec![-1]
                } else {
                    vec![-1, 1]
                };
                for offset in offsets {
                    if let Some(col) = add(pos.0, offset) {
                        if let Some(row) = add(pos.1, dir) {
                            match board.tiles[col][row] {
                                Tile::Single(piece) => {
                                    if piece.color != self.color {
                                        moves.get_mut("enemy").unwrap().push((col, row));
                                    }
                                }
                                Tile::Union{ .. } => {
                                    moves.get_mut("union").unwrap().push((col, row));
                                }
                                Tile::Empty => {}
                            }
                        }
                    }
                }
            },
            
            PieceKind::Knight => {
                let knight_jumps = [(2,1),(1,2),(1,-2),(2,-1),(-1,2),(-2,1),(-1,-2),(-2,-1)];
                for jump in knight_jumps {
                    if let Some(col) = add(pos.0, jump.0) {
                        if let Some(row) = add(pos.1, jump.1) {
                            match board.tiles[col][row] {
                                Tile::Empty => moves.get_mut("empty").unwrap().push((col, row)),
                                Tile::Single(piece) => {
                                    if piece.color != self.color {
                                        moves.get_mut("enemy").unwrap().push((col, row));
                                    }
                                }
                                Tile::Union{ .. } => {
                                    moves.get_mut("union").unwrap().push((col, row));
                                }
                            }
                        }
                    }
                }
            },

            PieceKind::Rook => {
                let dirs: Vec<(i8, i8)> = vec![(0, 1), (0, -1), (1, 0), (-1, 0)];
                moves = generic_piece(moves, self, pos, board, dirs);
            },
            
            PieceKind::Bishop => {
                let dirs: Vec<(i8, i8)> = vec![(1, 1), (1, -1), (-1, 1), (-1, -1)];
                moves = generic_piece(moves, self, pos, board, dirs);          
            },
            
            PieceKind::Queen => {
                let dirs: Vec<(i8, i8)> = vec![(1, 1), (1, -1), (-1, 1), (-1, -1), (0, 1), (0, -1), (1, 0), (-1, 0)];
                moves = generic_piece(moves, self, pos, board, dirs);         
            },
            
            PieceKind::King => {
                let dirs = [(1, 1), (1, -1), (-1, 1), (-1, -1), (0, 1), (0, -1), (1, 0), (-1, 0)];
                for dir in dirs {
                    if let Some(col) = add(pos.0, dir.0) {
                        if let Some(row) = add(pos.1, dir.1) {
                            match board.tiles[col][row] {
                                Tile::Empty => moves.get_mut("empty").unwrap().push((col, row)),
                                Tile::Single(piece) => {
                                    if piece.color != self.color {
                                        moves.get_mut("empty").unwrap().push((col, row));
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                }
            },
        }
        return moves;
    }
}

fn generic_piece<'a>(mut moves: HashMap<&'a str, Vec<(usize, usize)>>, piece: &'a Piece, pos: (usize, usize), board: &'a Board, dirs: Vec<(i8, i8)>) -> HashMap<&'a str, Vec<(usize, usize)>> {
    for (dirx, diry) in dirs {
        let mut col = pos.0;
        let mut row = pos.1;
        loop {
            if let Some(new_col) = add(col, dirx) {
                col = new_col;
            } else {
                break;
            }
            if let Some(new_row) = add(row, diry) {
                row = new_row;
            } else {
                break;
            }
            match board.tiles[col][row] {
                Tile::Empty => moves.get_mut("empty").unwrap().push((col, row)),
                Tile::Single(occupant) => {
                    if occupant.color != piece.color {
                        moves.get_mut("enemy").unwrap().push((col, row));
                    }
                    break;
                },
                Tile::Union{ .. } => {
                    moves.get_mut("union").unwrap().push((col, row));
                    break;
                },
            }
        }
    }
    return moves
}

fn add(x: usize, y: i8) -> Option<usize> {
    let result = (x as i8) + y;
    if result >= 0 && result <= 7 {
        Some(result as usize)
    } else {
        None
    }
}  