fn main() {
    println!("Hello, world!");
}
// test untuk tipe data
#[test]
fn data_type() {
    let mut angka:i64=-2006;
    angka +=300;
    println!("isi dari variable angka adalah: {}", angka)
}
#[test]
fn boolean_prove() {
    let mut t: bool = true;
    println!("nilai boolean dari t: {}", t);
    t = false;
    println!("nilai boolean dari t: {}", t);
}