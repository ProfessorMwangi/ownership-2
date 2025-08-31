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
    let (s1, length) = calculate_length(s1.trim().to_string());
    println!("The length of '{}' is {}.", s1, length);

}


