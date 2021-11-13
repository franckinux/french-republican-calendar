use repub_cli::gregorian_calendar;

/* $selId: vgregor.c,v 2.0 1995/10/24 01:13:06 lees Exp $
 * Copyright 1993-1995, Scott E. Lee, all rights reserved.
 * Permission granted to use, copy, modify, distribute and sell so long as
 * the above copyright and this permission statement are retained in all
 * copies.  THERE IS NO WARRANTY - USE AT YOUR OWN RISK.
 */

fn feb_length(year: i32) -> i32 {
    let mut y = year;
    if y < 0 {
        y += 1;
    }
    y += 4800;

    if y % 4 == 0 {
        if y % 100 == 0 {
            if y % 400 == 0 {
                29
            } else {
                28
            }
        } else {
            29
        }
    } else {
        28
    }
}

pub fn test_gregorian_dates() -> i32 {
    let mut month_length: [i32; 12] = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    println!("Verifying all Gregorian calendar dates from the year {} to {}\n", -4714, 10000);

    let mut year = -4714;
    let mut month = 11;
    let mut day = 25;
    let mut sdn = 1;
    month_length[1] = feb_length(year);
    let mut n_errors = 0;

    loop {
        let g_date2 = gregorian_calendar::GregorianDate{day, month, year};
        let sdn2 = gregorian_calendar::gregorian_to_sdn(&g_date2);

        let g_date = gregorian_calendar::sdn_to_gregorian(sdn);

        if sdn2 != sdn || g_date != g_date2 {
            n_errors += 1;
            if n_errors <= 10 {
                println!("{} {:?}   erroneous: {} {:?}", sdn, g_date, sdn2, g_date2);
            }
        }

        sdn += 1;
        day += 1;
        if day > month_length[month as usize - 1] {
            day = 1;
            month += 1;
            if month > 12 {
                month = 1;
                year += 1;
                if year == 0 {
                    year += 1;
                }
                if year % 500 == 0 {
                    if year >= 0 {
                        println!("{} A.D.", year);
                    } else {
                        println!("{} B.C.", -year);
                    }
                }
                if year > 10000 {
                    break;
                }
                month_length[1] = feb_length(year);
            }
        }
    }

    println!("Total number of errors found: {}", n_errors);
    n_errors
}
