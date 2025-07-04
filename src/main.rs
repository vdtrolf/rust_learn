fn main() {
     // Static (referenced to)
    static PI_VALUE: f32 = 3.14162758;

    // Constants (replaced during compil)
    const MAX_VAL: u8 = 128;
    const ARRAY_POS:usize = 1;

    // Variables
    let mut x:u8 = 11; // mut for mutable
    let y:u8 = 12; 
    let _z:bool = true; // not used _
    x += y;
    let a_char: char = 'f';
    let a_string = "A fixed string"; // immutable string
    let mut a_mut_string = String::from("tes"); // muttable string
    a_mut_string.push('t');

    println!("Hello, world with {a_string}! and a mutable: {a_mut_string}");

    // Arrays
    let tst_array: [u8; 3 ] = [1,2,4];
    let tst_array2 = [0;3];

    println!("Hello, world {x} / {MAX_VAL} - {tst_array:?}!");
    x -= tst_array[ARRAY_POS];
    println!("And this {} {} {:?}",x,a_char,tst_array2);

    // Vectors
    let vec = vec![1,2,3];
    let vec_2 : Vec<i16> = vec![-1,2,4];
    let elem = vec[1];
    println!("Ans a vector: {:?} : {elem} : {}", vec, vec_2[0]);

    // Tuples
    let tupl = ("Test",23,11.0);
    println!("This is in a tuple value and a static: {}, {PI_VALUE}",tupl.1);

    let (x1, x2, x3) = tupl;
    println!("Tupl extract {}, {}, {}", x1,x2,x3);

}
