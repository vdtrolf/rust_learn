pub fn learn_borrowing() {
    
    println!("Part 7: borrowing");
    println!("=================");

    // Borrowing means: referencing to a value without taking ownership
    // for example when it is only necessary to read the data
    // Borrowing is more efficient than ownership, because values in the heap are not affected

    /* Rules 
       =====
       - One mutable reference (&mut) in the scope OR several immutable references (&)
       - References must be valid
    */

    let mut vec_1 = vec![1,2,3]; // necessary to be mut since it is referenced by mutable references
    let vec_2 = vec![4,5];
    let ref1 = &mut vec_1;       // another &mut reference would generate an error
    let ref2 = &vec_2;           // immutable reference
    let ref3 = &vec_2;           // second immutable reference

    println!("refs are {:?}, {:?} and {:?}",ref1,ref2,ref3);

    // However SCOPE rule applies:
    
    let mut vec_3 = vec![1,2,3]; // necessary to be mut since it is referenced by mutable references
    let _ref4 = &mut vec_3;
    {
        let ref5 = &mut vec_3;   // Possible because in another scope
        println!("Second mut ref in inner scope {:?}",ref5); 
    }
    // println!("Second mut ref in inner scope {:?}",ref4);  // would generate an error
    
    // RULE: It is not possible to mix mutable and immutable references

    let vec_4 = vec![4, 5, 6];                                            // Could be mut
    let _ref6 = &vec_4;
    let _ref7 = &vec_4;
    // let ref8 = &mut vec_1;                                             // Error
    // println!("ref6: {:?}, ref7: {:?}, ref8: {:?}", ref6, ref7, ref8);  // ref8 in scope ref6 & ref7
    
    // But SCOPE applies again:

    let mut vec_5 = vec![4, 5, 6];                    //
    let refa = &vec_1;                                // -
    let refb = &vec_1;                                // | scope of refa & refb 
    println!("refa: {:?}, refb: {:?}", refa, refb);   // -
    let refc = &mut vec_5;                            // -       
    println!("refc: {:?}", refc);                     // | scope of refc

    // this is possible because the scope of refa & refb ended before the scope of refc

    // In functions:

}