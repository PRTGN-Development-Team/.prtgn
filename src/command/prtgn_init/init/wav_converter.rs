use rodio::{Decoder, OutputStream, OutputStreamBuilder, Sink};
use std::fs::File;
use std::io::BufReader;


pub fn wav_to_prtgn(filename_prt: String) -> Result<(), Box<dyn std::error::Error>> {

    // // let mut filename_wav_prt = filename_prt;
    // 
    // 
    // //     if !filename_wav_prt.ends_with(".wav") {
    // //    filename_wav_prt.push_str(".wav");
    // // }
    // 
    // let file = File::open(filename_prt)?;
    // let file_reader = BufReader::new(file);
    // 
    // // Decode the audio file
    // let source = Decoder::new(file_reader)?;
    // let prtgn_text = format!("{source}");
    // let obscured = obscure(prtgn_text);
    // 
    // 
    Ok(())

    }