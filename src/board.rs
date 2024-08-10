use crate::piece::{Piece, PieceType, Color, Square};

pub struct Board {
    pub squares: [[Square; 8]; 8],
}

impl Board {
    pub fn new() -> Self {
        let empty_square = Square { piece: None };
        let mut board = Board {
            squares: [[empty_square; 8]; 8],
        };

        // Set up the initial pieces on the board
        board.setup_pieces();
        board
    }

    pub fn setup_pieces(&mut self) {
        // Place pawns
        for i in 0..8 {
            self.squares[1][i].piece = Some(Piece {
                piece_type: PieceType::Pawn,
                color: Color::White,
            });
            self.squares[6][i].piece = Some(Piece {
                piece_type: PieceType::Pawn,
                color: Color::Black,
            });
        }

        // Place rooks
        self.squares[0][0].piece = Some(Piece {
            piece_type: PieceType::Rook,
            color: Color::White,
        });
        self.squares[0][7].piece = Some(Piece {
            piece_type: PieceType::Rook,
            color: Color::White,
        });
        self.squares[7][0].piece = Some(Piece {
            piece_type: PieceType::Rook,
            color: Color::Black,
        });
        self.squares[7][7].piece = Some(Piece {
            piece_type: PieceType::Rook,
            color: Color::Black,
        });

        // Place knights
        self.squares[0][1].piece = Some(Piece {
            piece_type: PieceType::Knight,
            color: Color::White,
        });
        self.squares[0][6].piece = Some(Piece {
            piece_type: PieceType::Knight,
            color: Color::White,
        });
        self.squares[7][1].piece = Some(Piece {
            piece_type: PieceType::Knight,
            color: Color::Black,
        });
        self.squares[7][6].piece = Some(Piece {
            piece_type: PieceType::Knight,
            color: Color::Black,
        });

        // Place bishops
        self.squares[0][2].piece = Some(Piece {
            piece_type: PieceType::Bishop,
            color: Color::White,
        });
        self.squares[0][5].piece = Some(Piece {
            piece_type: PieceType::Bishop,
            color: Color::White,
        });
        self.squares[7][2].piece = Some(Piece {
            piece_type: PieceType::Bishop,
            color: Color::Black,
        });
        self.squares[7][5].piece = Some(Piece {
            piece_type: PieceType::Bishop,
            color: Color::Black,
        });

        // Place queens
        self.squares[0][3].piece = Some(Piece {
            piece_type: PieceType::Queen,
            color: Color::White,
        });
        self.squares[7][3].piece = Some(Piece {
            piece_type: PieceType::Queen,
            color: Color::Black,
        });

        // Place kings
        self.squares[0][4].piece = Some(Piece {
            piece_type: PieceType::King,
            color: Color::White,
        });
        self.squares[7][4].piece = Some(Piece {
            piece_type: PieceType::King,
            color: Color::Black,
        });
    }

    pub fn display(&self) {
        for row in self.squares.iter() {
            for square in row.iter() {
                match square.piece {
                    Some(piece) => {
                        let symbol = match (piece.piece_type, piece.color) {
                            (PieceType::King, Color::White) => "♔",
                            (PieceType::Queen, Color::White) => "♕",
                            (PieceType::Rook, Color::White) => "♖",
                            (PieceType::Bishop, Color::White) => "♗",
                            (PieceType::Knight, Color::White) => "♘",
                            (PieceType::Pawn, Color::White) => "♙",
                            (PieceType::King, Color::Black) => "♚",
                            (PieceType::Queen, Color::Black) => "♛",
                            (PieceType::Rook, Color::Black) => "♜",
                            (PieceType::Bishop, Color::Black) => "♝",
                            (PieceType::Knight, Color::Black) => "♞",
                            (PieceType::Pawn, Color::Black) => "♟︎",
                            (PieceType::Assassin, _) => "A",
                        };
                        print!("{} ", symbol);
                    }
                    None => print!(". "),
                }
            }
            println!();
        }
    }
}
