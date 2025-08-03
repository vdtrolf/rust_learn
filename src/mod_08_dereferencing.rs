pub fn learn_dereferencing() {
    
    println!("Part 8: dereferencing");
    println!("=====================");

    // Dereferencing means: 
    // for example 
    //
    
    let mut some_data = 42;
    let ref1 = &mut some_data;
    let deref_copy = *ref1;
    *ref1 = 13;
    println!("some_data is: {some_data}, deref_copy is: {deref_copy}");



}