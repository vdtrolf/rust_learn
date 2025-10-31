use colored::Colorize;

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

pub fn learn_struct() {
    println!("{}", "Struct (9)".red().bold().underline());
    println!("Classical structs: struct Astruct {{ f1 : type, f2 : type, f3, type}}");
    println!(
        "  | let (mut) A = Astruct {{ f1 : val1, f2 : val2, f3: val3 }}; println!(\"{{}}\",A.f2);"
    );
    println!("  | A.1 = val3; // possible if A is mutable");
    println!("  it is possible to reuse the fields of another instance with ..A at the end");
    println!("  | let B = Astruct {{f1:val4, ..A}}");
    println!("  it is also possible to initiate an instance with vraiables with the same name:");
    println!("  | let f1:u8 = 1; let f2:u8 = 3; let A = Astruct {{f1, f2}};");
    println!("Tupple structs: struct Tstruct (type,type,type);");
    println!("  | let atupple = Tstruct(1,2,3;");
    println!("Unit structs are used as markers: struct ABC;  ");
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
