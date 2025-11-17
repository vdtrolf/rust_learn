use rust_learn::mod_utils::print_md_txt;
use rust_learn::mod_utils::print_title;

static TITLE: &str = "10-Implementation";
static EXP_TEXT: &str =
"An implementation block allows to reference to a certain type, such as a struct.
A function in such a block is called a method and doesn't expect any argument (self is implicit)
> stuct AStruct { f1:u8, f2:u8};
> impl AStruct {
>  fn do_it(&self) { println!(\"{self.f1}\") };
> }; ^( impl block with a function inside )^
> let A = AStruct { f1:1, f2:2 };
> A.do_it(); ^( doit will generate self and will print 1 )^

It is possible to send arguments and even change the value of self
> impl AStruct { fn do_more(&mut self, p1: u8) { self.f1 += p1 }; };
> let A = AStruct { f1:1, f2:2 }; A.do_more(2); ^( f1 has now the value 3 )^

**A method** can also take and transmit the ownership
> impl AStruct { fn move_it(self) -> Self) { self }; }; ^( Self = type of self )^
> let A = AStruct { f1:1, f2:2 }; let B = A.move_it(); ^( B has now the ownership of the struct )^

**An associated function** is a function part of the same implementation - can be called with Struct::
> impl AStruct { ... fn raise_it(&mut self) { AStruct::do_more(self,10) }; }; ^( :: means associated )^
> let A = AStruct { f1:1, f2:2 }; A.raise_it(); ^( f1 has now the value 11 )^";

pub fn learn_implementation(show_all: bool) {
    if show_all {
        print_title(TITLE);
        print_md_txt(EXP_TEXT);
    } else {
        println!("{}", TITLE);
    }
}

struct Car {
    owner: String,
    year: u32,
    price: u32,
}

pub fn test_implementation() {
    let mut my_car = Car {
        owner: String::from("ABC"),
        year: 2010,
        price: 5_000,
    };
    my_car.price = 6_000;
    my_car.raise_price(2_000);
    my_car.display_car_info();
    let mut new_car = my_car.sell_car();
    new_car.raise_auto();
    new_car.display_car_info();
}

impl Car {
    fn display_car_info(&self) {
        // car: &Car changed to &self
        println!(
            "owner: {}, Year: {}, Price: {}",
            self.owner, self.year, self.price
        );
    }
    // It is possible to send arguments and even change the value of self
    fn raise_price(&mut self, dollars: u32) {
        // &mut self makes self mutable
        self.price += dollars;
    }
    // A method can also take and transmit the ownership
    fn sell_car(self) -> Self {
        self
    }

    fn raise_auto(&mut self) {
        Car::raise_price(self, 1000); // calling associated functions
    }
}
