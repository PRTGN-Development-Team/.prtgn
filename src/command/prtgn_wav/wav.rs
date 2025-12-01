use crate::command::prtgn_wav::wav_converter;

pub fn wav(filename: String, wav: bool) {
    println!("Creating a new file...");
    println!("Wav flag is: {}", wav);

    let mut filename_prt = filename;

    if wav == true {
        if filename_prt.ends_with(".wav") {
        wav_converter::wav_to_prtgn(filename_prt);
      } 
        else if filename_prt.ends_with(".prtgn_wav") {
            
        }    
        else {
            println!("Mate, ya gotta stop upsetting Artiy. You requested a prtgn_wav, but didnt give em the correct file!")
        }
        //wav_converter::wav_to_prtgn(filename_prt);
    }
    else {
        println!("Mate, ya gotta stop upsetting Artiy. You requested a prtgn_wav, but didnt give em the correct file!")
    }


}