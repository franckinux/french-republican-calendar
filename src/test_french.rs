use repub_cli::{french_calendar, gregorian_calendar};

/* $selId: vfrench.c,v 2.0 1995/10/24 01:13:06 lees Exp $
 * Copyright 1993-1995, Scott E. Lee, all rights reserved.
 * Permission granted to use, copy, modify, distribute and sell so long as
 * the above copyright and this permission statement are retained in all
 * copies.  THERE IS NO WARRANTY - USE AT YOUR OWN RISK.
 */

pub fn test_french_dates() -> i32 {
    const YEAR_START: [i32; 14]  = [
        22, 22, 22, 23, 22, 22, 22, 23, 23, 23, 23, 24, 23, 23,
    ];

    let mut n_errors = 0;

    println!("Verifying the French calendar with 14 known dates");

    for (i, &year_start) in YEAR_START.iter().enumerate() {
        let ii: i32 = i as i32;
        let f_date = french_calendar::FrenchDate{year: ii+1, month: 1, day: 1};
        let f_sdn = french_calendar::french_to_sdn(&f_date);

        let g_date = gregorian_calendar::GregorianDate{year: ii+1792, month: 9, day: year_start};
        let g_sdn = gregorian_calendar::gregorian_to_sdn(&g_date);

        if f_sdn != g_sdn {
            n_errors += 1;
            println!(
                "error: {:?}={} != {:?}={}", f_date, f_sdn, g_date, g_sdn
            );
        }
    }

    println!("Verifying all French republican calendar dates from the year {} to {}", 1, 14);

    let mut year = 1;
    let mut month = 1;
    let mut day = 1;
    let mut sdn = 2375840;

    loop {
        let f_date2 = french_calendar::FrenchDate{day, month, year};
        let sdn2 = french_calendar::french_to_sdn(&f_date2);

        let f_date = french_calendar::sdn_to_french(sdn);

        if sdn2 != sdn || f_date != f_date2 {
            n_errors += 1;
            if n_errors <= 1000 {
                println!("{} {:?}   erroneous: {} {:?}", sdn, f_date, sdn2, f_date2);
            }
        }

        sdn += 1;
        day += 1;
        if day > 30 {
            day = 1;
            month += 1;
        }
        if month == 13 && day > 5 + if (year + 1) % 4 == 0 {1} else {0} {
            day = 1;
            month = 1;
            year += 1;
        }
        if year > 14 {
            break;
        }
    }

    println!("Total number of errors found: {}", n_errors);
    n_errors
}
