use criterion::{black_box, criterion_group, criterion_main, Criterion};

use enumset::EnumSet;
use sudoku::Token;

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

fn new_puzzle(matrix: [[u8; 9]; 9]) -> [[EnumSet<Token>; 9]; 9] {
    let mut puzzle: [[EnumSet<Token>; 9]; 9] = [[EnumSet::all(); 9]; 9];
    for y in 0..9 {
        for x in 0..9 {
            if matrix[y][x] != 0 {
                puzzle[y][x] = EnumSet::only(token_from_u8(matrix[y][x]));
            }
        }
    }
    return puzzle;
}

fn puzzle_1() -> [[EnumSet<Token>; 9]; 9] {
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

fn puzzle_2() -> [[EnumSet<Token>; 9]; 9] {
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

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sudoku::solve(puzzle_1)", |b| {
        b.iter_with_setup(puzzle_1, |mut puzzle| sudoku::solve(black_box(&mut puzzle)))
    });
    c.bench_function("sudoku::solve(puzzle_2)", |b| {
        b.iter_with_setup(puzzle_2, |mut puzzle| sudoku::solve(black_box(&mut puzzle)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
