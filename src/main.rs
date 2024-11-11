pub fn square(s: u32) -> u64 {
    match s {
        1..=64 => 2u64.pow(s - 1),
        _ => panic!("Square must be between 1 and 64"),
    }
}

pub fn total() -> u64 {
    (1..=64).map(square).sum()
}
fn main() {
    let test_squares = vec![1,2,10,64,0,65];
    for &square_num in &test_squares {
        match std::panic::catch_unwind(|| square(square_num)){
            Ok(result) => println!("Square of {}: {}", square_num, result),
            Err(_) => println!("Square of {}: Panic occurred", square_num),
        }
    }
    println!("Total grains on the chessboard: {}", total());
}