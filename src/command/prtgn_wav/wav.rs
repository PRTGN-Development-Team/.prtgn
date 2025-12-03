use crate::command::prtgn_wav::wav_converter;

pub fn wav(filename: String) {
    
    let filename_prt = filename;
    
        if filename_prt.ends_with(".prtgn_wav") {

            wav_converter::prtgn_to_wav(filename_prt).unwrap();

        }
        else if filename_prt.ends_with(".wav") {

            wav_converter::wav_to_prtgn(filename_prt).unwrap();

        }
        else {
            println!("Mate, ya gotta stop upsetting Artiy. You requested a prtgn_wav, but didnt give em the correct file!")
        }
        //wav_converter::wav_to_prtgn(filename_prt);
    
    }


