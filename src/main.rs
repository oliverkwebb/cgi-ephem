use pracstro::time;
use std::{env, path};
use value::*;

/// Handles the reading and querying of the catalog of celestial objects
pub mod catalog;
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
    // If there is no USER_AGENT environment variable, assume we are running in a CLI
    let driver = if env::var("HTTP_USER_AGENT")
        .unwrap_or("curl".into())
        .starts_with("curl")
    {
        text::ANSI_DRIVER
    } else {
        text::HTML_DRIVER
    };

    let date = time::Date::now();
    let obj = if let Ok(query_string) = env::var("PATH_INFO") {
        parse::object(query_string.strip_prefix("/").unwrap(), &catalog::read()).unwrap()
    } else {
        CelObj::Moon
    };

    let data = query::generate_cgi_data(&obj, date);

    print!("{}", driver.header);

    let location_tile: Vec<String> = (0..=14)
        .map(|x| {
            tiles::location_tile(data.location, x, date)
                .into_iter()
                .map(driver.render_atom)
                .collect::<String>()
        })
        .collect();

    let phase_tile: Vec<String> = if let Some(phaseangle) = data.phaseangle {
        (0..=14)
            .map(|x| {
                tiles::phase_tile(phaseangle, x, &obj)
                    .into_iter()
                    .map(driver.render_atom)
                    .collect::<String>()
            })
            .collect()
    } else {
        (0..=14)
            .map(|x| {
                tiles::na_nostart_tile(x, " Phase ".into())
                    .into_iter()
                    .map(driver.render_atom)
                    .collect::<String>()
            })
            .collect()
    };

    let brightness_tile: Vec<String> = (0..=14)
        .map(|x| {
            tiles::brightness_tile(data.brightness, x)
                .into_iter()
                .map(driver.render_atom)
                .collect::<String>()
        })
        .collect();

    for i in 0..=14 {
        print!("{}{}{}", location_tile[i], phase_tile[i], driver.eol);
    }
    for i in 0..=14 {
        print!("{}{}", brightness_tile[i], driver.eol);
    }

    print!("{}", driver.footer);
}
