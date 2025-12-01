use rodio::Decoder;
use std::fs::File;
use std::io::BufReader;
use prtgn_encoding::write;

pub fn wav_to_prtgn(filename_prt: String) -> Result<(), Box<dyn std::error::Error>> {

    let mut filename_wav_mut = filename_prt.clone();

    if !filename_wav_mut.ends_with(".wav") {
        filename_wav_mut.push_str(".wav");

        let file = File::open(&filename_wav_mut)?;
        let file_reader = BufReader::new(file);

        // Decode the audio file
        let decoder = Decoder::new(file_reader)?;
        let samples: Vec<f32> = decoder.collect();
        let source: String = samples
            .into_iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(" ");

        let mut filenam_prt_wav = filename_prt.clone();
        if let Some(stripped) = filenam_prt_wav.strip_suffix(".wav") {
            filenam_prt_wav = stripped.to_string();
        }

        if !filenam_prt_wav.ends_with(".prtgn_wav") {
            filenam_prt_wav.push_str(".prtgn_wav");
        }

        write(filenam_prt_wav, source)?;
    }
    else if !filename_wav_mut.ends_with(".prtgn_wav") {
        // call player
    }
    else {
        println!("Mate, ya gotta stop upsetting Artiy. You requested a prtgn_wav, but didnt give em the correct file!")
    }



    Ok(())
}

pub fn prtgn_to_wav(filename_prt: String) -> Result<(), Box<dyn std::error::Error>> {

    // let mut filename_wav_mut = filename_prt.clone();
    // 
    // if !filename_wav_mut.ends_with(".wav") {
    //     filename_wav_mut.push_str(".wav");
    //
    //     let file = File::open(&filename_wav_mut)?;
    //     let file_reader = BufReader::new(file);
    //
    //     // Decode the audio file
    //     let decoder = Decoder::new(file_reader)?;
    //     let samples: Vec<f32> = decoder.collect();
    //     let source: String = samples
    //         .into_iter()
    //         .map(|s| s.to_string())
    //         .collect::<Vec<String>>()
    //         .join(" ");
    //
    //     let mut filenam_prt_wav = filename_prt.clone();
    //     if let Some(stripped) = filenam_prt_wav.strip_suffix(".wav") {
    //         filenam_prt_wav = stripped.to_string();
    //     }
    //
    //     if !filenam_prt_wav.ends_with(".prtgn_wav") {
    //         filenam_prt_wav.push_str(".prtgn_wav");
    //     }
    //
    //     write(filenam_prt_wav, source)?;
    // }
    // else if !filename_wav_mut.ends_with(".prtgn_wav") {
    //
    // }
    // else {
    //     println!("Mate, ya gotta stop upsetting Artiy. You requested a prtgn_wav, but didnt give em the correct file!")
    // }



    Ok(())
}

// Order of operations. Wav -> string -> prtgn -> string -> wav -> playback