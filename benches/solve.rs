use criterion::{black_box, criterion_group, criterion_main, Criterion};

use sudoku::Game;

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

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("sudoku::solve(game_1)", |b| {
        b.iter_with_setup(game_1, |mut game| sudoku::solve(black_box(&mut game)))
    });
    c.bench_function("sudoku::solve(game_2)", |b| {
        b.iter_with_setup(game_2, |mut game| sudoku::solve(black_box(&mut game)))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
