use day14::*;

fn main() {
    divan::main();
}

#[divan::bench]
fn part1() {
    solve_part1(divan::black_box(include_str!("../input.txt")));
}

#[divan::bench]
fn part2() {
    solve_part2(divan::black_box(include_str!("../input.txt")));
}
