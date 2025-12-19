use clap::{Parser, Subcommand};

mod prtgn_init;
mod prtgn_wav;

mod prtgn_flac;
mod player;

#[derive(Parser)]
#[command(author, version, about = "
    -----------------------------------------------------------------

    A protogen inspired file extension written in Rust.

    -----------------------------------------------------------------

    .prtgn  Copyright (C) 2025  PRTGN Development Team
    This program comes with ABSOLUTELY NO WARRANTY.
    This is free software, and you are welcome to redistribute it
    under certain conditions.

    Licensed under the GNU General Public License v3.0

    -----------------------------------------------------------------

    Artiy and official PRTGN artwork Copyright (C) 2025 by PRTGN Development Team
    Licensed under Creative Commons Attribution-NonCommercial 4.0 International.
    To view a copy of this license, visit https://creativecommons.org/licenses/by-nc/4.0/

    PRTGN Official artwork can be found at this repo : https://github.com/PRTGN-Development-Team/PRTGN_Artwork

    -----------------------------------------------------------------

    Protogens, Primagens, and Zeniths Outer Reach (ZOR) were created by CoolKoinu.
    All credit for Protogens, Primagens, and ZOR go to them and there team.

")]
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
    #[command(about = "Create or playback a .prtgn_wav file")]
    Wav {
        filename: String,
        #[arg(long, action = clap::ArgAction::SetTrue)]
        convert: bool,
        #[arg(long, action = clap::ArgAction::SetTrue)]
        play: bool,
    },
    #[command(about = "Create or playback a .prtgn_flac file")]
    Flac {
        filename: String,
        #[arg(long, action = clap::ArgAction::SetTrue)]
        convert: bool,
        #[arg(long, action = clap::ArgAction::SetTrue)]
        play: bool,
    },

}

pub fn command() {
    let args = Cli::parse();

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

