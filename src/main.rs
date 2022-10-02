use std::fmt;
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
struct Date {
    days: i32,
}

impl Date {
    // create Date from year/month/day triple
    fn from_ymd(year: i32, month: i32, day: i32) -> Date {
        let x: i32 = if is_leapyear(year) { 29 } else { 28 };
        let month_days: [i32; 12] = [31, x, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut days_count: i32 = 0;

        if year >= 0 {
            for i in 0..(year) {
                if is_leapyear(i) {
                    days_count += 366;
                } else {
                    days_count += 365
                }
            }
            for i in 0..(month - 1) {
                days_count += month_days[i as usize];
            }

            days_count += day - 1;
        } else {
            for i in year + 1..0 {
                if is_leapyear(i) {
                    days_count += 366;
                } else {
                    days_count += 365
                }
            }

            for i in month..12 {
                days_count += month_days[i as usize]
            }

            days_count += (month_days[(month - 1) as usize] - day) + 1;

            days_count *= -1;
        }
        return Date { days: days_count };
    }
    // convert back to year/month/day triple
    fn ymd(&self) -> (i32, i32, i32) {
        let mut days: i32 = self.days;
        let mut year: i32 = 0;
        let mut month: i32 = 0;
        let mut day: i32 = 1;

        if days >= 0 {
            month = 1;
            while days >= 365 {
                if is_leapyear(year) {
                    days -= 366;
                } else {
                    days -= 365;
                }
                year += 1;
            }

            let x: i32 = if is_leapyear(year) { 29 } else { 28 };
            let month_days: [i32; 12] = [31, x, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
            let mut i: i32 = 0;
            while days >= month_days[i as usize] {
                days -= month_days[i as usize];
                month += 1;
                i += 1;
            }
            day += days;
        } else {
            year = -1;
            month = 12;
            days *= -1;
            while days >= 365 {
                year -= 1;
                if is_leapyear(year) {
                    days -= 366;
                } else {
                    days -= 365;
                }
            }
            let x: i32 = if is_leapyear(year) { 29 } else { 28 };
            let month_days: [i32; 12] = [31, x, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
            let mut i: i32 = 11;

            while days > month_days[i as usize] {
                days -= month_days[i as usize];
                i -= 1;
            }
            month = i +1;
            day = month_days[i as usize] - days +1
        }
        return (year, month, day);
    }
}

fn is_leapyear(year: i32) -> bool {
    return if year % 400 == 0 {
        true
    } else if year % 100 == 0 {
        false
    } else if year % 4 == 0 {
        true
    } else {
        false
    };
}

impl Add<i32> for Date {
    type Output = Self;

    fn add(self, to_add: i32) -> Self {
        Self {
            days: self.days + to_add
        }
    }
}

impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if Date::ymd(&self).0 < 0 {
            write!(f, "{}/{}/{} BC", (Date::ymd(&self).0 * -1), Date::ymd(&self).1, Date::ymd(&self).2)
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
        println!("{}", date + i * 30);
    }
}


