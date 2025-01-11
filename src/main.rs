fn main() {
    fn say_hello_word() { //membuat fungsi say_hello_word
        println!("Hello, world!");
    }
    fn argument(x:i32) {
        println!("nilai dari x adalah: {}",x)
    }
    // fn argument_manual(){
    //     let mut input = String::new();
    //     println!("Masukkan nilai x: ");
    //     io::stdin().read_line(&mut input).expect("Failed to read line");
    //     let x: i32 = input.trim().parse().expect("Invalid input");
    //     println!("isi variable x adalah: {}", x)
    // }
    argument(12);
    say_hello_word(); //memanggil fungsi say_hello_word
    // argument_manual(); //memanggil fungsi argument dengan parameter yang diinput
}
// test untuk tipe data
#[test]
fn data_type() {
    let mut angka:i64=-2006;
    angka +=300;
    println!("isi dari variable angka adalah: {}", angka)
}
//test untuk boolean
#[test]
fn boolean_prove() {
    let mut t: bool = true;
    println!("nilai boolean dari t: {}", t);
    t = false;
    println!("nilai boolean dari t: {}", t);
}
//test untuk tupple
#[test]
fn tupples() {
    let tup = (500, 6.4, 1);
    let (_x, y, _z) = tup; //Ini adalah destructuring tuple. Artinya, kita membongkar nilai-nilai yang ada di dalam tuple tup dan menyimpannya dalam variabel yang terpisah.
    //note: compiler akan memberitahu jika ada variable yang tidak digunakan, untuk mengatasi ini menggunakan underscore(_) di depan variable untuk memberitahu compiler untuk mengabaikan variable tersebut
    println!("The value of y is: {}", y);
}
//test untuk tupple dengan index array
#[test]
fn tupple_array() {
    let x: (i32, f64, u8) = (500, 6.4, 1);
    let _five_hundred = x.0;
    let six_point_four = x.1;
    let _one = x.2; //caranya buat variable baru dan diisi dari x menggunakan titik untuk index
    println!("isi dari six_point_four: {}", six_point_four)
}
//test untuk array
#[test]
fn array(){
    let a: [i32; 5] = [1, 2, 3, 4, 5];//a adalah sebuah array yang memiliki 5 elemen dan tipe data i32 (integer 32-bit).
    let b = [1; 5];//Sintaks [1; 5] adalah cara singkat untuk membuat array dengan 5 elemen, di mana setiap elemen diinisialisasi dengan nilai 1.
    let c = [0; 5];
    let d = [2; 10];
    println!("Array a: {:?}", a);
    println!("Array b: {:?}", b);
    println!("Array c: {:?}", c);
    println!("Array d: {:?}", d);
    //jadi dari var b, c, d, berisi array dengan nilai yang sama, tapi dengan ukuran yang berbeda masing-masing variable.
}
#[test]
fn array_2(){
    let a = [1, 2, 3, 4, 5];
    let index = 0;
    let element = a[index];
    println!("The value of element is: {}", element);
    //mendeklarasikan array a dengan nilai [1, 2, 3, 4, 5], kemudian mengakses elemen pertama (indeks 0) dari array tersebut, menyimpannya dalam variabel element, dan mencetak nilai element, yaitu 1.
}
#[test]
fn espressions(){
    let y = {
        let x = 3;
        x + 1 // Tidak ada titik koma, ini adalah ekspresi yang menghasilkan nilai 4
    };
    println!("The value of y is: {}", y);
}