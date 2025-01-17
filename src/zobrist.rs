use crate::Board;

// https://matklad.github.io/2023/01/04/on-random-numbers.html
pub fn rng(seed: u32) -> impl Iterator<Item = u32> {
    let mut random = seed;
    std::iter::repeat_with(move || {
        random ^= random << 13;
        random ^= random >> 17;
        random ^= random << 5;
        random
    })
}

pub struct ZobristHasher {
    piece_squares: [[u32; 64]; 12],
    side_to_move: u32,
    castles: [u32; 4],
    en_passant_files: [u32; 8]
}

type KEY_SIZE = u32;
const SEED: u32 = 0b11101010100100100110000110011110;

impl ZobristHasher {
    pub fn init() -> Self {
        let mut rand = rng(SEED);

        let mut zobrist = Self {
            piece_squares: [[0; 64]; 12],
            side_to_move: 0,
            castles: [0; 4],
            en_passant_files: [0; 8]
        };
        for p in 0..12 {
            for sq in 0..64 {
                zobrist.piece_squares[p][sq] = rand.next().unwrap();
            }
        }
        zobrist.side_to_move = rand.next().unwrap();
        for c in 0..4 {
            zobrist.castles[c] = rand.next().unwrap();
        }
        for f in 0..8 {
            zobrist.en_passant_files[f] = rand.next().unwrap();
        }
        zobrist
    }

    pub fn hash(&self, board: &Board) -> u32 {
        let mut hash = 0;
        for sq in 0..64 {
            let (y, x) = (sq / 8, sq % 8);
            if let Some(piece) = board.board[y][x] {
                hash ^= self.piece_squares[piece.zobrist_index()][sq];
            }
        }
        if board.side_to_move {
            hash ^= self.side_to_move;
        }
        for castle in 0..4 {
            if board.allowed_castling[castle] {
                hash ^= self.castles[castle];
            }
        }
        if let Some((_, x)) = board.en_passant {
            hash ^= self.en_passant_files[x];
        }
        hash
    }
}