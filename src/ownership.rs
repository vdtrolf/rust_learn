pub fn learn_ownership() {
    
    println!("Part 6: ownership");
    println!("=================");

    // The rules of ownership are:
    // 1. Each value has a variable that serves as its “owner.”
    // 2. A value can have only one owner at a time.
    // 3. If an owner goes out of scope, the value is cleaned up.


    let s1 = String::from("hello");
    let _s2 = s1;
    // This would now result in an error:  println!("s1 is now: {s1}"); 
    // Variable definition (s1, _s2) are in the stack, values ('Hello'), 
    // are in the heap

    // Stack             Heap
    // s1, length=5 -->  "Hello"
    // _s2, length=? -->  ...
    //
    // s1, length=5              (will be destroyed)
    // _s2, length=5 --> "Hello"

    let s1 = String::from("world"); // possible since s1 has been discarded
    let _s3 = s1.clone();
    println!("s1 is now: {s1}"); // works since s1 is still owner 

    // Stack             Heap
    // s1, length=5 -->  "Hello"
    // _s2, length=? -->  ...
    //
    // s1, length=5 -->  "Hello"             (will not be destroyed)
    // _s2, length=5 --> "Hello"            (this is a clone)

    // For some primitives it is different, since they are stored
    // in the stack (integer, float, boolean, characters)
    // called: copy types or stack-allocated types

    let x = 4;
    let _y = x;
    println!("x is : {x}"); // works

    // Stack             Heap
    // x, val = 4        ...
    // y, ...            ...
    //
    // x, val = 4        ...     (will not be destroyed)
    // y, val = 4        ...     (this is a copy)

    // Ownership is also limited to the scope
    
    let s1 = String::from("world");
    {
        let s2 = s1.clone();
    }
    // println!("s2 is: {s2}"); gives an error, since s2 only exist in the innner scope





}