
struct Animal {
  name : String,
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

pub fn learn_struct() {
    
  println!("Part 9: struct");
  println!("==============");

  // If mutable, instances values can be changed

  let mut zebra = Animal {
    name: String::from("Albert"),
    age: 12,
  };
  zebra.age = 13;
  println!("Animal is a {:?}, aged {}",zebra.name,zebra.age);
    
  // It is poseible part of an instance values 

  let man = Mamal {
    name: String::from("Toto"),
    age:20,
    legs: 2,
  };
  println!("{:?} ({}) has {} legs",man.name,man.age,man.legs);

  let old_man = Mamal {
    age: 80,
    legs: 3,
    ..man
  };
  println!("{:?} ({}) has {} legs",old_man.name,old_man.age,old_man.legs);

  // a field with the same can be used during an instance initialisation

  let gender = 1;
  let alive = true;
  let a_man = Human {
    name: String::from("Titi"),
    gender,
    alive,
  };
  println!("{:?} ({}) is {}",a_man.name,a_man.gender,a_man.alive);


}
