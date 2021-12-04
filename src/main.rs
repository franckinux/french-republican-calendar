// use fltk::{app, enums::{FrameType, Shortcut}, frame::Frame, menu::{MenuBar, MenuFlag}, prelude::*, window::Window};
// use fltk::{app, enums::{FrameType, Shortcut}, frame::Frame, prelude::*, window::Window};
use fltk::{
    app,
    enums::{CallbackTrigger, Color, FrameType,},
    frame::Frame,
    group::{Pack, PackType},
    input::IntInput,
    menu::Choice,
    prelude::*,
    window::Window
};

use french_republican_calendar::{french_calendar, gregorian_calendar};


#[derive(Debug, Clone, Copy)]
enum Message {
	Day,
	Month,
	Year
}


fn print_gregorian_date(frame: &mut Frame, fd: i32, fm: i32, fy: i32) -> bool {
    let french_date = french_calendar::FrenchDate{day: fd, month: fm, year: fy};
    let sdn = french_calendar::french_to_sdn(&french_date);
    let gregorian_date = gregorian_calendar::sdn_to_gregorian(sdn);
    if gregorian_date.day == 0 {
        frame.set_label("");
        false
    } else {
        frame.set_label(
            format!(
                "{} {} {:4}",
                gregorian_date.day,
                gregorian_calendar::LONG_MONTH_NAME[gregorian_date.month as usize],
                gregorian_date.year
            ).as_str()
        );
        true
    }
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
	let mut day = 1;
	let mut month = 1;
	let mut year = 1;

    let app = app::App::default().with_scheme(app::Scheme::Gleam);
    let mut win = Window::default()
        .with_size(400, 200)
        .center_screen()
        .with_label("French revolutionary calendar converter");

    let mut hpack = Pack::new(60, 50, 200, 25, "French Revolutionary");

	let mut day_input = IntInput::default().with_size(40, 0);
    day_input.set_value("1");
    day_input.set_trigger(CallbackTrigger::Changed);

    let mut choice = Choice::default().with_size(150, 0);
    for &m in french_calendar::FRENCH_MONTH_NAME.iter() {
        choice.add_choice(m);
    }
    choice.end();
    choice.set_value(0);

	let mut year_input = IntInput::default().with_size(60, 0);
    year_input.set_value("1");
    year_input.set_trigger(CallbackTrigger::Changed);

    hpack.end();
    hpack.set_type(PackType::Horizontal);
    hpack.set_spacing(10);

    let mut hpack2 = Pack::new(90, 130, 200, 25, "Gregorian");
	let mut frame = Frame::default().with_size(0, 25);
    hpack.end();

    win.make_resizable(false);
    win.end();
    win.show();

    let (s, r) = app::channel::<Message>();

    choice.emit(s, Message::Month);
    day_input.emit(s, Message::Day);
    year_input.emit(s, Message::Year);

    while app.wait() {
        if let Some(msg) = r.recv() {
            match msg {
                Message::Day => {
                    match day_input.value().parse() {
                        Ok(d) => {
                            day = d;
                            print_gregorian_date(&mut frame, day, month, year);
                        },
                        Err(_) => {
                            frame.set_label("");
                        }
                    }
                }
                Message::Month => {
                    month = choice.value() + 1;
                    print_gregorian_date(&mut frame, day, month, year);
                }
                Message::Year => {
                    match year_input.value().parse() {
                        Ok(y) => {
                            year = y;
                            print_gregorian_date(&mut frame, day, month, year);
                        }
                        Err(_) => {
                            frame.set_label("");
                            continue;
                        }
                    }
                    print_gregorian_date(&mut frame, day, month, year);
                },
            }
        }
    }

	Ok(())
}
