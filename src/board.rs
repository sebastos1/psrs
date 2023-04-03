use crate::piece::{Color, Piece, PieceKind};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Tile {
    Empty,
    Single(Piece),
    Union { white: Piece, black: Piece },
}

#[derive(Debug, Clone)]
pub struct Board {
    pub tiles: [[Tile; 8]; 8],
}

impl Board {
    pub fn new() -> Board {
        let mut board = [[Tile::Empty; 8]; 8];
        let mut iter = [
            PieceKind::Rook,
            PieceKind::Knight,
            PieceKind::Bishop,
            PieceKind::Queen,
            PieceKind::King,
            PieceKind::Bishop,
            PieceKind::Knight,
            PieceKind::Rook,
        ].iter();
        for i in 0..8 {
            let piece = *iter.next().unwrap();
            board[i][0] = Tile::Single(Piece::new(piece, Color::White));
            board[i][1] = Tile::Single(Piece::new(PieceKind::Pawn, Color::White));
            board[i][6] = Tile::Single(Piece::new(PieceKind::Pawn, Color::Black));
            board[i][7] = Tile::Single(Piece::new(piece, Color::Black));
        }
        Board { tiles: board }
    }

    // pub fn from_fen or something

    pub fn print(&self) {
        for row in (0..8).rev() {
            print!("{} ", row + 1);
            for col in 0..8 {
                match self.tiles[col][row] {
                    Tile::Empty => print!(" _  "),
                    Tile::Single(piece) => match piece.kind {
                        PieceKind::Pawn => print!(" P  "),
                        PieceKind::Knight => print!(" N  "),
                        PieceKind::Bishop => print!(" B  "),
                        PieceKind::Rook => print!(" R  "),
                        PieceKind::Queen => print!(" Q  "),
                        PieceKind::King => print!(" K  "),
                    },
                    Tile::Union { white, black } => {
                        match white.kind {
                            PieceKind::Pawn => print!("P"),
                            PieceKind::Knight => print!("N"),
                            PieceKind::Bishop => print!("B"),
                            PieceKind::Rook => print!("R"),
                            PieceKind::Queen => print!("Q"),
                            PieceKind::King => print!("K"),
                        }
                        print!("x");
                        match black.kind {
                            PieceKind::Pawn => print!("P "),
                            PieceKind::Knight => print!("N "),
                            PieceKind::Bishop => print!("B "),
                            PieceKind::Rook => print!("R "),
                            PieceKind::Queen => print!("Q "),
                            PieceKind::King => print!("K "),
                        }
                    }
                }
            }
            println!();
        }
        println!("   A   B   C   D   E   F   G   H");
    }

    pub fn get_piece(&self, pos: (usize, usize)) -> Option<Piece> {
        match self.tiles[pos.0][pos.1] {
            Tile::Empty => None,
            Tile::Single(piece) => Some(piece),
            Tile::Union { white, black } => {
                if white.color == Color::White {
                    Some(white)
                } else {
                    Some(black)
                }
            }
        }
    }


    pub fn try_update(&mut self, start: (usize, usize), moves: Vec<(usize, usize)>, playing: Color) -> bool {
        let end = moves.last().unwrap();
        match self.tiles[start.0][start.1] {
            Tile::Single(piece) => {
                match moves.len() {
                    1 => {
                        if piece.valid_moves(start, self)["empty"].contains(&end) || piece.valid_moves(start, self)["enemy"].contains(&end) {
                            self.update(start, *end, Tile::Single(piece));
                            return true
                        } else {
                            return false
                        }
                    },
                    _ => {
                        match self.tiles[end.0][end.1] {
                            Tile::Union { .. } => { 
                                return false
                            },
                            _ => { 
                                let mut theory_board = self.clone();
                                if theory_board.validate_sequence(start, piece, moves, playing) {
                                    *self = theory_board.clone();
                                    return true
                                } else {
                                    return false
                                }
                            },
                        }
                    },
                }
            },
            Tile::Union { white, black } => {
                let piece = if white.color == playing { white } else { black };
                println!("{:?}, {:?}", piece.valid_moves(start, self)["empty"], end);
                if piece.valid_moves(start, self)["empty"].contains(&end) {
                    match self.tiles[end.0][end.1] {
                        Tile::Union { .. } => { return false },
                        _ => { self.update(start, *end, Tile::Union{white,black}); return true }, // update here
                    }
                } else {
                    return false
                }
            }
            Tile::Empty => { return false },
        }
    }

    fn validate_sequence(&mut self, current: (usize, usize), moving_piece: Piece, moves: Vec<(usize, usize)>, playing: Color) -> bool {
        self.print();
        println!("step");
        println!("current: {:?}, moving_piece: {:?}, moves: {:?}, playing: {:?}", current, moving_piece, moves, playing);
        let next = moves[0];
        
        println!("{:.?}", self.tiles[current.0][current.1]);
        match self.tiles[current.0][current.1] {
            Tile::Single(..) => self.tiles[current.0][current.1] = Tile::Empty,
            Tile::Union { white, black } => {
                match playing {
                    Color::White => self.tiles[current.0][current.1] = Tile::Union { white: moving_piece, black },
                    Color::Black => self.tiles[current.0][current.1] = Tile::Union { white, black: moving_piece },
                }
            }
            Tile::Empty => return false,
        }
        
        if let Tile::Union { white, black } = self.tiles[next.0][next.1] {
            let new_piece = if playing == Color::White { white } else { black };
            self.tiles[next.0][next.1] = Tile::Union { white: moving_piece, black };
            let new_moves = moves[1..].to_vec();
            return self.validate_sequence(next, new_piece, new_moves, playing);
        } else if moving_piece.valid_moves(current, self)["empty"].contains(&next) || moving_piece.valid_moves(current, self)["enemy"].contains(&next) {
            match self.tiles[next.0][next.1] {
                Tile::Union { .. } => { return false },
                Tile::Single(opponent) => { 
                    match playing {
                        Color::White => self.tiles[next.0][next.1] = Tile::Union { white: moving_piece, black: opponent },
                        Color::Black => self.tiles[next.0][next.1] = Tile::Union { white: opponent, black: moving_piece },
                    }
                },
                Tile::Empty => { self.tiles[next.0][next.1] = Tile::Single(moving_piece) },
            }
            return true;
        } else {
            println!("{:.?}, {:.?}", moving_piece.valid_moves(current, self)["empty"], moving_piece.valid_moves(current, self)["enemy"]);
            return false;
        }
    }

    fn update(&mut self, start: (usize, usize), dest: (usize, usize), piece: Tile) {
        match piece {
            Tile::Single(piece) => {
                if let Tile::Single(occupant) = self.tiles[dest.0][dest.1] {
                    self.tiles[start.0][start.1] = Tile::Empty;
                    self.tiles[dest.0][dest.1] = if piece.color == Color::White {
                        Tile::Union { white: piece, black: occupant }
                    } else {
                        Tile::Union { white: occupant, black: piece }
                    }
                } else {
                    self.tiles[start.0][start.1] = Tile::Empty;
                    self.tiles[dest.0][dest.1] = Tile::Single(piece);
                }
            }
            Tile::Union { white, black } => {
                self.tiles[start.0][start.1] = Tile::Empty;
                self.tiles[dest.0][dest.1] = Tile::Union { white, black };
            }
            _ => {}
        }        
    }
}