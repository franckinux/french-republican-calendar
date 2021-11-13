/*
 * This library is based on Scott E. Lee's work which is not more available.
 *
 * Copyright 1993-1995, Scott E. Lee, all rights reserved.
 * Permission granted to use, copy, modify, distribute and sell so long as
 * the above copyright and this permission statement are retained in all
 * copies.  THERE IS NO WARRANTY - USE AT YOUR OWN RISK.
 */

/**************************************************************************
 *
 * These are the externally visible components of this file:
 *
 *     pub fn sdn_to_gregorian(sdn: i32) -> GregorianDate
 *
 * Convert a SDN to a Gregorian calendar date.  If the input SDN is less
 * than 1, the three output values will all be set to zero, otherwise
 * *pYear will be >= -4714 and != 0; *pMonth will be in the range 1 to 12
 * inclusive; *pDay will be in the range 1 to 31 inclusive.
 *
 *     pub fn gregorian_to_sdn(gregorian_date: &GregorianDate) -> i32
 *
 * Convert a Gregorian calendar date to a SDN.  Zero is returned when the
 * input date is detected as invalid or out of the supported range.  The
 * return value will be > 0 for all valid, supported dates, but there are
 * some invalid dates that will return a positive value.  To verify that a
 * date is valid, convert it to SDN and then back and compare with the
 * original.
 *
 *     pub const SHORT_MONTH_NAME: &'static [&'static str]
 *
 * Convert a Gregorian month number (1 to 12) to the abbreviated (three
 * character) name of the Gregorian month (null terminated).  An index of
 * zero will return a zero length string.
 *
 *     pub const LONG_MONTH_NAME: &'static [&'static str]
 *
 * Convert a Gregorian month number (1 to 12) to the name of the Gregorian
 * month (null terminated).  An index of zero will return a zero length
 * string.
 *
 * VALID RANGE
 *
 *     4714 B.C. to at least 10000 A.D.
 *
 *     Although this software can handle dates all the way back to 4714
 *     B.C., such use may not be meaningful.  The Gregorian calendar was
 *     not instituted until October 15, 1582 (or October 5, 1582 in the
 *     Julian calendar).  Some countries did not accept it until much
 *     later.  For example, Britain converted in 1752, The USSR in 1918 and
 *     Greece in 1923.  Most European countries used the Julian calendar
 *     prior to the Gregorian.
 *
 * CALENDAR OVERVIEW
 *
 *     The Gregorian calendar is a modified version of the Julian calendar.
 *     The only difference being the specification of leap years.  The
 *     Julian calendar specifies that every year that is a multiple of 4
 *     will be a leap year.  This leads to a year that is 365.25 days long,
 *     but the current accepted value for the tropical year is 365.242199
 *     days.
 *
 *     To correct this error in the length of the year and to bring the
 *     vernal equinox back to March 21, Pope Gregory XIII issued a papal
 *     bull declaring that Thursday October 4, 1582 would be followed by
 *     Friday October 15, 1582 and that centennial years would only be a
 *     leap year if they were a multiple of 400.  This shortened the year
 *     by 3 days per 400 years, giving a year of 365.2425 days.
 *
 *     Another recently proposed change in the leap year rule is to make
 *     years that are multiples of 4000 not a leap year, but this has never
 *     been officially accepted and this rule is not implemented in these
 *     algorithms.
 *
 * ALGORITHMS
 *
 *     The calculations are based on three different cycles: a 400 year
 *     cycle of leap years, a 4 year cycle of leap years and a 5 month
 *     cycle of month lengths.
 *
 *     The 5 month cycle is used to account for the varying lengths of
 *     months.  You will notice that the lengths alternate between 30
 *     and 31 days, except for three anomalies: both July and August
 *     have 31 days, both December and January have 31, and February
 *     is less than 30.  Starting with March, the lengths are in a
 *     cycle of 5 months (31, 30, 31, 30, 31):
 *
 *         Mar   31 days  \
 *         Apr   30 days   |
 *         May   31 days    > First cycle
 *         Jun   30 days   |
 *         Jul   31 days  /
 *
 *         Aug   31 days  \
 *         Sep   30 days   |
 *         Oct   31 days    > Second cycle
 *         Nov   30 days   |
 *         Dec   31 days  /
 *
 *         Jan   31 days  \
 *         Feb 28/9 days   |
 *                          > Third cycle (incomplete)
 *
 *     For this reason the calculations (internally) assume that the
 *     year starts with March 1.
 *
 * TESTING
 *
 *     This algorithm has been tested from the year 4714 B.C. to 10000
 *     A.D.  The source code of the verification program is included in
 *     this package.
 *
 * REFERENCES
 *
 *     Conversions Between Calendar Date and Julian Day Number by Robert J.
 *     Tantzen, Communications of the Association for Computing Machinery
 *     August 1963.  (Also published in Collected Algorithms from CACM,
 *     algorithm number 199).
 *
 **************************************************************************/

const SDN_OFFSET: i32 = 32045;
const DAYS_PER_5_MONTHS: i32 = 153;
const DAYS_PER_4_YEARS: i32 = 1461;
const DAYS_PER_400_YEARS: i32 = 146097;

#[derive(Debug, PartialEq, Eq)]
pub struct GregorianDate {
    pub day: i32,
    pub month: i32,
    pub year: i32
}

pub const SHORT_MONTH_NAME: &'static [&'static str] = &[
    "",
    "Jan",
    "Feb",
    "Mar",
    "Apr",
    "May",
    "Jun",
    "Jul",
    "Aug",
    "Sep",
    "Oct",
    "Nov",
    "Dec"
];

pub const LONG_MONTH_NAME: &'static [&'static str] = &[
    "",
    "January",
    "February",
    "March",
    "April",
    "May",
    "June",
    "July",
    "August",
    "September",
    "October",
    "November",
    "December"
];

pub fn sdn_to_gregorian(sdn: i32) -> GregorianDate {
    if sdn <= 0 {
        GregorianDate {
           day: 0,
           month: 0,
           year: 0,
        }
    } else {
        let mut temp: i32 = (sdn + SDN_OFFSET) * 4 - 1;

        /* Calculate the century (year/100). */
        let century: i32 = temp / DAYS_PER_400_YEARS;

        /* Calculate the year and day of year (1 <= day_of_year <= 366). */
        temp = ((temp % DAYS_PER_400_YEARS) / 4) * 4 + 3;
        let mut year: i32 = (century * 100) + (temp / DAYS_PER_4_YEARS);
        let day_of_year: i32 = (temp % DAYS_PER_4_YEARS) / 4 + 1;

        /* Calculate the month and day of month. */
        temp = day_of_year * 5 - 3;
        let mut month: i32 = temp / DAYS_PER_5_MONTHS;
        let day: i32 = (temp % DAYS_PER_5_MONTHS) / 5 + 1;

        /* Convert to the normal beginning of the year. */
        if month < 10 {
            month += 3;
        } else {
            year += 1;
            month -= 9;
        }

        /* Adjust to the B.C./A.D. type numbering. */
        year -= 4800;
        if year <= 0 {
            year -= 1;
        }

        GregorianDate {year, month, day}
    }
}


pub fn gregorian_to_sdn(gregorian_date: &GregorianDate) -> i32 {
    /* check for invalid dates */
    if gregorian_date.year == 0 || gregorian_date.year < -4714
        || gregorian_date.month <= 0 || gregorian_date.month > 12
        || gregorian_date.day <= 0 || gregorian_date.day > 31 {
        0
    } else {
        /* check for dates before SDN 1 (Nov 25, 4714 B.C.) */
        if gregorian_date.year == -4714
            && (gregorian_date.month < 11
                || (gregorian_date.month == 11 && gregorian_date.day < 25)
            ) {
            0
        } else {
            /* Make year always a positive number. */
            let mut year;
            if gregorian_date.year < 0 {
                year = gregorian_date.year + 4801;
            } else {
                year = gregorian_date.year + 4800;
            }

            /* Adjust the start of the year. */
            let month;
            if gregorian_date.month > 2 {
                month = gregorian_date.month - 3;
            } else {
                month = gregorian_date.month + 9;
                year -= 1;
            }

            ((year / 100) * DAYS_PER_400_YEARS) / 4
                + ((year % 100) * DAYS_PER_4_YEARS) / 4
                + (month * DAYS_PER_5_MONTHS + 2) / 5
                + gregorian_date.day
                - SDN_OFFSET
        }
    }
}

/**************************************************************************
 *
 * These are the externally visible components of this file:
 *
 *     pub fn day_of_week(sdn: i32) -> i32 {
 *
 * Convert a SDN to a day-of-week number (0 to 6).  Where 0 stands for
 * Sunday, 1 for Monday, etc. and 6 stands for Saturday.
 *
 *     pub const SHORT_DAY_NAME: &'static [&'static str] = &[
 *
 * Convert a day-of-week number (0 to 6), as returned from DayOfWeek(), to
 * the abbreviated (three character) name of the day.
 *
 *     pub const LONG_DAY_NAME: &'static [&'static str] = &[
 *
 * Convert a day-of-week number (0 to 6), as returned from DayOfWeek(), to
 * the name of the day.
 *
 **************************************************************************/

pub const SHORT_DAY_NAME: &'static [&'static str] = &[
    "Sun",
    "Mon",
    "Tue",
    "Wed",
    "Thu",
    "Fri",
    "Sat"
];

pub const LONG_DAY_NAME: &'static [&'static str] = &[
    "Sunday",
    "Monday",
    "Tuesday",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday"
];

pub fn day_of_week(sdn: i32) -> i32 {
    let dow = (sdn + 1) % 7;
    if dow >= 0 {
        dow
    } else {
        dow + 7
    }
}
