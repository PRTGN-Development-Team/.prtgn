use crate::command::prtgn_wav::player::player;
use hound;
use prtgn_encoding::{read, write};
use rodio::{Decoder, Source};
use std::fs::File;
use std::io::BufReader;
use indicatif::{ProgressBar, ProgressStyle};


pub fn wav_to_prtgn(filename: String) -> Result<(), Box<dyn std::error::Error>> {

    println!("Converting `{filename}` into a PRTGN_WAV file.");

    let mut filename_wav = filename.clone();
    if !filename_wav.ends_with(".wav") {
        filename_wav.push_str(".wav");
    }

    // Use hound to get the number of samples for the progress bar
    let reader = hound::WavReader::open(&filename_wav)?;
    let num_samples = reader.len() as u64;

    let pb = ProgressBar::new(num_samples * 2);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.red} [{elapsed_precise}] [{bar:40.magenta/cyan}] {pos}/{len} ({eta}) {msg}")
        .expect("Failed to set progress bar template")
        .progress_chars("=>-"));
    pb.set_message("Transforming...");

    let file = File::open(&filename_wav)?;
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

    pb.finish_with_message("Congrats on you're new PRTGN_WAV file. Enjoy!");

    let source = format!("{} {} {}", channels, sample_rate, samples_string);

    let mut filename_prtgn = filename;
    if let Some(stripped) = filename_prtgn.strip_suffix(".wav") {
        filename_prtgn = stripped.to_string();
    }
    if !filename_prtgn.ends_with(".prtgn_wav") {
        filename_prtgn.push_str(".prtgn_wav");
    }

    write(filename_prtgn, source)?;

    Ok(())
}



pub fn prtgn_to_wav(filename: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut filename_prtgn = filename.clone();
    if !filename_prtgn.ends_with(".prtgn_wav") {
        filename_prtgn.push_str(".prtgn_wav");
    }

    let decoded_string = read(filename_prtgn)?;

    let parts: Vec<&str> = decoded_string.splitn(3, ' ').collect();
    if parts.len() < 3 {
        return Err("Invalid prtgn_wav format: missing metadata".into());
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

    let mut filename_wav = filename;
    if let Some(stripped) = filename_wav.strip_suffix(".prtgn_wav") {
        filename_wav = stripped.to_string();
    }
    if !filename_wav.ends_with(".wav") {
        filename_wav.push_str(".wav");
    }

    let spec = hound::WavSpec {
        channels,
        sample_rate,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };

    let mut writer = hound::WavWriter::create(&filename_wav, spec)?;
    for sample in samples {
        writer.write_sample(sample)?;
        pb.inc(1);
    }
    writer.finalize()?;

    pb.finish_with_message("Now playing..");

    player(filename_wav)?;

    Ok(())
}

// Order of operations. Wav -> string -> prtgn -> string -> wav -> playback
