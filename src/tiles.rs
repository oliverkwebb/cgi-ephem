//! The functions for generating ASCII art for data
//!
//! Each function takes a line number (0-11) and input data, and outputs a vector of atoms that is 55 characters long when rendered
use std::f64;

use crate::query;
use crate::query::CGIData;
use crate::text::TextFormatting;
use crate::text::{self, TextAtom};
use crate::text::{ANSIColors, Color};
use crate::value::CelObj;
use crate::value::CrdView;
use iau_constellations;
use pracstro::coord::Coord;
use pracstro::time::{self, Date};

const EMPTY_LINE: &str = "|                                                     |";
const TILE_FOOTER: &str = "+-----------------------------------------------------+";

/// Is it possibele to generate the phase tile
pub fn is_phase_possible(obj: CelObj) -> bool {
    match obj {
        CelObj::Moon | CelObj::Planet(_) => true,
        CelObj::Crd(_) | CelObj::Sun | CelObj::Star(_) => false,
    }
}

pub fn color_of(obj: CelObj) -> Color {
    match obj {
        CelObj::Moon => Color(ANSIColors::White, false),
        CelObj::Planet(p) => match p.name {
            "Venus" => Color(ANSIColors::White, true),
            "Mars" => Color(ANSIColors::Red, false),
            "Jupiter" | "Saturn" => Color(ANSIColors::Yellow, false),
            "Uranus" => Color(ANSIColors::Cyan, true),
            "Neptune" => Color(ANSIColors::Blue, false),
            "Mercury" => Color(ANSIColors::White, false),
            _ => unreachable!(),
        },
        _ => unreachable!(),
    }
}

/// Generates a slice of a crecent at a phase angle
///
/// Loose adaptation from phoon's source code at <https://www.acme.com/software/phoon/>
pub fn crecent_slice(phaseangle: f64, line: usize, aspect_ratio: f64, numlines: usize) -> String {
    let mcap = -(phaseangle.cos());
    let yrad = (numlines as f64) / 2.0;
    let xrad = yrad / aspect_ratio;

    // Compute Edges of Slice
    let y = (line as f64) + 0.5 - yrad;
    let mut xright = xrad * (1.0 - (y * y) / (yrad * yrad)).sqrt();
    let mut xleft = -xright;

    if phaseangle >= 0.0 && phaseangle < f64::consts::PI {
        xleft = mcap * xleft;
    } else {
        xright = mcap * xright;
    }

    let colleft = ((xrad + 0.5) as isize) + ((xleft + 0.5) as isize);
    let colright = ((xrad + 0.5) as isize) + ((xright + 0.5) as isize);

    let mut result = String::new();

    (0..(colleft as usize)).for_each(|_| result.push(' '));
    ((colleft as usize)..=(colright as usize)).for_each(|_| result.push('@'));

    result
}

pub fn phase_tile(data: CGIData, line: usize, date: Date, obj: &CelObj) -> Vec<TextAtom> {
    let phaseangle = data.phaseangle.unwrap();
    match line {
        0 => vec![text::TextAtom {
            content: format!("+{:-^53}+", " Phase "),
            special_formatting: None,
        }],
        7 => vec![
            text::TextAtom {
                content: "|".into(),
                special_formatting: None,
            },
            text::TextAtom {
                content: format!("{:<23}", crecent_slice(phaseangle, line - 2, 0.5, 10)),
                special_formatting: Some(TextFormatting {
                    bold: true,
                    italic: false,
                    underline: false,
                    color: Some(color_of(obj.clone())),
                    bgcolor: None,
                }),
            },
            text::TextAtom {
                content: format!(
                    "{:^30}",
                    crate::value::Value::Phase(
                        pracstro::time::Angle::from_radians(phaseangle),
                        crate::value::PhaseView::Default(true)
                    )
                    .to_string()
                ),
                special_formatting: Some(TextFormatting {
                    bold: true,
                    italic: false,
                    underline: false,
                    color: None,
                    bgcolor: None,
                }),
            },
            text::TextAtom {
                content: "|".into(),
                special_formatting: None,
            },
        ],
        2..12 => vec![
            text::TextAtom {
                content: "|".into(),
                special_formatting: None,
            },
            text::TextAtom {
                content: format!("{:<53}", crecent_slice(phaseangle, line - 2, 0.5, 10)),
                special_formatting: Some(TextFormatting {
                    bold: true,
                    italic: false,
                    underline: false,
                    color: Some(color_of(obj.clone())),
                    bgcolor: None,
                }),
            },
            text::TextAtom {
                content: "|".into(),
                special_formatting: None,
            },
        ],
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

pub fn zodiac_of(coord: Coord, date: time::Date) -> &'static str {
    [
        "Aries",
        "Taurus",
        "Gemini",
        "Cancer",
        "Leo",
        "Virgo",
        "Libra",
        "Scorpio",
        "Sagittarius",
        "Capricorn",
        "Aquarius",
        "Pisces",
    ][(coord.ecliptic(date).0.degrees() / 30.0) as usize]
}

pub fn location_tile(data: query::CGIData, line: usize, date: time::Date) -> Vec<text::TextAtom> {
    let coords_1875 = data
        .location
        .precess(
            date,
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
        3 => vec![
            text::TextAtom {
                content: format!("|{:>31}", "In the Constellation"),
                special_formatting: None,
            },
            text::TextAtom {
                content: format!(" {:<21}", constellation),
                special_formatting: Some(TextFormatting {
                    bold: true,
                    color: None,
                    bgcolor: None,
                    italic: false,
                    underline: false,
                }),
            },
            text::TextAtom {
                content: "|".into(),
                special_formatting: None,
            },
        ],
        4 => vec![
            text::TextAtom {
                content: "|".into(),
                special_formatting: None,
            },
            text::TextAtom {
                content: format!("{:>25}", "Zodiac:"),
                special_formatting: Some(TextFormatting {
                    bold: false,
                    color: Some(Color(ANSIColors::Magenta, true)),
                    bgcolor: None,
                    italic: false,
                    underline: false,
                }),
            },
            text::TextAtom {
                content: format!(" {:<27}", zodiac_of(data.location, date)),
                special_formatting: Some(TextFormatting {
                    bold: true,
                    color: Some(Color(ANSIColors::Magenta, true)),
                    bgcolor: None,
                    italic: false,
                    underline: false,
                }),
            },
            text::TextAtom {
                content: "|".into(),
                special_formatting: None,
            },
        ],
        7 => vec![text::TextAtom {
            content: format!("|{:^53}|", "Coordinates (Equatorial):"),
            special_formatting: None,
        }],
        8 => vec![text::TextAtom {
            content: format!(
                "|{:^53}|",
                crate::value::Value::Crd(data.location, CrdView::Equatorial).to_string()
            ),
            special_formatting: None,
        }],
        10 => vec![text::TextAtom {
            content: format!("|{:^53}|", "Coordinates (Ecliptic):"),
            special_formatting: None,
        }],
        11 => vec![text::TextAtom {
            content: format!(
                "|{:^53}|",
                crate::value::Value::Crd(data.location, CrdView::Ecliptic(date)).to_string()
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
