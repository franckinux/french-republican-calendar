use repub_cli::gregorian_calendar;

/* $selId: vgregor.c,v 2.0 1995/10/24 01:13:06 lees Exp $
 * Copyright 1993-1995, Scott E. Lee, all rights reserved.
 * Permission granted to use, copy, modify, distribute and sell so long as
 * the above copyright and this permission statement are retained in all
 * copies.  THERE IS NO WARRANTY - USE AT YOUR OWN RISK.
 */

#[test]
pub fn test_gregorian_dates() {
    println!("Verifying all Gregorian calendar dates from the year {} to {}", -4714, 10000);

    let mut sdn = 1;
    let mut g_date2 = gregorian_calendar::GregorianDate{day: 25, month: 11, year: -4714};

    loop {
        let sdn2 = gregorian_calendar::gregorian_to_sdn(&g_date2);
        let g_date = gregorian_calendar::sdn_to_gregorian(sdn);
        assert!(sdn2 == sdn && g_date == g_date2, "{} {:?}   erroneous: {} {:?}", sdn, g_date, sdn2, g_date2);

        sdn += 1;
        g_date2.next_day();
        if g_date2.year > 10000 {
            break;
        }
    }
}
