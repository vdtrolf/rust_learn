use colored::Colorize;

pub fn learn_borrowing() {
    println!("{}", "Borrowing (7)".red().bold().underline());
    println!("Borrowing means: referencing to a value without taking ownership for example when it is only necessary to read the data");
    println!("It is more efficient than ownership, because values in the heap are not affected");
    println!("\nOne mutable reference (&mut) in the scope OR several immutable references (&) and References must be valid");
    println!(
        "{}{}",
        "Mutable  ".cyan().bold(),
        ": let mut vec_1 = vec![1, 2, 3]; let ref1 = &mut vec_1;"
    );
    println!(
        "{}{}",
        "Immutable".cyan().bold(),
        ": let vec_2 = vec![4, 5]; let ref2 = &vec_2; let ref3 = &vec_2;"
    );
}

pub fn test_borrowing() {
    // Borrowing means: referencing to a value without taking ownership
    // for example when it is only necessary to read the data
    // Borrowing is more efficient than ownership, because values in the heap are not affected

    /* Rules
       =====
       - One mutable reference (&mut) in the scope OR several immutable references (&)
       - References must be valid
    */

    let mut vec_1 = vec![1, 2, 3]; // necessary to be mut since it is referenced by mutable references
    let vec_2 = vec![4, 5];
    let ref1 = &mut vec_1; // another &mut reference would generate an error
    let ref2 = &vec_2; // immutable reference
    let ref3 = &vec_2; // second immutable reference

    println!("refs are {:?}, {:?} and {:?}", ref1, ref2, ref3);

    // However SCOPE rule applies:

    let mut vec_3 = vec![1, 2, 3]; // necessary to be mut since it is referenced by mutable references
    let _ref4 = &mut vec_3;
    {
        let ref5 = &mut vec_3; // Possible because in another scope
        println!("Second mut ref in inner scope {:?}", ref5);
    }
    // println!("Second mut ref in inner scope {:?}",ref4);  // would generate an error

    // RULE: It is not possible to mix mutable and immutable references

    let vec_4 = vec![4, 5, 6]; // Could be mut
    let _ref6 = &vec_4;
    let _ref7 = &vec_4;
    // let ref8 = &mut vec_1;                                             // Error
    // println!("ref6: {:?}, ref7: {:?}, ref8: {:?}", ref6, ref7, ref8);  // ref8 in scope ref6 & ref7

    // But SCOPE applies again:

    let mut vec_5 = vec![4, 5, 6]; //
    let refa = &vec_1; // -
    let refb = &vec_1; // | scope of refa & refb
    println!("refa: {:?}, refb: {:?}", refa, refb); // -
    let refc = &mut vec_5; // -
    println!("refc: {:?}", refc); // | scope of refc

    // this is possible because the scope of refa & refb ended before the scope of refc

    // In functions: immutable

    let vec_6 = vec![4, 5, 6];
    let refd = &vec_6;
    borrows_vec(refd);
    println!("vec_6: {:?}", vec_6);

    // In functions: mutable - takes and give

    let vec_7 = vec![21, 22, 23];
    let refe = &vec_7;
    borrows_vec(refe);
    // add_value(vec_7);               // Generates an error because add_values doesnt borrow
    let vec_7 = add_value(vec_7); // Works because it takes an gives, renewing ownership
    println!("vec_7: {:?}", vec_7);

    // In functions: mutable - borrow

    let mut vec_8 = vec![18, 19, 20];
    let refg = &vec_8;
    borrows_vec(refg);
    add_borrowed_value(&mut vec_8);
    println!("vec_8: {:?}", vec_8);

    // NB returning a reference to a value when the ownership has been passed to the function
    // is a bad idea, since the value will be cleared when the owning function will be cleared
}

fn borrows_vec(vec: &Vec<i32>) {
    println!("In the borrow function: {:?}", vec);
}

fn add_value(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(24);
    vec
}

fn add_borrowed_value(vec: &mut Vec<i32>) {
    vec.push(21);
}
