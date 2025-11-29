use crate::mod_utils::{print_md_txt, print_title};

static TITLE: &str = " 9-Struct";
static EXP_TEXT: &str = "1. **Classical structs** :
> struct Animal {name: String, age: u8, legs: u8}
> let mut zebra = Animal {
>    name: String::from(\"Albert\"),
>    age: 12,
>    legs: 4};
> zebra.age = 13; ^(possible since zebra is mut)^
it is possible to **reuse the fields of another instance** with ..XXX at the end
> let zebra_two = Mamal {
>    name: String::from(\"Jerome\"),
>    ..zebra};    ^(reuse the othe fields of zebra)^
it is also possible to initiate an instance with variables with the same name:
> let age:u8 = 1;
> let legs:u8 = 3;
> let bird = Animal{name:String::from(\"titi\"),age, legs};
**Tupple structs** : struct Tstruct (type,type,type);
> struct Point3d(i32, i32, i32);
> let a_point = Point3d(1, 2, 3);
**Unit structs** are used as markers:
> struct ABC;";

struct Animal {
    name: String,
    age: u8,
    legs: u8,
}

struct Point3d(i32, i32, i32);

pub fn learn_struct(show_all: bool) {
    if show_all {
        print_title(TITLE);
        print_md_txt(EXP_TEXT);
    } else {
        println!("{}", TITLE);
    }
}

pub fn test_struct() {
    // If mutable, instances values can be changed

    let mut zebra = Animal {
        name: String::from("Albert"),
        age: 12,
        legs: 4,
    };
    zebra.age = 13;
    println!(
        "Animal is {:?}, aged {}, legs: {}",
        zebra.name, zebra.age, zebra.legs
    );

    // It is poseible part of an instance values

    let zebra_two = Animal {
        name: String::from("Jerome"),
        ..zebra
    };
    println!(
        "Animal is {:?}, aged {}, legs: {}",
        zebra_two.name, zebra_two.age, zebra_two.legs
    );

    // a field with the same can be used during an instance initialisation

    let age: u8 = 1;
    let legs: u8 = 3;
    let bird = Animal {
        name: String::from("titi"),
        age,
        legs,
    };
    println!(
        "Animal is {:?}, aged {}, legs: {}",
        bird.name, bird.age, bird.legs
    );

    // It is also possible to use tupple structs like Point3d(i32, i32, i32);

    let a_point = Point3d(1, 2, 3);
    println!("Points are: {} {} {}", a_point.0, a_point.1, a_point.2);
}
