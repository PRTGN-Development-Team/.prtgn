use crate::command::prtgn_wav::wav_converter;

pub fn wav(filename: String, convert: bool, play: bool) {


    let mut filename_prt = filename;

        if filename_prt.ends_with(".prtgn_wav"){
            wav_converter::prtgn_to_wav(filename_prt).unwrap();
        }
        else if convert == true && play == true {

            let mut fileanme_prt_wav = filename_prt.clone();

            if fileanme_prt_wav.ends_with(".wav") {
                wav_converter::wav_to_prtgn(fileanme_prt_wav).unwrap();
            }
            else {
                if !fileanme_prt_wav.ends_with(".wav") {
                    fileanme_prt_wav.push_str(".wav");
                    wav_converter::wav_to_prtgn(fileanme_prt_wav).unwrap();

                }
            }

            if !filename_prt.ends_with(".prtgn_wav") {
                filename_prt.push_str(".prtgn_wav");
                wav_converter::prtgn_to_wav(filename_prt).unwrap();
            }

        }
        else if convert == true {

            if filename_prt.ends_with(".wav") {
                wav_converter::wav_to_prtgn(filename_prt).unwrap();
            }
            else {
                if !filename_prt.ends_with(".wav") {
                    filename_prt.push_str(".wav");
                    wav_converter::wav_to_prtgn(filename_prt).unwrap();

                }
            }

        }
        else if play == true {
            if !filename_prt.ends_with(".prtgn_wav") {
                filename_prt.push_str(".prtgn_wav");
                wav_converter::prtgn_to_wav(filename_prt).unwrap();
            }
        }
        else {
            if !filename_prt.ends_with(".prtgn_wav") {
                filename_prt.push_str(".prtgn_wav");
                wav_converter::prtgn_to_wav(filename_prt).unwrap();
            }
        };

    }

// if play & convert are true, play convert. If one is true do one. If none is true do play.


