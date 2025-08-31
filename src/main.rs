use std::io;
fn calculate_length(s: String)-> (String, usize) {
    let s1 = s;
    let length = s1.len();
    (s1, length)

}
fn main() {
    println!("Enter a string to calculate its length:");
    let mut s1= String::new();
    io::stdin().read_line(&mut s1).expect("Failed to read line");
/*************  ✨ Windsurf Command 🌟  *************/
    let (s1, length) = calculate_length(s1.trim().to_string());
    println!("The length of '{}' is {}.", s1, length);
    // The trim() function is used to remove the newline character from the string.
    // This is because the read_line() function includes the newline character in the string.
    // The to_string() function is used to convert the trimmed string to a String.
/*******  4ee0d798-694d-4ec0-b8ab-7c5df8b6c89e  *******/

}


