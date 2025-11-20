use rust_learn::mod_utils::print_md_txt;
use rust_learn::mod_utils::print_title;

static TITLE: &str = " 9-Struct";
static EXP_TEXT: &str = "**Classical structs** :
> struct Astruct { f1 : String, f2 : u8}
> let ^(mut)^ A = Astruct { f1 : \"tata\".to_string(), f2 : 17 };
> println!(\"{}\",A.f2); ^( 17 )^
> A.2 = 21; ^( possible if A is mutable )^

it is possible to reuse the fields of another instance with ..A at the end
> let B = Astruct {f1:val4, ..A}

it is also possible to initiate an instance with variables with the same name:
> let f1:u8 = 1;
> let f2:u8 = 3;
> let A = Astruct {f1, f2};

**Tupple structs** : struct Tstruct (type,type,type);
> let atupple = Tstruct(1,2,3);

**Unit structs** are used as markers:
> struct ABC;";

struct Animal {
    name: String,
    age: u16,
}

struct Mamal {
    name: String,
    age: u16,
    legs: u8,
}

struct Human {
    name: String,
    gender: u8,
    alive: bool,
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
    };
    zebra.age = 13;
    println!("Animal is a {:?}, aged {}", zebra.name, zebra.age);

    // It is poseible part of an instance values

    let man = Mamal {
        name: String::from("Toto"),
        age: 20,
        legs: 2,
    };
    println!("{:?} ({}) has {} legs", man.name, man.age, man.legs);

    let old_man = Mamal {
        age: 80,
        legs: 3,
        ..man
    };
    println!(
        "{:?} ({}) has {} legs",
        old_man.name, old_man.age, old_man.legs
    );

    // a field with the same can be used during an instance initialisation

    let gender = 1;
    let alive = true;
    let a_man = Human {
        name: String::from("Titi"),
        gender,
        alive,
    };
    println!("{:?} ({}) is {}", a_man.name, a_man.gender, a_man.alive);

    // It is also possible to use tupple structs like Point3d(i32, i32, i32);

    let a_point = Point3d(1, 2, 3);
    println!("Points are: {} {} {}", a_point.0, a_point.1, a_point.2);
}
