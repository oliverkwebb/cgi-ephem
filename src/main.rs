use pracstro::{
    sol::{JUPITER, MARS},
    time,
};
use value::*;

use crate::{query::generate_cgi_data, tiles::is_phase_possible};

/// Handles the reading and querying of the catalog of celestial objects
pub mod catalog;
pub mod output;
pub mod parse;
pub mod query;
pub mod text;
pub mod tiles;
pub mod value;

/// pracstro provides a way to do this, but that isn't functional in a lot of contexts
///
/// Used in ephemeris generation and date reading
pub mod timestep {
    use chrono::prelude::*;
    use pracstro::time;
    #[derive(Copy, Clone, Debug, PartialEq)]
    /// Most things can be represented as seconds or months
    /// * 1 second: 1 second
    /// * 1 minute: 60 seconds
    /// * 1 hour: 3600 seconds
    /// * 1 day: 86400 seconds
    /// * 1 week: 604800 seconds
    /// * 1 month: 1 month
    /// * 1 year: 12 months
    pub enum Step {
        S(f64),
        M(chrono::Months),
    }
    pub fn step_forward_date(d: time::Date, s: Step) -> time::Date {
        match s {
            Step::S(sec) => time::Date::from_julian(d.julian() + (sec.abs() / 86400.0)),
            Step::M(m) => time::Date::from_unix(
                (DateTime::from_timestamp(d.unix() as i64, 0).unwrap() + m).timestamp() as f64,
            ),
        }
    }
    pub fn step_back_date(d: time::Date, s: Step) -> time::Date {
        match s {
            Step::S(sec) => time::Date::from_julian(d.julian() - (sec.abs() / 86400.0)),
            Step::M(m) => time::Date::from_unix(
                (DateTime::from_timestamp(d.unix() as i64, 0).unwrap() - m).timestamp() as f64,
            ),
        }
    }
    pub struct EphemIter {
        now: time::Date,
        step: Step,
        end: time::Date,
    }
    impl EphemIter {
        pub fn new(start: time::Date, step: Step, end: time::Date) -> EphemIter {
            EphemIter {
                now: start,
                step,
                end,
            }
        }
    }
    impl Iterator for EphemIter {
        type Item = time::Date;

        fn next(&mut self) -> Option<Self::Item> {
            if self.now.julian() < self.end.julian() {
                let s = self.now;
                self.now = step_forward_date(self.now, self.step);
                Some(s)
            } else {
                None
            }
        }
    }
}

fn main() {
    let driver = text::ANSI_DRIVER;

    let date = time::Date::now();

    let obj = CelObj::Moon;

    let data = generate_cgi_data(obj.clone(), date);

    print!("{}", driver.header);

    (0..=14).for_each(|x| {
        tiles::location_tile(data, x, date)
            .into_iter()
            .for_each(|y| print!("{}", (driver.render_atom)(y)));
        print!("{}", driver.eol);
    });

    if (is_phase_possible(obj.clone())) {
        (0..=14).for_each(|x| {
            tiles::phase_tile(data, x, date, &obj)
                .into_iter()
                .for_each(|y| print!("{}", (driver.render_atom)(y)));
            print!("{}", driver.eol);
        });
    }

    print!("{}", driver.footer);
}
