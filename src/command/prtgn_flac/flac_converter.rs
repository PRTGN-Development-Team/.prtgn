use crate::command::player::player;
use claxon;
use prtgn_encoding::{read, write};
use rodio::{Decoder, Source, buffer::SamplesBuffer};
use std::fs::File;
use std::io::BufReader;
use indicatif::{ProgressBar, ProgressStyle};



pub fn flac_to_prtgn(filename: String) -> Result<(), Box<dyn std::error::Error>> {

    println!("Converting `{filename}` into a PRTGN_FLAC file.");

    let mut filename_flac = filename.clone();
    if !filename_flac.ends_with(".flac") {
        filename_flac.push_str(".flac");
    }

    // Use claxon to get the number of samples for the progress bar
    let reader = claxon::FlacReader::open(&filename_flac)?;
    let num_samples = reader.streaminfo().samples.unwrap_or(0);

    let pb = ProgressBar::new(num_samples * 2);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.red} [{elapsed_precise}] [{bar:40.magenta/cyan}] {pos}/{len} ({eta}) {msg}")
        .expect("Failed to set progress bar template")
        .progress_chars("=>-"));
    pb.set_message("Transforming...");

    let file = File::open(&filename_flac)?;
    let file_reader = BufReader::new(file);

    let decoder = Decoder::new(file_reader)?;
    let sample_rate = decoder.sample_rate();
    let channels = decoder.channels();

    let samples: Vec<f32> = decoder.map(|s| { pb.inc(1); s }).collect();

    let samples_vec: Vec<String> = samples
        .into_iter()
        .map(|s| { pb.inc(1); s.to_string() })
        .collect();

    let samples_string = samples_vec.join(" ");

    pb.finish_with_message("Congrats on you're new PRTGN_FLAC file. Enjoy!");

    let source = format!("{} {} {}", channels, sample_rate, samples_string);

    let mut filename_prtgn = filename;
    if let Some(stripped) = filename_prtgn.strip_suffix(".flac") {
        filename_prtgn = stripped.to_string();
    }
    if !filename_prtgn.ends_with(".prtgn_flac") {
        filename_prtgn.push_str(".prtgn_flac");
    }

    write(filename_prtgn, source)?;

    Ok(())
}



pub fn prtgn_to_flac(filename: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut filename_prtgn = filename.clone();
    if !filename_prtgn.ends_with(".prtgn_flac") {
        filename_prtgn.push_str(".prtgn_flac");
    }

    let decoded_string = read(filename_prtgn.clone())?;

    let parts: Vec<&str> = decoded_string.splitn(3, ' ').collect();
    if parts.len() < 3 {
        return Err("Mate. No. That wont work. Your files invalid. Try again.".into());
    }

    let channels: u16 = parts[0].parse()?;
    let sample_rate: u32 = parts[1].parse()?;
    let samples_str = parts[2];

    let sample_strings: Vec<&str> = samples_str.split(' ').collect();
    let num_samples = sample_strings.len() as u64;

    let pb = ProgressBar::new(num_samples * 2);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.red} [{elapsed_precise}] [{bar:40.magenta/cyan}] {pos}/{len} ({eta}) {msg}")
        .expect("Failed to set progress bar template")
        .progress_chars("=>-"));
    pb.set_message("Deciphering...");

    let samples: Vec<f32> = sample_strings
        .into_iter()
        .filter_map(|s| {
            pb.inc(1);
            s.parse::<f32>().ok()
        })
        .collect();

    let mut filename_flac = filename;
    if let Some(stripped) = filename_flac.strip_suffix(".prtgn_flac") {
        filename_flac = stripped.to_string();
    }
    if !filename_flac.ends_with(".flac") {
        filename_flac.push_str(".flac");
    }

    pb.finish_with_message("Now playing..");

    let source = SamplesBuffer::new(channels, sample_rate, samples);

    player(filename_prtgn)?;

    Ok(())
}