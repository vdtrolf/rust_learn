use rust_learn::mod_utils::print_md_txt;
use rust_learn::mod_utils::print_title;

static TITLE: &str = "11-Enums";
static EXP_TEXT: &str = "1. An enum is a list of values :
> enum WeekendDays {
>   Saturday,
>   Sunday,
> };
> let a_day = WeekendDays::Saturday
2. Enums can be implemented:
> impl WeekendDays {
>   fn compensation_hours(&self, hours : f32) => f32 {
>     return match self {
>       WeekendDays::Saturday => hours * 1.5,
>       WeekendDays::Sunday => hours * 2,
> } } }
> let comp = a_day.compensation_hours(3);
3. It is also possible to give a type to an enum value:
> enum Grades {
>   low(u8),
>   medium(u8),
> }
4. With an impl which doesn't need a parameter anymore:
> let a_grade = grades::low(3);
> impl Grades {
>   fn recalc_grades(&self) -> u8 {
>     return match self {
>       Grades::Low(grade_val) => grade_val + 2,  ^(grade_val will hold the actual grade value)^
>       Grades::Medium(grade_val) => grade_val + 3,
> } } }
> let tot = a_grade.recalc();";

pub fn learn_enums(show_all: bool) {
    if show_all {
        print_title(TITLE);
        print_md_txt(EXP_TEXT);
    } else {
        println!("{}", TITLE);
    }
}

#[derive(Debug)]
enum WeekendDays {
    Saturday,
    _Sunday,
}

impl WeekendDays {
    fn compensation_hours(&self, hours: f32) -> f32 {
        return match self {
            WeekendDays::Saturday => hours * 1.5,
            WeekendDays::_Sunday => hours * 2.0,
        };
    }
}

enum Grades {
    Low(u8),
    _Medium(u8),
}

impl Grades {
    fn recalc_grades(&self) -> u8 {
        return match self {
            Grades::Low(grade_val) => grade_val + 2,
            Grades::_Medium(grade_val) => grade_val + 3,
        };
    }
}

pub fn test_enums() {
    let a_day = WeekendDays::Saturday;

    println!("1. a_day is '{:?}'", a_day);

    let hours: u8 = 3;
    let comp: f32 = a_day.compensation_hours(hours.into());

    println!("2. hours : '{}' comp : '{}'", hours, comp);

    let a_grade = Grades::Low(3);
    let tot = a_grade.recalc_grades();
    println!("4. Low grade 3, tot : '{}'", tot);
}
