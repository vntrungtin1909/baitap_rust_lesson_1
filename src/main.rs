fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from(" world!");
    let s3 = s1 + &s2;
    println!("{}, {}", s2, s3);
}
