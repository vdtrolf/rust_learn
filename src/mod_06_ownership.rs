use crate::mod_utils::{print_md_txt, print_title};

static TITLE: &str = " 6-Ownership";
static EXP_TEXT: &str = "**The rules of ownership are:**
- Each value has a variable that serves as its “owner.”
- A value can have only one owner at a time.
- If an owner goes out of scope, the value is cleaned up.

1. **Variable definitions** are in the stack, while the values are in the heap
> let s1 = String::from(\"hello\");
> let _s2 = s1;
> println(\"1. {}\",s1); ^( Error: s1 doesn't reference anything anymore )^
2. To preserve s1, it possible to **clone** it:
> let s1 = String::from(\"world\"); ^(possible since s1 has been discarded)^
> let _s3 = s1.clone();
> println!(\"2. s1 is now: {s1}\"); ^(works since s1 is still owner)^
3. Since variables only lives in their scope, ownership is lost out of the scope
> let s1 = String::from(\"hello\");
> { let _s2 = s1 };
> println(\"3. {}\",_s2); ^( Error: _s2 only lives in the inner scope )^
4. For **primitives** (i,u,f,bool,char) the value is also in the stack, so there is no ownership
> let x = 4;
> let _y = x;
> println!(\"4. x is : {x}\"); ^(works)^
5. When a value is passed to a **function** it's  ownership is also transfered
6. Unless the function returns a value, in which the ownership can 'be returned back'
7. For **primitives** this all doen't apply since the function always receives a copy of the valus";

pub fn learn_ownership(show_all: bool) {
    if show_all {
        print_title(TITLE);
        print_md_txt(EXP_TEXT);
    } else {
        println!("{}", TITLE);
    }
}

pub fn test_ownership() {
    // The rules of ownership are:
    // 1. Each value has a vprinprintln!("You entered: {}", input);tln!("You entered: {}", input);ariable that serves as its “owner.”
    // 2. A value can have only one owner at a time.
    // 3. If an owner goes out of scope, the value is cleaned up.

    // VALUES
    // ======

    let s1 = String::from("hello");
    let _s2 = s1;
    // This would now result in an error:  println!("s1 is now: {s1}");
    // Variable definition (s1, _s2) are in the stack, values ('Hello'),
    // are in the heap

    // Stack             Heap
    // s1, length=5 -->  "Hello"
    // _s2, length=? -->  ..println!("You entered: {}", input);.
    //
    // s1, length=5              (will be destroyed)
    // _s2, length=5 --> "Hello"

    let s1 = String::from("world"); // possible since s1 has been discarded
    let _s3 = s1.clone();
    println!("2. s1 is now: {s1}"); // works since s1 is still owner

    // Stack             Heap
    // s1, length=5 -->  "Hello"
    // _s2, length=? -->  ...
    //
    // s1, length=5 --> "Hello"             (will not be destroyed)
    // _s2, length=5 --> "Hello"            (this is a clone)

    // SCOPE

    // Since variables only lives in their scope, ownership is lost out of the scope
    let s1 = String::from("hello");
    {
        let _s2 = s1.clone();
    };
    // println(\"{}\",_s2); // Error: _s2 only lives in the inner scope

    // PRIMITIVES
    // ==========
    // For some primitives it is different, since they are stored
    // in the stack (integer, float, boolean, characters)
    // called: copy types or stack-allocated types

    let x = 4;
    let _y = x;
    println!("4. x is : {x}"); // works

    // Stack             Heap
    // x, val = 4        ...
    // y, ...            ...
    //
    // x, val = 4        ...     (will not be destroyed)
    // y, val = 4        ...     (this is a copy)

    // FUNCTIONS
    // =========
    // When a value is passed to a function taking ownership, itś ownership is also transfered:

    let vec_1 = vec![1, 2, 3];
    takes_ownership(vec_1);
    // println!("vec 1 is: {:?}", vec_1); // Gives an Error since vec_1 is no more owner

    let vec_2 = vec![4, 5];
    takes_ownership(vec_2.clone());
    println!("5. vec 2 is: {:?}", vec_2); // Works since vec_2 is still owner

    // FUNCTION RETURNING OWNERSHIP
    // ============================
    // In certain cases the function returns the ownership

    let vec_3 = gives_ownership();
    println!("6. vec 3 is: {:?}", vec_3);

    // Finally the function can receive and return the ownership

    let vec_4 = vec![10, 11, 12];
    let vec_5 = takes_and_gives_ownership(vec_4);
    println!("6. vec 5 is: {:?}", vec_5);

    // PRIMITIVE OWNERSHIP IN FUNCTIONS
    // ================================
    // When called with a primitive, a function always receive a copy of that primitive value
    // Which means it does not take over the ownership

    let x = 10;
    primitive_function(x);
    println!("7. In main, x is: {x}");
}

fn takes_ownership(vec: Vec<i32>) {
    println!("5. vec is: {:?}", vec);
}

fn gives_ownership() -> Vec<i32> {
    vec![7, 8]
}

fn takes_and_gives_ownership(mut vec: Vec<i32>) -> Vec<i32> {
    vec.push(13);
    vec
}

fn primitive_function(y: i32) {
    println!("7. In the function, y is: {y}");
}
