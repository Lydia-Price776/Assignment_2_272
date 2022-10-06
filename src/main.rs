use std::fmt;
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
struct Date {
    days: i32,
}

impl Date {
    // Creates Date given a year/month/day triple
    fn from_ymd(year: i32, month: i32, day: i32) -> Date {
        let month_days: [i32; 12] = [31, if is_leap_year(year) { 29 } else { 28 },
            31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let mut days_count: i32 = 0;

        //Finds the Number of Days if the year is AD
        if year >= 0 {
            for i in 0..(year) {
                days_count += find_year_days(i);
            }
            for i in 0..(month - 1) {
                days_count += month_days[i as usize];
            }
            days_count += day - 1;

            //Finds the Number of Days if the year is BC
        } else {
            for i in year + 1..0 {
                days_count += find_year_days(i);
            }
            for i in month..12 {
                days_count += month_days[i as usize]
            }
            days_count += (month_days[(month - 1) as usize] - day) + 1;
            days_count *= -1;
        }
        return Date { days: days_count };
    }


    // Converts a given number of days back to year/month/day triple
    fn ymd(&self) -> (i32, i32, i32) {
        let mut days_count: i32 = self.days;
        let mut year: i32 = 0;
        let mut month: i32;
        let mut day: i32 = 1;

        //Accounts for positive days/AD dates
        if days_count >= 0 {
            month = 1;
            while days_count >= 365 {
                days_count -= find_year_days(year);
                year += 1;
            }
            let month_days: [i32; 12] = set_month_days(year);
            let mut i: i32 = 0;

            while days_count >= month_days[i as usize] {
                days_count -= month_days[i as usize];
                month += 1;
                i += 1;
            }
            day += days_count;

            //Accounts for Negative days/BC dates
        } else {
            year = -1;
            days_count *= -1;
            while days_count >= 365 {
                year -= 1;
                days_count -= find_year_days(year);
            }
            let month_days: [i32; 12] = set_month_days(year);
            let mut i: i32 = 11;

            while days_count > month_days[i as usize] {
                days_count -= month_days[i as usize];
                i -= 1;
            }
            month = i + 1;
            day = month_days[i as usize] - days_count + 1;
        }
        return (year, month, day);
    }
}

//Returns an array of the number of days in each month dependent on year
fn set_month_days(year: i32) -> [i32; 12] {
    let x: i32 = if is_leap_year(year) { 29 } else { 28 };
    let month_days: [i32; 12] = [31, x, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    month_days
}

//Returns a boolean if a given year is a leap year
fn is_leap_year(year: i32) -> bool {
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

//Finds the number of days in a year dependent on whether the year is a leap year or not
fn find_year_days(year: i32) -> i32 {
    return if is_leap_year(year) {
        366
    } else {
        365
    };
}

//Implements the Add trait for Date
impl Add<i32> for Date {
    type Output = Self;
    fn add(self, to_add: i32) -> Self {
        Self {
            days: self.days + to_add
        }
    }
}

//Implements the Display trait for date with a formatter
impl fmt::Display for Date {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if Date::ymd(&self).0 <= 0 {
            write!(f, "{}/{}/{} BC", (Date::ymd(&self).0 * -1) + 1, Date::ymd(&self).1, Date::ymd(&self).2)
        } else {
            write!(f, "{}/{}/{} ", Date::ymd(&self).0, Date::ymd(&self).1, Date::ymd(&self).2)
        }
    }
}


fn main() {
    // testing from_ymd; should print Date { days: 738885 }
    println!("{:?}", Date::from_ymd(2022, 12, 31));

    // testing Add and Display
    let date = Date::from_ymd(-1, 12, 31);

    // increase date by 30 days, 60 days, etc.
    for i in 0..20 {
        // first iteration should print 2/12/31 BC
        println!("{}", date + i * 30);
    }
}
