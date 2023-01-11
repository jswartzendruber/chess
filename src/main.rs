use std::fmt;
use std::io::{self, Write};

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
            Color::Black => {}
        };

        write!(f, "{piece}")
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
        writeln!(f, "  +------------+")?;
        for x in 0..8 {
            write!(f, "{} |  ", 8 - x)?;
            for y in 0..8 {
                let p = self.pieces[x * 8 + y];
                if p.is_some() {
                    write!(f, "{}", p.unwrap())?;
                } else {
                    write!(f, " ")?;
                }
            }
            writeln!(f, "  |")?;
        }
        writeln!(f, "  +------------+")?;
        writeln!(f, "     ABCDEFGH")?;
        Ok(())
    }
}

impl Board {
    fn starting_position() -> Self {
        let starting_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string();
        Self::load_fen(starting_fen)
    }

    fn move_piece(&mut self, from: usize, to: usize) {
        let temp = self.pieces[from];
        if let Some(piece) = temp {
            match piece.piece {
                PieceEnum::King => self.move_king(from, to),
                PieceEnum::Queen => self.move_queen(from, to),
                PieceEnum::Rook => self.move_rook(from, to),
                PieceEnum::Bishop => self.move_bishop(from, to),
                PieceEnum::Knight => self.move_knight(from, to),
                PieceEnum::Pawn => self.move_pawn(from, to),
            }
        }
    }

    fn move_king(&mut self, _from: usize, _to: usize) {}

    fn move_queen(&mut self, _from: usize, _to: usize) {}

    fn move_rook(&mut self, _from: usize, _to: usize) {}

    fn move_bishop(&mut self, _from: usize, _to: usize) {}

    fn move_knight(&mut self, _from: usize, _to: usize) {}

    fn move_pawn(&mut self, from: usize, to: usize) {
        self.pieces[to] = self.pieces[from];
        self.pieces[from] = None;
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

pub enum Error {
    SamePosition,
    InvalidEntry,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::SamePosition => write!(f, "The positions entered cannot be the same."),
            Error::InvalidEntry => write!(f, "The positions entered are invalid."),
        }
    }
}

pub fn parse_move(mov: &str) -> Result<(usize, usize), Error> {
    let upper = mov.to_ascii_uppercase();
    let mut chars = upper.chars();
    if let (Some(f1), Some(r1), Some(f2), Some(r2)) =
        (chars.next(), chars.next(), chars.next(), chars.next())
    {
        // Bounds checking to ensure input is A..H, 1..8
        if f1.is_ascii_alphabetic()
            && r1.is_ascii_digit()
            && f2.is_ascii_alphabetic()
            && r2.is_ascii_digit()
            && (1..=8).contains(&(f1 as usize - '@' as usize))
            && (1..=8).contains(&(f2 as usize - '@' as usize))
            && (1..=8).contains(&(r1 as usize - '0' as usize))
            && (1..=8).contains(&(r2 as usize - '0' as usize))
        {
            let row1 = f1 as usize - 'A' as usize;
            let col1 = 7 - (r1 as usize - '1' as usize);
            let row2 = f2 as usize - 'A' as usize;
            let col2 = 7 - (r2 as usize - '1' as usize);

            let idx1 = col1 * 8 + row1;
            let idx2 = col2 * 8 + row2;

            // Cannot move from same pos to same pos
            if f1 == f2 && r1 == r2 {
                return Err(Error::SamePosition);
            }

            return Ok((idx1, idx2));
        }
    }

    Err(Error::InvalidEntry)
}

fn main() {
    let mut board = Board::starting_position();
    println!("{board}");

    loop {
        print!("> ");
        io::stdout().flush().expect("Failed to flush output");

        let mut mov = String::new();
        io::stdin()
            .read_line(&mut mov)
            .expect("Failed to read line");

        let mov = mov.trim();

        if mov == "quit" {
            break;
        }

        let (from, to) = match parse_move(mov) {
            Ok(coords) => coords,
            Err(e) => {
                println!("Error: {e}");
                continue;
            }
        };

        board.move_piece(from, to);
        println!("{board}");
    }
}
