use std::io;
use std::io::Write;

use repub_cli::{french_calendar, gregorian_calendar};

fn read_value(entity: &str) -> i32 {
    loop {
        let mut value = String::new();
        print!("{} : ", entity);
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut value).expect("Failed to read line");
        match value.trim().parse() {
            Ok(val) => return val,
            Err(error) => println!("This is not a correct number ({})", error),
        }
    }
}

fn main() {
    loop {
        println!( "\nEntrée de la date révolutionnaire");
        println!( "---------------------------------");
        println!( "Mois révolutionnaires:");
        for (i, &m) in french_calendar::FRENCH_MONTH_NAME.iter().enumerate()  {
            if i == 0 { continue; }
            print!("{:2} : {:<20}", i, m);
            if i % 3 == 0 {
                println!("");
                io::stdout().flush().unwrap();
            }
        }
        println!("");
        let day = read_value("Jour");
        let month = read_value("Mois");
        let year = read_value("Annee");

        let sdn = french_calendar::french_to_sdn(&french_calendar::FrenchDate{day, month, year});
        let gregorian_date = gregorian_calendar::sdn_to_gregorian(sdn);

        println!(
            "Date au calendrier révolutionnaire : {} {} an {}",
            day,
            french_calendar::FRENCH_MONTH_NAME[month as usize],
            year
        );
        println!(
            "Date au calendrier grégorien : {} {} {:4}",
            gregorian_date.day,
            gregorian_calendar::LONG_MONTH_NAME[gregorian_date.month as usize],
            gregorian_date.year
        );
    }
}
