#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PieceType {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
    Assassin,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Color {
    White,
    Black,
}

#[derive(Clone, Copy, Debug)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: Color,
}

#[derive(Clone, Copy, Debug)]
pub struct Square {
    pub piece: Option<Piece>,
}
