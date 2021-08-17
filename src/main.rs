fn main() {
    fill_the_stack();

    let name = whoami::username();

    println!("name address: {:?}", name.as_bytes().as_ptr());

    println!(
        "Hello, {:?}. Would you like to play a game of thermonuclear war?",
        name
    );
}

/// This leaves behind non-zero bytes on the stack.
///
/// NOTE: In release mode, this will probably get optimized out
/// because the return value can be computed at compile time.
fn fill_the_stack() -> i32 {
    // If you make this shorter, you won't observe the bug.
    let nums = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16];
    nums.iter().sum()
}
