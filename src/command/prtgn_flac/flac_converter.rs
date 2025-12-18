use crate::command::prtgn_flac::player::player;
use prtgn_encoding::{read, write};
use rodio::{Decoder, Source};
use std::fs::File;
use std::io::{BufReader, BufWriter};
use indicatif::{ProgressBar, ProgressStyle};
use flac_codec::decode::StreamDecode;
use flac_codec::encode::StreamEncode;
use flac_codec::model::{Block, StreamInfo};

pub fn flac_to_prtgn(filename: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("Converting `{filename}` into a PRTGN_FLAC file.");

    let mut filename_flac = filename.clone();
    if !filename_flac.ends_with(".flac") {
        filename_flac.push_str(".flac");
    }

    let input = Box::new(File::open(&filename_flac)?);
    let (mut decoder, stream_info) = StreamDecoder::new(input)?;

    let num_samples = stream_info.total_samples;

    let pb = ProgressBar::new(num_samples * 2);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.red} [{elapsed_precise}] [{bar:40.magenta/cyan}] {pos}/{len} ({eta}) {msg}")
        .expect("Failed to set progress bar template")
        .progress_chars("=>-"));
    pb.set_message("Transforming...");

    let sample_rate = stream_info.sample_rate;
    let channels = stream_info.channels;
    let bits_per_sample = stream_info.bits_per_sample;

    let mut samples = Vec::new();
    while let Some(block) = decoder.next_block()? {
        if let Block::Data(data) = block {
            for i in 0..data.nsamples() {
                for j in 0..data.nchannels() {
                    let sample = data.sample(j, i);
                    pb.inc(1);
                    samples.push(sample as f32 / (1 << (bits_per_sample - 1)) as f32);
                }
            }
        }
    }

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

    let decoded_string = read(filename_prtgn)?;

    let parts: Vec<&str> = decoded_string.splitn(3, ' ').collect();
    if parts.len() < 3 {
        return Err("Mate. No. That wont work. Your files invalid. Try again.".into());
    }

    let channels: u32 = parts[0].parse()?;
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

    let samples: Vec<i32> = sample_strings
        .into_iter()
        .filter_map(|s| {
            pb.inc(1);
            s.parse::<f32>().ok().map(|x| (x * i32::MAX as f32) as i32)
        })
        .collect();

    let mut filename_flac = filename;
    if let Some(stripped) = filename_flac.strip_suffix(".prtgn_flac") {
        filename_flac = stripped.to_string();
    }
    if !filename_flac.ends_with(".flac") {
        filename_flac.push_str(".flac");
    }

    let writer = Box::new(BufWriter::new(File::create(&filename_flac)?));
    let stream_info = StreamInfo::new(sample_rate, channels, 32);
    let mut encoder = StreamEncoder::new(writer, stream_info)?;

    encoder.write_interleaved(samples.as_slice())?;
    encoder.finish()?;

    pb.finish_with_message("Now playing..");

    player(filename_flac)?;

    Ok(())
}
