use crate::mod_utils::{print_md_txt, print_title};

static TITLE: &str = "12-Options";
static EXP_TEXT: &str = "The option enum is there to handle cases where there is possibly no value,
like null in other languages (Rust doen´t allow null)
## 1. The option is defined by Option<type>, like in:
> struct Student {
>   name: String,
>   grade: Option<u32>,
> }
It is then possible to create a Student instance with 'None' or 'Some(^value^)' :
> let a_student = Student {
>   name: String::from(\"Mathias\"),
>   grade: Some(10),
> }
It can be tested with a match on the option value:
> match a_student. is grade {
>   None => println!(\"1. {:?} has no grade}\", a_student.name),
>   Some(grade) => println!(\"1. {:?} has {grade}\", a_student.name),
> }
## 2. With 'grade: None', the code would have returned 'Matthias has no code'
## 3. Similarly, it is possible to test a single possibility with if let
> if let Some(grade) = c_student.grade {
>   println!(\"3. {:?} has grade {grade}\", c_student.name),
> }";

pub fn learn_options(show_all: bool) {
    if show_all {
        print_title(TITLE);
        print_md_txt(EXP_TEXT);
    } else {
        println!("{}", TITLE);
    }
}

struct Student {
    name: String,
    grade: Option<u32>,
}

pub fn test_options() {
    let a_student = Student {
        name: String::from("Mathias"),
        grade: Some(10),
    };
    test_grade("1", a_student);
    let b_student = Student {
        name: String::from("Mathias"),
        grade: None,
    };
    test_grade("2", b_student);

    let c_student = Student {
        name: String::from("Henri"),
        grade: Some(10),
    };
    if let Some(grade) = c_student.grade {
        println!("3. {:?} has grade {grade}", c_student.name);
    }
}

fn test_grade(test_num: &str, student: Student) {
    match student.grade {
        None => println!("{}. {:?} has no grade", test_num, student.name),
        Some(grade) => println!("{}. {:?} has {grade}", test_num, student.name),
    }
}
