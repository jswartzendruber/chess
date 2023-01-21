use std::fmt;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy)]
enum PieceEnum {
    BlackKing = 0,
    BlackQueen,
    BlackRook,
    BlackBishop,
    BlackKnight,
    BlackPawn,
    WhiteKing,
    WhiteQueen,
    WhiteRook,
    WhiteBishop,
    WhiteKnight,
    WhitePawn,
}

impl fmt::Display for PieceEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut piece = match self {
            PieceEnum::BlackKing => "k",
            PieceEnum::BlackQueen => "q",
            PieceEnum::BlackRook => "r",
            PieceEnum::BlackBishop => "b",
            PieceEnum::BlackKnight => "n",
            PieceEnum::BlackPawn => "p",
            PieceEnum::WhiteKing => "K",
            PieceEnum::WhiteQueen => "Q",
            PieceEnum::WhiteRook => "R",
            PieceEnum::WhiteBishop => "B",
            PieceEnum::WhiteKnight => "N",
            PieceEnum::WhitePawn => "P",
        };

        write!(f, "{piece}")
    }
}

impl From<usize> for PieceEnum {
    fn from(item: usize) -> PieceEnum {
        match item {
            0 => PieceEnum::BlackKing,
            1 => PieceEnum::BlackQueen,
            2 => PieceEnum::BlackRook,
            3 => PieceEnum::BlackBishop,
            4 => PieceEnum::BlackKnight,
            5 => PieceEnum::BlackPawn,
            6 => PieceEnum::WhiteKing,
            7 => PieceEnum::WhiteQueen,
            8 => PieceEnum::WhiteRook,
            9 => PieceEnum::WhiteBishop,
            10 => PieceEnum::WhiteKnight,
            11 => PieceEnum::WhitePawn,
            _ => unreachable!(),
        }
    }
}

type BitBoard = u64;

struct Board {
    individual_pieces: [BitBoard; 12],

    black_pieces: BitBoard,
    white_pieces: BitBoard,

    all_pieces: BitBoard,
}

impl Board {
    fn starting_position() -> Self {
        let starting_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR".to_string();
        Self::load_fen(starting_fen)
    }

    fn move_piece(&mut self, from: usize, to: usize) {
	println!("move {} to {}", from, to);
    }

    fn load_fen(fen: String) -> Self {
        let mut individual_pieces = [0; 12];
        let mut chars = fen.chars().peekable();

        let mut index = 0;
        while chars.peek().is_some() {
            let next = chars.next().unwrap();
            let mut index_increment = 1;

            match next {
                'k' => individual_pieces[PieceEnum::BlackKing as usize] |= 1 << index,
                'q' => individual_pieces[PieceEnum::BlackQueen as usize] |= 1 << index,
                'r' => individual_pieces[PieceEnum::BlackRook as usize] |= 1 << index,
                'b' => individual_pieces[PieceEnum::BlackBishop as usize] |= 1 << index,
                'n' => individual_pieces[PieceEnum::BlackKnight as usize] |= 1 << index,
                'p' => individual_pieces[PieceEnum::BlackPawn as usize] |= 1 << index,
                'K' => individual_pieces[PieceEnum::WhiteKing as usize] |= 1 << index,
                'Q' => individual_pieces[PieceEnum::WhiteQueen as usize] |= 1 << index,
                'R' => individual_pieces[PieceEnum::WhiteRook as usize] |= 1 << index,
                'B' => individual_pieces[PieceEnum::WhiteBishop as usize] |= 1 << index,
                'N' => individual_pieces[PieceEnum::WhiteKnight as usize] |= 1 << index,
                'P' => individual_pieces[PieceEnum::WhitePawn as usize] |= 1 << index,
                '/' => index_increment = 0,
                '0'..='8' => index_increment = next as usize - '0' as usize,
                _ => panic!("Unsupported fen string"),
            }

            index += index_increment;
        }

        let mut black_pieces = 0;
        for piece in individual_pieces.iter().take(6) {
            black_pieces |= piece;
        }
        let mut white_pieces = 0;
        for piece in individual_pieces.iter().skip(6) {
            white_pieces |= piece;
        }

        let all_pieces = black_pieces | white_pieces;

        Board {
            individual_pieces,
            black_pieces,
            white_pieces,
            all_pieces,
        }
    }
}

struct PrintableBoard {
    pieces: [Option<PieceEnum>; 64],
}

impl PrintableBoard {
    fn new(board: &Board) -> Self {
        let mut pieces = [None; 64];
        for piece_idx in (PieceEnum::BlackKing as usize)..=(PieceEnum::WhitePawn as usize) {
            let piece = board.individual_pieces[piece_idx];
            for (bit, printable_piece) in pieces.iter_mut().enumerate() {
                let piece_exists = piece >> bit & 1 == 1;
                if piece_exists {
                    *printable_piece = Some(piece_idx.into())
                }
            }
        }

        Self { pieces }
    }
}

impl fmt::Display for PrintableBoard {
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

// Don't look at this
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
    println!("{}", PrintableBoard::new(&board));

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
	println!("{}", PrintableBoard::new(&board));
    }
}
