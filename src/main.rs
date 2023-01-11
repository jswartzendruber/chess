use std::fmt;

#[derive(Debug, Clone, Copy)]
enum PieceEnum {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
}

#[derive(Debug, Clone, Copy)]
enum Color {
    Black,
    White,
}

#[derive(Debug, Clone, Copy)]
struct Piece {
    piece: PieceEnum,
    color: Color,
}

impl fmt::Display for Piece {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	let mut piece = match self.piece {
	    PieceEnum::King => "k",
	    PieceEnum::Queen => "q",
	    PieceEnum::Rook => "r",
	    PieceEnum::Bishop => "b",
	    PieceEnum::Knight => "n",
	    PieceEnum::Pawn => "p",
	};

	let upper = piece.to_ascii_uppercase();
	match self.color {
	    Color::White => piece = &upper,
	    Color::Black => {},
	};

        write!(f, "{}", piece)
    }
}

impl Piece {
    fn new(piece: PieceEnum, color: Color) -> Self {
        Self { piece, color }
    }
}

#[derive(Debug)]
struct Board {
    pieces: [Option<Piece>; 64],
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	for x in 0..8 {
	    for y in 0..8 {
		let p = self.pieces[x * 8 + y];
		if p.is_some() {
		    write!(f, "{}", p.unwrap())?;
		} else {
		    write!(f, " ")?;
		}
	    }
	    if x != 7 { // Skip extra println
		writeln!(f)?;
	    }
	}
	Ok(())
    }
}

impl Board {
    fn starting_position() -> Self {
        let starting_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string();
        Self::load_fen(starting_fen)
    }

    fn load_fen(fen: String) -> Self {
        let mut pieces = [None; 64];
        let mut chars = fen.chars().peekable();

        let mut index = 0;
        while chars.peek().is_some() {
            let next = chars.next().unwrap();

            match next {
                'r' => {
                    pieces[index] = Some(Piece::new(PieceEnum::Rook, Color::Black));
                    index += 1;
                }
                'n' => {
                    pieces[index] = Some(Piece::new(PieceEnum::Knight, Color::Black));
                    index += 1;
                }
                'b' => {
                    pieces[index] = Some(Piece::new(PieceEnum::Bishop, Color::Black));
                    index += 1;
                }
                'q' => {
                    pieces[index] = Some(Piece::new(PieceEnum::Queen, Color::Black));
                    index += 1;
                }
                'k' => {
                    pieces[index] = Some(Piece::new(PieceEnum::King, Color::Black));
                    index += 1;
                }
                'p' => {
                    pieces[index] = Some(Piece::new(PieceEnum::Pawn, Color::Black));
                    index += 1;
                }
                'R' => {
                    pieces[index] = Some(Piece::new(PieceEnum::Rook, Color::White));
                    index += 1;
                }
                'N' => {
                    pieces[index] = Some(Piece::new(PieceEnum::Knight, Color::White));
                    index += 1;
                }
                'B' => {
                    pieces[index] = Some(Piece::new(PieceEnum::Bishop, Color::White));
                    index += 1;
                }
                'Q' => {
                    pieces[index] = Some(Piece::new(PieceEnum::Queen, Color::White));
                    index += 1;
                }
                'K' => {
                    pieces[index] = Some(Piece::new(PieceEnum::King, Color::White));
                    index += 1;
                }
                'P' => {
                    pieces[index] = Some(Piece::new(PieceEnum::Pawn, Color::White));
                    index += 1;
                }
                '0'..='8' => {
                    let i: usize = next as usize - '0' as usize;
                    index += i;
                }
                '/' => {}
                _ => panic!("Unsupported fen string"),
            }
        }

        Board { pieces }
    }
}

fn main() {
    let board = Board::starting_position();
    println!("{}", board);
}
