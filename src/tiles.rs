//! The functions for generating ASCII art for data
//!
//! Each function takes a line number (0-11) and input data, and outputs a vector of atoms that is 55 characters long when rendered
use std::f64;

use crate::text::TextFormatting;
use crate::text::{self, TextAtom};
use crate::text::{ANSIColors, Color};
use crate::value::{CelObj, Value};

pub const EMPTY_LINE: &str = "|                                                     |";
pub const EMPTY_LINE_NOSTART: &str = "                                                     |";
pub const TILE_FOOTER: &str = "+-----------------------------------------------------+";
pub const TILE_FOOTER_NOSTART: &str = "-----------------------------------------------------+";

const ANGDIAS: [(&str, f64); 10] = [
    ("Moon (max): 34'6\"", 2046.0),
    ("Sun (max): 32'32\"", 1952.0),
    ("Sun (min): 31'27\"", 1887.0),
    ("Moon (min): 29'20\"", 1760.0),
    ("Venus (max): 1'6\"", 66.0),
    ("Human Eye Resolution: 1'0\"", 60.0),
    ("Jupiter (max): 0'50.1\"", 50.1),
    ("Mars (max): 0'25.1\"", 25.1),
    ("Uranus (max): 0'4.1\"", 4.1),
    ("Hubble Resolution: 0'0.1\"", 0.1),
];

const BRIGHTNESSES: [(&str, f64); 17] = [
    ("Sun (avg)", -26.83),
    ("Full Moon", -12.60),
    ("ISS", -5.9),
    ("Venus (max)", -4.92),
    ("Mars (max)", -2.94),
    ("Jupiter (max)", -2.94),
    ("Sirius", -1.47),
    ("Vega", 0.03),
    ("Polaris", 1.98),
    ("Human Eye Limit (Heavy Pollution)", 3.0),
    ("Andromeda Galaxy", 3.44),
    ("Human Eye Limit (Medium Pollution)", 4.0),
    ("Uranus (max)", 5.38),
    ("Human Eye Limit", 6.5),
    ("Binoculars Limit", 9.5),
    ("Pluto (max)", 13.65),
    ("Hubble Limit", 31.5),
];

pub fn na_start_tile(line: usize, title: String) -> Vec<TextAtom> {
    match line {
        0 => vec![text::TextAtom {
            content: format!("|{:-^53}+", title),
            special_formatting: None,
        }],
        7 => vec![text::TextAtom {
            content: format!("|{:^53}|", "N/A"),
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

pub fn distance_tile_noangdia(distance: f64, line: usize) -> Vec<TextAtom> {
    match line {
        0 => vec![text::TextAtom {
            content: format!("+{:-^53}+", " Distance "),
            special_formatting: None,
        }],
        2 => vec![
            TextAtom {
                content: "|".into(),
                special_formatting: None,
            },
            text::TextAtom {
                content: format!("{:^53}", format!("Distance: {:.2}", Value::Dist(distance))),
                special_formatting: Some(TextFormatting {
                    bold: true,
                    italic: false,
                    underline: false,
                    color: Some(Color(ANSIColors::Blue, true)),
                    bgcolor: None,
                }),
            },
            TextAtom {
                content: "|".into(),
                special_formatting: None,
            },
        ],
        7 => vec![TextAtom {
            content: format!("|{:^53}|", "No Angular Diameter Available"),
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

pub fn distance_tile(
    distance: f64,
    angdia: Option<pracstro::time::Angle>,
    line: usize,
) -> Vec<TextAtom> {
    if angdia.is_none() {
        return distance_tile_noangdia(distance, line);
    }
    let angdia = angdia.unwrap();
    let mut index = 0;
    let angdiaarcsec = angdia.degrees() * 3600.0;
    for i in 0..ANGDIAS.len() {
        if ANGDIAS[i].1 < angdiaarcsec {
            index = i;
            break;
        }
    }

    match line {
        0 => vec![text::TextAtom {
            content: format!("+{:-^53}+", " Distance "),
            special_formatting: None,
        }],
        2 => vec![
            TextAtom {
                content: "|".into(),
                special_formatting: None,
            },
            text::TextAtom {
                content: format!("{:^53}", format!("Distance: {:.2}", Value::Dist(distance))),
                special_formatting: Some(TextFormatting {
                    bold: true,
                    italic: false,
                    underline: false,
                    color: Some(Color(ANSIColors::Blue, true)),
                    bgcolor: None,
                }),
            },
            TextAtom {
                content: "|".into(),
                special_formatting: None,
            },
        ],
        4..7 | 8..11 => {
            let item = if index + (line - 4) >= 3 {
                ANGDIAS.get(index + (line - 4) - 3).unwrap_or(&("", 0.0))
            } else {
                &("", 0.0)
            };
            vec![
                TextAtom {
                    content: "|".into(),
                    special_formatting: None,
                },
                TextAtom {
                    content: if item.0 != "" {
                        format!("{:^53}", item.0)
                    } else {
                        format!("{:^53}", "-")
                    },
                    special_formatting: Some(TextFormatting {
                        bold: false,
                        italic: true,
                        underline: false,
                        color: Some(Color(ANSIColors::Magenta, false)),
                        bgcolor: None,
                    }),
                },
                TextAtom {
                    content: "|".into(),
                    special_formatting: None,
                },
            ]
        }
        7 => vec![
            TextAtom {
                content: "|".into(),
                special_formatting: None,
            },
            TextAtom {
                content: format!(
                    "{:^53}",
                    format!(
                        "Current Observation: {}",
                        Value::Ang(angdia, crate::value::AngView::Angle)
                    )
                ),
                special_formatting: Some(TextFormatting {
                    bold: true,
                    italic: false,
                    underline: false,
                    color: None,
                    bgcolor: None,
                }),
            },
            TextAtom {
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
pub fn na_nostart_tile(line: usize, title: String) -> Vec<TextAtom> {
    match line {
        0 => vec![text::TextAtom {
            content: format!("{:-^53}+", title),
            special_formatting: None,
        }],
        7 => vec![text::TextAtom {
            content: format!("{:^53}|", "N/A"),
            special_formatting: None,
        }],
        14 => vec![text::TextAtom {
            content: TILE_FOOTER_NOSTART.into(),
            special_formatting: None,
        }],
        1..14 => vec![text::TextAtom {
            content: EMPTY_LINE_NOSTART.into(),
            special_formatting: None,
        }],
        _ => unreachable!(),
    }
}

pub fn brightness_tile(brightness: f64, line: usize) -> Vec<TextAtom> {
    let mut index = 0;
    for i in 0..BRIGHTNESSES.len() {
        if BRIGHTNESSES[i].1 > brightness {
            index = i;
            break;
        }
    }

    match line {
        0 => vec![text::TextAtom {
            content: format!("{:-^53}+", " Brightness "),
            special_formatting: None,
        }],
        2 => vec![
            text::TextAtom {
                content: format!("{:^53}", format!("Brightness: {:.2}", brightness)),
                special_formatting: Some(TextFormatting {
                    bold: true,
                    italic: false,
                    underline: false,
                    color: Some(Color(ANSIColors::Blue, true)),
                    bgcolor: None,
                }),
            },
            TextAtom {
                content: "|".into(),
                special_formatting: None,
            },
        ],
        4..7 | 8..11 => {
            let item = if index + (line - 4) >= 3 {
                BRIGHTNESSES
                    .get(index + (line - 4) - 3)
                    .unwrap_or(&("", 0.0))
            } else {
                &("", 0.0)
            };
            vec![
                TextAtom {
                    content: if item.0 != "" {
                        format!("{:^53}", format!("{}: {}", item.0, item.1))
                    } else {
                        format!("{:^53}", "-")
                    },
                    special_formatting: Some(TextFormatting {
                        bold: false,
                        italic: true,
                        underline: false,
                        color: Some(Color(ANSIColors::Magenta, false)),
                        bgcolor: None,
                    }),
                },
                TextAtom {
                    content: "|".into(),
                    special_formatting: None,
                },
            ]
        }
        7 => vec![
            TextAtom {
                content: format!("{:^53}", format!("Current Observation: {:0.2}", brightness)),
                special_formatting: Some(TextFormatting {
                    bold: true,
                    italic: false,
                    underline: false,
                    color: None,
                    bgcolor: None,
                }),
            },
            TextAtom {
                content: "|".into(),
                special_formatting: None,
            },
        ],
        14 => vec![text::TextAtom {
            content: TILE_FOOTER_NOSTART.into(),
            special_formatting: None,
        }],
        1..14 => vec![text::TextAtom {
            content: EMPTY_LINE_NOSTART.into(),
            special_formatting: None,
        }],
        _ => unreachable!(),
    }
}

pub fn color_of(obj: &CelObj) -> Color {
    match obj {
        CelObj::Moon => Color(ANSIColors::White, false),
        CelObj::Planet(p) => match p.name {
            "Venus" | "Pluto" => Color(ANSIColors::White, true),
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

pub fn phase_tile(phaseangle: f64, line: usize, obj: &CelObj) -> Vec<TextAtom> {
    match line {
        0 => vec![text::TextAtom {
            content: format!("{:-^53}+", " Phase "),
            special_formatting: None,
        }],
        7 => vec![
            text::TextAtom {
                content: format!("{:<23}", crecent_slice(phaseangle, line - 2, 0.5, 10)),
                special_formatting: Some(TextFormatting {
                    bold: true,
                    italic: false,
                    underline: false,
                    color: Some(color_of(&obj)),
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
                content: format!("{:<53}", crecent_slice(phaseangle, line - 2, 0.5, 10)),
                special_formatting: Some(TextFormatting {
                    bold: true,
                    italic: false,
                    underline: false,
                    color: Some(color_of(&obj)),
                    bgcolor: None,
                }),
            },
            text::TextAtom {
                content: "|".into(),
                special_formatting: None,
            },
        ],
        14 => vec![text::TextAtom {
            content: TILE_FOOTER_NOSTART.into(),
            special_formatting: None,
        }],
        1..14 => vec![text::TextAtom {
            content: EMPTY_LINE_NOSTART.into(),
            special_formatting: None,
        }],
        _ => unreachable!(),
    }
}
