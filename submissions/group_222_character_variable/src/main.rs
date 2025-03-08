fn main() {
    use std::io::{self, Write};
    
    let mut input = String::new();
    
    // Prompt for user input
    print!("Enter your name: ");
    io::stdout().flush().unwrap();
    
    // Read user input
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");
    
    // Get the first character, or use 'X' as fallback if string is empty
    let output: char = input.chars().next().unwrap_or('X');
    
    println!("First character: {}", output);
}
