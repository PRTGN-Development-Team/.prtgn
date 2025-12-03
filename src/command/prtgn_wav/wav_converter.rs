use crate::command::prtgn_wav::player::play;
use hound;
use prtgn_encoding::{read, write};
use rodio::{Decoder, Source};
use std::fs::File;
use std::io::BufReader;

pub fn wav_to_prtgn(filename: String) -> Result<(), Box<dyn std::error::Error>> {
    let mut filename_wav = filename.clone();
    if !filename_wav.ends_with(".wav") {
        filename_wav.push_str(".wav");
    }

    let file = File::open(&filename_wav)?;
    let file_reader = BufReader::new(file);

    let decoder = Decoder::new(file_reader)?;
    let sample_rate = decoder.sample_rate();
    let channels = decoder.channels();
    let samples: Vec<f32> = decoder.collect();

    let samples_string: String = samples
        .into_iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>()
        .join(" ");

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

    let samples: Vec<f32> = samples_str
        .split(' ')
        .filter_map(|s| s.parse::<f32>().ok())
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
    }
    writer.finalize()?;

    play(filename_wav)?;

    Ok(())
}

// Order of operations. Wav -> string -> prtgn -> string -> wav -> playback
