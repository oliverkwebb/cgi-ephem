//! The functions for generating ASCII art for data
//!
//! Each function takes a line number (0-11) and input data, and outputs a vector of atoms that is 55 characters long when rendered
use crate::query;
use crate::text;
use iau_constellations;
use pracstro::time;

const EMPTY_LINE: &str = "|                                                     |";
const TILE_FOOTER: &str = "+-----------------------------------------------------+";

pub fn location_tile(data: query::CGIData, line: usize) -> Vec<text::TextAtom> {
    let coords_1875 = data
        .location
        .precess(
            time::Date::now(),
            time::Date::from_calendar(1875, 0, 0, time::Angle::default()),
        )
        .equatorial();
    let constellation = iau_constellations::CONSTELLATION_NAMES[iau_constellations::constell_1875(
        coords_1875.0.degrees(),
        coords_1875.1.to_latitude().degrees(),
    )];
    match line {
        0 => vec![text::TextAtom {
            content: format!("+{:-^53}+", " Location "),
            special_formatting: None,
        }],
        3 => vec![text::TextAtom {
            content: format!(
                "|{:^53}|",
                format!("In the constellation {}", constellation)
            ),
            special_formatting: None,
        }],
        14 => vec![text::TextAtom {
            content: TILE_FOOTER.into(),
            special_formatting: None,
        }],
        1..14 => vec![text::TextAtom {
            content: EMPTY_LINE.into(),
            special_formatting: None,
        }],
        _ => unreachable!(),
    }
}
