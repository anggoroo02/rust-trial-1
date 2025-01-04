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
#[test]
fn tupples() {
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup; //Ini adalah destructuring tuple. Artinya, kita membongkar nilai-nilai yang ada di dalam tuple tup dan menyimpannya dalam variabel yang terpisah.
    //note: compiler akan memberitahu jika ada variable yang tidak digunakan, untuk mengatasi ini menggunakan underscore(_) di depan variable untuk memberitahu compiler untuk mengabaikan variable tersebut
    println!("The value of y is: {}", y);
}
#[test]
fn tupple_array() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = x.0;
    let six_point_four = x.1;
    let _one = x.2; //caranya buat variable baru dan diisi dari x menggunakan titik untuk index
    println!("isi dari six_point_four: {}", six_point_four)
}
