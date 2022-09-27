use std::fmt;
use std::ops::Add;

#[derive(Debug)]
struct Date {
    days: i32,
}

impl Date {
    // create date from year/month/date format
    fn from_ymd(year: i32, month: i32, day: i32) -> Date {
        return Date { days: 0 };
    }

    fn ymd(&self) -> (i32, i32, i32) {
        return (0, 0, 0);
    }
}


impl Add for Date {
    type Output = Date;

    fn add(self, other: Date) -> Date {
        Date {
            days: self.days + other.days
        }
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if Date::ymd(&self).0 < 0 {
            write!(f, "{}/{}/{} BC", Date::ymd(&self).0, Date::ymd(&self).1, Date::ymd(&self).2)
        } else {
            write!(f, "{}/{}/{} ", Date::ymd(&self).0, Date::ymd(&self).1, Date::ymd(&self).2)
        }
    }
}


fn main() {
    println!("{:?}", Date::from_ymd(2022, 12, 31));
    let date = Date::from_ymd(-2, 12, 31);

    for i in 0..20 {
        println!("{}", date + i * 30);
    }
}


