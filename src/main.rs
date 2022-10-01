use std::fmt;
use std::ops::Add;

#[derive(Debug)]
struct Date {
    days: i32,
}

impl Date {
    // create Date from year/month/day triple
    fn from_ymd(year: i32, month: i32, day: i32) -> Date {
        return Date { days: 0 };
    }
    // convert back to year/month/day triple
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
    // testing from_ymd; should print Date { days: 738885 }

    println!("{:?}", Date::from_ymd(2022, 12, 31));
    // testing Add and Display

    let date = Date::from_ymd(-2, 12, 31);
    // increase date by 30 days, 60 days, etc.

    for i in 0..20 {
        // first iteration should print 2/12/31 BC

        println!("{}", (date.days + i * 30)); // added date.days + i * 30 instead of date + i * 30
    }
}


