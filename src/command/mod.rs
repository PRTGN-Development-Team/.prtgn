/*
 * .prtgn Copyright (C) 2026 PRTGN Development Team
 *
 * This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License along with this program. If not, see https://www.gnu.org/licenses/.
 */

use clap::{Parser, Subcommand, CommandFactory, FromArgMatches};
use colored::Colorize;

mod prtgn_init;
mod prtgn_wav;

mod prtgn_flac;
mod player;
mod prtgn_fox;

fn get_about() -> String {
    let art = format!(
        "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
        "             ++++++++++++++++++++".red(),
        "          +÷÷÷÷××××××××××××××××÷÷÷÷++".red(),
        "        +-÷÷÷××××××××××××××××××××÷÷÷×+".red(),
        "      +-÷÷÷××××××××××××××××××××××××÷÷÷++".red(),
        "    ++÷÷÷×××××××××÷÷÷÷÷÷÷××××××××××××÷÷÷++".red(),
        "  ++÷÷÷×××××××××÷÷÷÷+÷÷÷÷÷÷÷÷÷÷×××××××÷÷÷÷++".red(),
        " +÷÷÷××××××××××÷÷÷++   +++++-÷÷÷÷×××××××÷÷÷÷++".red(),
        " +÷÷×××××××××÷÷÷++            ++÷÷××××××××÷÷++".red(),
        format!("{}{}{}", " +÷÷×××××××÷÷÷-+   ".red(), "+∞√√√∞++".cyan(), "    +÷÷×××××××××÷++".red()),
        format!("{}{}{}", " +÷÷××××××÷÷×++  ".red(), "+≈√√√√ππ√π√".cyan(), "   +×÷÷÷×××××××÷++".red()),
        format!("{}{}{}", " +÷÷××××÷÷÷÷++  ".red(), "∞√√√ππππππ√π+".cyan(), "    +×÷÷÷×××××÷++".red()),
        format!("{}{}{}", " +÷÷××÷÷÷÷++    ".red(), "+∞√√ππ√πππ√√√+".cyan(), "    ++×÷÷÷×××÷++".red()),
        format!("{}{}{}", " +÷÷÷÷÷÷++       ".red(), "+π√√ππππ√√√√+".cyan(), "      ++÷÷÷÷×÷++".red()),
        format!("{}{}{}", " +÷÷÷×++    +++   ".red(), "∞√ππ√√√√√+".cyan(), "   ++     ++×÷÷÷++".red()),
        format!("{}{}{}", " +÷÷+      +÷÷++     ".red(), "++≈∞+".cyan(), "   ++÷÷÷++     +×÷++".red()),
        " +++    ++×÷÷÷÷++++        ++÷÷÷÷÷÷÷++    ++++".red(),
        "      ++-÷÷÷××÷÷÷÷÷÷×++++++÷÷÷÷×××÷÷÷÷++".red(),
        "     +÷÷÷÷××××××××÷÷÷÷÷÷÷÷÷÷÷×××××××÷÷÷÷++".red(),
        "     +×÷÷÷×××××××××××××××÷××××××××××÷÷÷÷+".red(),
        "       +×÷÷÷××××××××××××××××××××××÷÷÷÷++".red(),
        "        ++÷÷÷÷×××××××××××××××××××÷÷÷++".red(),
        "            ++÷÷÷÷÷÷÷÷÷÷÷÷÷÷÷÷÷÷++".red()
    );

    let separator = "-----------------------------------------------------------------".cyan();

    format!(
        "{}\n\n    {}\n\n    {}\n\n    {}\n\n    {}  Copyright (C) 2026  PRTGN Development Team\n    This program comes with ABSOLUTELY NO WARRANTY.\n    This is free software, and you are welcome to redistribute it\n    under certain conditions.\n\n    Licensed under the GNU General Public License v3.0\n\n    {}\n\n    Artiy and official PRTGN artwork Copyright (C) 2025 by PRTGN Development Team\n    Licensed under Creative Commons Attribution-NonCommercial 4.0 International.\n    To view a copy of this license, visit https://creativecommons.org/licenses/by-nc/4.0/\n\n    PRTGN Official artwork can be found at this repo : https://github.com/PRTGN-Development-Team/PRTGN_Artwork\n\n    {}\n\n    Protogens, Primagens, and Zeniths Outer Reach (ZOR) were created by CoolKoinu.\n    All credit for Protogens, Primagens, and ZOR go to them and there team.\n\n\n    {}\n\n    Project inspired by Generic Purple Protogen (https://www.youtube.com/@genericpurpleprotogen1)\n\n    {}",
        art,
        separator,
        "A protogen CLI resource.".bold(),
        separator,
        ".prtgn".green(),
        separator,
        separator,
        separator,
        separator
    )
}

#[derive(Parser)]
#[command(author, version)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Create or Edit/View a .prtgn file")]
    Init {
        filename: String,
    },
    #[command(about = "Create or playback a .prtgn_wav file | Inspired by ivycomb (https://www.youtube.com/@ivycomb) / Ivycomb Music (https://www.youtube.com/@IvycombMusic) / ANTIHUMAN / ANTIHUMAN Inc")]
    Wav {
        filename: String,
        #[arg(long, action = clap::ArgAction::SetTrue)]
        convert: bool,
        #[arg(long, action = clap::ArgAction::SetTrue)]
        play: bool,
    },
    #[command(about = "Create or playback a .prtgn_flac file | Inspired by ivycomb (https://www.youtube.com/@ivycomb) / Ivycomb Music (https://www.youtube.com/@IvycombMusic) / ANTIHUMAN / ANTIHUMAN Inc ")]
    Flac {
        filename: String,
        #[arg(long, action = clap::ArgAction::SetTrue)]
        convert: bool,
        #[arg(long, action = clap::ArgAction::SetTrue)]
        play: bool,
    },
    #[command(about = "Display a random fox image from the fox.pics api | Inspired by Save A Fox (https://www.youtube.com/@saveafox)")]
    Fox {

    },

}

pub fn command() {
    let args_vec: Vec<String> = std::env::args().collect();
    if args_vec.len() == 2 {
        let arg = &args_vec[1];
        if !arg.starts_with('-') && !["init", "wav", "flac", "fox"].contains(&arg.as_str()) {
            let path = std::path::Path::new(arg);
            if let Some(ext) = path.extension().and_then(|s| s.to_str()) {
                match ext {
                    "prtgn" => {
                        println!("Init: {:?}", arg);
                        prtgn_init::init(arg.to_string());
                        return;
                    }
                    "prtgn_wav" => {
                        println!("Wav: {:?}", arg);
                        println!("Convert : false");
                        println!("Play : true");
                        prtgn_wav::wav(arg.to_string(), false, true);
                        return;
                    }
                    "prtgn_flac" => {
                        println!("Flac: {:?}", arg);
                        println!("Convert : false");
                        println!("Play : true");
                        prtgn_flac::flac(arg.to_string(), false, true);
                        return;
                    }
                    _ => {}
                }
            }
        }
    }

    let matches = Cli::command().about(get_about()).get_matches();
    let args = Cli::from_arg_matches(&matches).expect("Failed to parse args");

    match &args.command {
        Some(Commands::Init { filename }) => {
            println!("Init: {:?}", filename);
            prtgn_init::init(filename.to_string());
        }
        Some(Commands::Wav { filename, convert, play}) => {
            println!("Wav: {:?}", filename);
            println!("Convert : {:?}", convert);
            println!("Play : {:?}", play);
            prtgn_wav::wav(filename.to_string(), convert.to_owned(), play.to_owned());
        }
        Some(Commands::Flac { filename, convert, play}) => {
            println!("Flac: {:?}", filename);
            println!("Convert : {:?}", convert);
            println!("Play : {:?}", play);
            prtgn_flac::flac(filename.to_string(), convert.to_owned(), play.to_owned());
        }
        Some(Commands::Fox {}) => {

            prtgn_fox::fox(false);

        }
        // Some(Commands::Open { filename }) => {
        //     println!("Open: {}", filename);
        //     prtgn_open::open_file(filename.to_string());a
        //     //prtgn_open::open_file(open_filename);
        // }
        None => {
            println!("Hey you! Ya you! Artiy ain't very happy right now. Ya' didn't give them a sub command!");
        }
    }
}
