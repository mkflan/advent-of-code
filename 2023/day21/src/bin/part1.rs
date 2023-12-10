use day21::solve_part1;

#[cfg(feature = "dhat-heap")]
#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

fn main() {
    #[cfg(feature = "dhat-heap")]
    let _heap_profiler = dhat::Profiler::new_heap();

    let input = include_str!("../../input.txt");
    let answer = solve_part1(input);
    println!("Answer: {answer}");
}
