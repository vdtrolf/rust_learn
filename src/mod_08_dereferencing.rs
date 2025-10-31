use colored::Colorize;

pub fn learn_dereferencing() {
    println!("{}", "Dereferencing (8)".red().bold().underline());
    println!("Dereferencing means: accesing a variable value pointed by a reference or pointer");
    println!("This is usefull after borrowing a a reference, and it is necessary to manipulate");
    println!("or access the underlying data. Works for stack variables, not heap values");
    println!("  let mut x = 42;");
    println!("  let ref1 = &mut x; // mutable reference");
    println!("  let deref_copy = *ref1; // creates a copy of the value");
    println!("  *ref1 = 13;  // x = 13 and deref_copy = 42");
}

pub fn test_dereferencing() {
    // Dereferencing means: accesing a variable value pointed by a reference or pointer
    // This is usefull after borrowing a a reference, and it is necessary to manipulate
    // or access the underlying data. Works for stack variables, not heap values

    let mut some_data = 42;
    let ref1 = &mut some_data; // mutable reference
    let deref_copy = *ref1; // creates a copy of the value
    *ref1 = 13; // mutates the original value
    println!("some_data: {some_data}, deref_copy: {deref_copy}");

    // For heap values, clone does the same job

    let mut heap_data = vec![5, 6, 7];
    let ref2 = &mut heap_data;
    let deref2_copy = ref2.clone();
    ref2.push(8);
    println!("hepa_data: {:?}, deref2_copy: {:?}", heap_data, deref2_copy);

    // It is possible to combine dereferencing and borrowing:

    let mut x = 45;
    let z = &mut x;
    let y = &*z;
    println! {"z = {z}, y = {y}"}
}
