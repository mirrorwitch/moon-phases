use clap::Parser;
use std::fmt;
use moon_phase::MoonPhase;
use std::time::SystemTime;
use chrono::{Datelike,Timelike,DateTime,offset::Utc,TimeZone};
use human_date_parser::from_human_time;

// Unicode variation selectors (VS): these are invisible characters that will make the previous
// emoji show in text- or color presentation.
//
// If no VS is present it's up to the system how to display the emojis.
const VS15: &str = "\u{fe0e}"; // text emoji
const VS16: &str = "\u{fe0f}"; // color emoji

const NORTH_EMOJI: [&str; 8] = [
    "🌑",
    "🌒",
    "🌓",
    "🌔",
    "🌕",
    "🌖",
    "🌗",
    "🌘",
];
const SOUTH_EMOJI: [&str; 8] = [
    "🌑",
    "🌘",
    "🌗",
    "🌖",
    "🌕",
    "🌔",
    "🌓",
    "🌒",
];
const NORTH_EMOJI_FACE: [&str; 8] = [
    "🌚",
    "🌚",
    "🌛",
    "🌛",
    "🌝",
    "🌝",
    "🌜",
    "🌜",
];
const SOUTH_EMOJI_FACE: [&str; 8] = [
    "🌚",
    "🌚",
    "🌜",
    "🌜",
    "🌝",
    "🌝",
    "🌛",
    "🌛",
];

#[derive(clap::ValueEnum, Clone, Debug)]
enum Mode {
    Name,
    Emoji,
    Numeric,
    Zodiac
}
impl std::fmt::Display for Mode {
    // Display the name of the enum value in lowercase
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s : String = format!("{:?}", self);
        write!(f, "{}", s.to_lowercase())
    }
}

#[derive(Parser)]
#[command(author,
          version,
          about="Show the moon phase as an emoji, number, or string.",
          max_term_width=80,
          long_about = None )]
struct Cli {
    /// How to show the moon phase, or moon sign for "zodiac"
    #[arg(short, long, default_value_t=Mode::Name,)]
    mode: Mode,

    /// Equivalent to --mode numeric
    #[arg(short, long)]
    numeric: bool,

    /// Equivalent to --mode emoji
    #[arg(short, long)]
    emoji: bool,

    /// Equivalent to --mode zodiac
    #[arg(short, long)]
    zodiac: bool,

    /// Use emojis direction for the Southern hemisphere (waxing crescent is 🌘)
    #[arg(short, long)]
    south_hemisphere: bool,

    /// Use variation selectors to prefer colour emoji (support depends on terminal)
    #[arg(short, long)]
    color_emoji: bool,

    /// Use variation selectors to prefer text emoji (monochrome)
    #[arg(short, long)]
    text_emoji: bool,

    /// Use cartoon face moon emojis (reduce distinct phases from 8 to 4)
    #[arg(short, long)]
    face_emoji: bool,


    /// Date with optional time to query the moon phase
    /// (e.g. "2023-10-31", "2023-10-31 23:59:59", "Friday").
    /// By default, shows the current date and time.
    date: Option<String>,

}

fn str_to_system_time(timestr: &str) -> Result<SystemTime, &'static str> {
    match from_human_time(timestr) {
        Ok(result) => {
            match result {
                human_date_parser::ParseResult::DateTime(dt) => { 
                    let utc: DateTime<Utc> = dt.into();
                    Ok(utc.into())
                },
                human_date_parser::ParseResult::Date(nd) => {
                    // can you get the local tz without needing a .now()?
                    let tz = chrono::Local::now().timezone();
                    let datetime_local = tz.with_ymd_and_hms(
                        nd.year(), nd.month(), nd.day(),
                        0,0,0
                    );
                    let datetime_utc: DateTime<Utc> = datetime_local.unwrap().into();
                    Ok(datetime_utc.into())
                },
                human_date_parser::ParseResult::Time(nt) => {
                    let now = chrono::Local::now();
                    let tz = now.timezone();
                    let datetime_local = tz.with_ymd_and_hms(
                        now.year(), now.month(), now.day(),
                        nt.hour(), nt.minute(), nt.second(),
                    );
                    let datetime_utc: DateTime<Utc> = datetime_local.unwrap().into();
                    Ok(datetime_utc.into())
                }
            }
        }
        Err(_) => Err("Invalid date")
    }
}

fn to_emoji(phase: f64,
            south_hemisphere: bool, 
            face: bool,
            color_emoji: bool,
            text_emoji: bool)
    -> String {
        let emoji_set = if south_hemisphere && face {
            SOUTH_EMOJI_FACE
        } else if south_hemisphere {
            SOUTH_EMOJI
        } else if face {
            NORTH_EMOJI_FACE
        } else {
            NORTH_EMOJI
        };
        let vs = if color_emoji {
            VS16
        } else if text_emoji {
            VS15
        } else {
            ""
        };
        let emoji = match phase {
            x if x <  0.125 => emoji_set[0],
            x if x <  0.25  => emoji_set[1],
            x if x <  0.375 => emoji_set[2],
            x if x <  0.50  => emoji_set[3],
            x if x <  0.625 => emoji_set[4],
            x if x <  0.75  => emoji_set[5],
            x if x <  0.875 => emoji_set[6],
            x if x <  1.00  => emoji_set[7],
            _ => emoji_set[0]
        };

        format!("{}{}", emoji, vs)
}

fn main() {
    let cli = Cli::parse();

    let mode: Mode = if cli.numeric {
        Mode::Numeric
    } else if cli.emoji {
        Mode::Emoji
    } else if cli.zodiac {
        Mode::Zodiac
    } else {
        cli.mode
    };

    let moontime: SystemTime;
    if cli.date.is_some() {
        match str_to_system_time(cli.date.unwrap().as_str()) {
            Ok(t) => { moontime = t;} 
            Err(_) => {
                println!("Invalid date!");
                std::process::exit(2);
            }
        }
    } else {
        moontime = SystemTime::now();
    }

    let moon = MoonPhase::new(moontime);

    match mode {
        Mode::Numeric => println!("{:1.2}", moon.phase),
        Mode::Name    => println!("{}", moon.phase_name),
        Mode::Zodiac  => println!("{}", moon.zodiac_name),
        _ => {
            let emoji = to_emoji(moon.phase,
                                 cli.south_hemisphere,
                                 cli.face_emoji,
                                 cli.color_emoji,
                                 cli.text_emoji);

            println!("{}", emoji);
        }
    }
}
