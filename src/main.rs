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

struct Error {}

// Horizontal
#[derive(Debug, PartialEq)]
enum Rank {
    Rank(usize),
}

impl Rank {
    fn new(x: usize) -> Self {
        assert!((1..=8).contains(&x));
        Self::Rank(x)
    }

    fn from(c: char) -> Result<Self, Error> {
        match c {
            '1'..='8' => Ok(Self::new(c as usize - '0' as usize)),
            _ => Err(Error {}),
        }
    }
}

#[derive(Debug)]
struct Coordinate {
    file: File,
    rank: Rank,
}

impl Coordinate {
    fn new(file: File, rank: Rank) -> Self {
        Self { file, rank }
    }
}

// Vertical
#[derive(Debug, PartialEq)]
enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl File {
    fn from(c: char) -> Result<File, Error> {
        match c.to_ascii_uppercase() {
            'A' => Ok(File::A),
            'B' => Ok(File::B),
            'C' => Ok(File::C),
            'D' => Ok(File::D),
            'E' => Ok(File::E),
            'F' => Ok(File::F),
            'G' => Ok(File::G),
            'H' => Ok(File::H),
            _ => Err(Error {}),
        }
    }
}

fn parse_move(mov: &str) -> Result<(Coordinate, Coordinate), Error> {
    let mut chars = mov.chars();
    if let (Some(f1), Some(r1), Some(f2), Some(r2)) =
        (chars.next(), chars.next(), chars.next(), chars.next())
    {
        let f1 = File::from(f1)?;
        let r1 = Rank::from(r1)?;
        let f2 = File::from(f2)?;
        let r2 = Rank::from(r2)?;

        // Cannot move from same pos to same pos
        if f1 == f2 && r1 == r2 {
            return Err(Error {});
        }

        return Ok((Coordinate::new(f1, r1), Coordinate::new(f2, r2)));
    }

    Err(Error {})
}

fn main() {
    let board = Board::starting_position();
    println!("{}", board);

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
            Err(e) => continue,
        };

        println!("from {:?}, to {:?}", from, to);
    }
}
