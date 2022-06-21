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

impl Token {
    fn from_u8(n: u8) -> Token {
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
}

const BLOCK: usize = 3;
const SIDE: usize = BLOCK * BLOCK;

type AbstractPuzzle<A> = [[A; SIDE]; SIDE];
pub type Puzzle = AbstractPuzzle<EnumSet<Token>>;

pub struct Game {
    puzzle: Puzzle,
    rows: [EnumSet<Token>; SIDE],
    columns: [EnumSet<Token>; SIDE],
    blocks: [[EnumSet<Token>; BLOCK]; BLOCK],
}

impl Game {
    pub fn new(matrix: AbstractPuzzle<u8>) -> Game {
        let mut game: Game = Game {
            puzzle: [[EnumSet::all(); SIDE]; SIDE],
            rows: [EnumSet::empty(); SIDE],
            columns: [EnumSet::empty(); SIDE],
            blocks: [[EnumSet::empty(); BLOCK]; BLOCK],
        };
        for y in 0..SIDE {
            for x in 0..SIDE {
                if matrix[y][x] != 0 {
                    game.puzzle[y][x] = EnumSet::only(Token::from_u8(matrix[y][x]));
                }
            }
        }
        return game;
    }
    fn over(&self) -> bool {
        for y in 0..SIDE {
            for x in 0..SIDE {
                if self.puzzle[y][x].len() > 1 {
                    return false;
                }
            }
        }
        return true;
    }
    fn play(&mut self) {
        for y in 0..SIDE {
            for x in 0..SIDE {
                match self.puzzle[y][x].len() {
                    1 => self.update_at(x, y),
                    _ => self.refine_at(x, y),
                }
            }
        }
    }

    fn update_at(&mut self, x: usize, y: usize) {
        self.rows[y] |= self.puzzle[y][x];
        self.columns[x] |= self.puzzle[y][x];
        self.blocks[y / BLOCK][x / BLOCK] |= self.puzzle[y][x];
    }

    fn refine_at(&mut self, x: usize, y: usize) {
        self.puzzle[y][x] =
            self.puzzle[y][x] - self.rows[y] - self.columns[x] - self.blocks[y / BLOCK][x / BLOCK];
    }
}

pub fn solve(game: &mut Game) {
    while !game.over() {
        game.play();
    }
}

#[cfg(test)]
mod tests {

    use super::{solve, Game, Token};

    fn game_1() -> Game {
        return Game::new([
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

    fn game_2() -> Game {
        return Game::new([
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
        let mut game = game_1();
        for y in 0..super::SIDE {
            for x in 0..super::SIDE {
                if game.puzzle[y][x].len() == 1 {
                    game.update_at(x, y)
                }
            }
        }
        assert_eq!(game.rows[0], Token::_8 | Token::_5 | Token::_4);
    }

    #[test]
    fn column_for() {
        let mut game = game_1();
        for y in 0..super::SIDE {
            for x in 0..super::SIDE {
                if game.puzzle[y][x].len() == 1 {
                    game.update_at(x, y)
                }
            }
        }
        assert_eq!(game.columns[0], Token::_8 | Token::_9 | Token::_4);
    }

    #[test]
    fn block_for() {
        let mut game = game_1();
        for y in 0..super::SIDE {
            for x in 0..super::SIDE {
                if game.puzzle[y][x].len() == 1 {
                    game.update_at(x, y)
                }
            }
        }
        assert_eq!(game.blocks[0][0], Token::_8 | Token::_5 | Token::_2);
        assert_eq!(game.blocks[1][1], Token::_9 | Token::_7 | Token::_5);
    }

    #[test]
    fn test_1() {
        let solution = Game::new([
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
        let mut game = game_1();
        solve(&mut game);
        assert_eq!(game.puzzle, solution.puzzle);
    }

    #[test]
    fn test_2() {
        let solution = Game::new([
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
        let mut game = game_2();
        solve(&mut game);
        assert_eq!(game.puzzle, solution.puzzle);
    }
}
