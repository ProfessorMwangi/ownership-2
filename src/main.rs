fn calculate_length(s: String){
    let s1 = s;
    let length = s1.len();
    println!("The length of '{}' is {}.", s1, length);

}
fn main() {
    let s1= String::from("hello");
    calculate_length(s1);
}


