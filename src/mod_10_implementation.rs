use crate::mod_utils::{print_md_txt, print_title};

static TITLE: &str = "10-Implementation";
static EXP_TEXT: &str =
"An implementation block allows to reference to a certain type, such as a struct.
A function in such a block is called a method and doesn't expect any argument (self is implicit):
> struct Car { owner: String,price: u32,}
> impl Car {
>   fn display_car_info(&self) {
>     println!(\"owner: {}, Price: {}\",self.owner, self.price); ^(car: &Car changed to &self)^
>   }
It is possible to **send arguments** and even change the value of self:
>   fn raise_price(&mut self, dollars: u32) {
>     self.price += dollars;  ^(&mut self makes self mutable )^
>   }
**A method** can also take and transmit the ownership
>   fn sell_car(self) -> Self {
>     self
>   }
**An associated function** is a function part of the same implementation - can be called with Struct:: :
>  fn raise_auto(&mut self) {
>     Car::raise_price(self, 1000); // calling associated functions
>   }
> }";

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
    price: u32,
}

impl Car {
    fn display_car_info(&self) {
        // car: &Car changed to &self
        println!("--> Owner: {},Price: {}", self.owner, self.price);
    }
    // It is possible to send arguments and even change the value of self
    fn raise_price(&mut self, dollars: u32) {
        // &mut self makes self mutable
        self.price += dollars;
    }
    // A method can also take and transmit the ownership0
    fn sell_car(self) -> Self {
        self
    }
    // A method can call another method
    fn raise_auto(&mut self) {
        Car::raise_price(self, 1000); // calling associated functions
    }
}

pub fn test_implementation() {
    let mut my_car = Car {
        owner: String::from("ABC"),
        price: 100,
    };
    println!("Original price is 100");
    my_car.display_car_info();
    my_car.price = 6_000;
    println!("New price is 6000");
    my_car.display_car_info();
    my_car.raise_price(2_000);
    println!("Raised by 2000");
    my_car.display_car_info();
    let mut new_car = my_car.sell_car();
    new_car.raise_auto();
    println!("Made mutable after sell_car and auto raise");
    new_car.display_car_info();
}
