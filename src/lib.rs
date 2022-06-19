use enumset::{EnumSet, EnumSetType};

#[derive(EnumSetType, Debug)]
pub enum Token {
    _1,
    _2,
    _3,
    _4,
    _5,
    _6,
    _7,
    _8,
    _9,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Cell {
    Solved(Token),
    Unsolved(EnumSet<Token>),
}

const BLOCK: usize = 3;
const SIDE: usize = BLOCK * BLOCK;

trait Puzzle {
    fn row_for(&self, y: usize) -> EnumSet<Token>;
    fn column_for(&self, x: usize) -> EnumSet<Token>;
    fn block_for(&self, x: usize, y: usize) -> EnumSet<Token>;
}

impl Puzzle for [[Cell; SIDE]; SIDE] {
    fn row_for(&self, y: usize) -> EnumSet<Token> {
        let mut row = EnumSet::empty();
        for x in 0..SIDE {
            if let Cell::Solved(token) = self[y][x] {
                row |= token;
            }
        }
        return row;
    }
    fn column_for(&self, x: usize) -> EnumSet<Token> {
        let mut column = EnumSet::empty();
        for y in 0..SIDE {
            if let Cell::Solved(token) = self[y][x] {
                column |= token;
            }
        }
        return column;
    }
    fn block_for(&self, x: usize, y: usize) -> EnumSet<Token> {
        let mut block = EnumSet::empty();
        let x_offset = x / BLOCK * BLOCK;
        let y_offset = y / BLOCK * BLOCK;
        for y in 0..BLOCK {
            for x in 0..BLOCK {
                if let Cell::Solved(token) = self[y + y_offset][x + x_offset] {
                    block |= token;
                }
            }
        }
        return block;
    }
}

fn refine(puzzle: &mut [[Cell; SIDE]; SIDE]) {
    for y in 0..SIDE {
        for x in 0..SIDE {
            if let Cell::Unsolved(candidates) = puzzle[y][x] {
                puzzle[y][x] = Cell::Unsolved(
                    candidates - puzzle.row_for(y) - puzzle.column_for(x) - puzzle.block_for(x, y),
                );
            }
        }
    }
}

fn step(puzzle: &mut [[Cell; SIDE]; SIDE]) -> u8 {
    let mut solved = 0;
    for y in 0..SIDE {
        for x in 0..SIDE {
            if let Cell::Unsolved(candidates) = puzzle[y][x] {
                if candidates.len() == 1 {
                    puzzle[y][x] = Cell::Solved(candidates.iter().next().unwrap());
                    solved += 1;
                }
            }
        }
    }
    return solved;
}

pub fn solve(puzzle: &mut [[Cell; SIDE]; SIDE]) {
    loop {
        refine(puzzle);
        if step(puzzle) == 0 {
            break;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::{solve, Cell, Puzzle, Token, SIDE};
    use enumset::EnumSet;

    fn token_from_u8(n: u8) -> Token {
        match n {
            1 => Token::_1,
            2 => Token::_2,
            3 => Token::_3,
            4 => Token::_4,
            5 => Token::_5,
            6 => Token::_6,
            7 => Token::_7,
            8 => Token::_8,
            9 => Token::_9,
            _ => panic!("{} is not a valid input", n),
        }
    }

    fn new_puzzle(matrix: [[u8; SIDE]; SIDE]) -> [[Cell; SIDE]; SIDE] {
        let mut puzzle: [[Cell; SIDE]; SIDE] = [[Cell::Unsolved(EnumSet::all()); SIDE]; SIDE];
        for y in 0..SIDE {
            for x in 0..SIDE {
                if matrix[y][x] != 0 {
                    puzzle[y][x] = Cell::Solved(token_from_u8(matrix[y][x]));
                }
            }
        }
        return puzzle;
    }

    fn puzzle_1() -> [[Cell; SIDE]; SIDE] {
        return new_puzzle([
            [8, 0, 5, 4, 0, 0, 0, 0, 0],
            [0, 0, 2, 0, 0, 0, 0, 4, 5],
            [0, 0, 0, 0, 6, 0, 2, 9, 0],
            [9, 4, 6, 0, 0, 0, 0, 1, 0],
            [0, 7, 0, 0, 9, 0, 0, 0, 0],
            [0, 2, 0, 7, 0, 5, 0, 3, 0],
            [0, 5, 0, 0, 0, 4, 7, 0, 2],
            [0, 8, 0, 0, 1, 0, 4, 0, 0],
            [4, 6, 0, 0, 5, 0, 3, 0, 0],
        ]);
    }

    fn puzzle_2() -> [[Cell; SIDE]; SIDE] {
        return new_puzzle([
            [0, 0, 0, 1, 0, 5, 0, 7, 0],
            [2, 0, 0, 0, 0, 6, 0, 3, 0],
            [0, 0, 3, 0, 0, 8, 0, 4, 0],
            [0, 0, 5, 8, 0, 2, 0, 0, 3],
            [8, 0, 2, 0, 0, 4, 7, 0, 0],
            [1, 9, 6, 0, 0, 0, 4, 8, 0],
            [3, 7, 8, 0, 6, 0, 5, 1, 0],
            [4, 2, 0, 5, 0, 0, 3, 0, 0],
            [0, 6, 0, 4, 7, 3, 0, 2, 9],
        ]);
    }

    #[test]
    fn row_for() {
        assert_eq!(puzzle_1().row_for(0), Token::_8 | Token::_5 | Token::_4);
    }

    #[test]
    fn column_for() {
        assert_eq!(puzzle_1().column_for(0), Token::_8 | Token::_9 | Token::_4);
    }

    #[test]
    fn block_for() {
        assert_eq!(
            puzzle_1().block_for(0, 0),
            Token::_8 | Token::_5 | Token::_2
        );
        assert_eq!(
            puzzle_1().block_for(2, 2),
            Token::_8 | Token::_5 | Token::_2
        );
        assert_eq!(
            puzzle_1().block_for(3, 3),
            Token::_9 | Token::_7 | Token::_5
        );
    }

    #[test]
    fn test_1() {
        let solution = new_puzzle([
            [8, 1, 5, 4, 2, 9, 6, 7, 3],
            [6, 9, 2, 3, 7, 8, 1, 4, 5],
            [7, 3, 4, 5, 6, 1, 2, 9, 8],
            [9, 4, 6, 8, 3, 2, 5, 1, 7],
            [5, 7, 3, 1, 9, 6, 8, 2, 4],
            [1, 2, 8, 7, 4, 5, 9, 3, 6],
            [3, 5, 1, 9, 8, 4, 7, 6, 2],
            [2, 8, 7, 6, 1, 3, 4, 5, 9],
            [4, 6, 9, 2, 5, 7, 3, 8, 1],
        ]);
        let mut puzzle = puzzle_1();
        solve(&mut puzzle);
        assert_eq!(puzzle, solution);
    }

    #[test]
    fn test_2() {
        let solution = new_puzzle([
            [9, 8, 4, 1, 3, 5, 2, 7, 6],
            [2, 5, 7, 9, 4, 6, 1, 3, 8],
            [6, 1, 3, 7, 2, 8, 9, 4, 5],
            [7, 4, 5, 8, 1, 2, 6, 9, 3],
            [8, 3, 2, 6, 9, 4, 7, 5, 1],
            [1, 9, 6, 3, 5, 7, 4, 8, 2],
            [3, 7, 8, 2, 6, 9, 5, 1, 4],
            [4, 2, 9, 5, 8, 1, 3, 6, 7],
            [5, 6, 1, 4, 7, 3, 8, 2, 9],
        ]);
        let mut puzzle = puzzle_2();
        solve(&mut puzzle);
        assert_eq!(puzzle, solution);
    }
}
