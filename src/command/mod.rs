use clap::{Parser, Subcommand};

mod prtgn_init;
mod prtgn_wav;

#[derive(Parser)]
#[command(author, version, about = "
    A protogen inspired file extension written in Rust.

    .prtgn  Copyright (C) 2025  PRTGN Development Team
    This program comes with ABSOLUTELY NO WARRANTY.
    This is free software, and you are welcome to redistribute it
    under certain conditions.

    Licensed under the GNU General Public License v3.0
")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Create or Edit/View a .prtgn file and sub-files")]
    Init {
        filename: String,
    },
    #[command(about = "Open the PRTGN Audio Interface")]
    Wav {
        filename: String,
        // #[arg(long, action = clap::ArgAction::SetTrue)]
        // wav: bool,
    },

}

pub fn command() {
    let args = Cli::parse();

    match &args.command {
        Some(Commands::Init { filename }) => {
            println!("Init: {:?}", filename);
            prtgn_init::init(filename.to_string());
        }
        Some(Commands::Wav { filename}) => {
            println!("Wav: {:?}", filename);
            prtgn_wav::wav(filename.to_string());

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

