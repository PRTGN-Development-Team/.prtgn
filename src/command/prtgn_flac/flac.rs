use crate::command::prtgn_flac::flac_converter;

pub fn flac(filename: String, convert: bool, play: bool) {


    let mut filename_prt = filename;

        if filename_prt.ends_with(".prtgn_flac"){
            flac_converter::prtgn_to_flac(filename_prt).unwrap();
        }
        else if convert == true && play == true {

            let mut filename_prt_flac = filename_prt.clone();

            if filename_prt_flac.ends_with(".flac") {
                flac_converter::flac_to_prtgn(filename_prt_flac).unwrap();
            }
            else {
                if !filename_prt_flac.ends_with(".flac") {
                    filename_prt_flac.push_str(".flac");
                    flac_converter::flac_to_prtgn(filename_prt_flac).unwrap();

                }
            }

            if !filename_prt.ends_with(".prtgn_flac") {
                filename_prt.push_str(".prtgn_flac");
                flac_converter::prtgn_to_flac(filename_prt).unwrap();
            }

        }
        else if convert == true {

            if filename_prt.ends_with(".flac") {
                flac_converter::flac_to_prtgn(filename_prt).unwrap();
            }
            else {
                if !filename_prt.ends_with(".flac") {
                    filename_prt.push_str(".flac");
                    flac_converter::flac_to_prtgn(filename_prt).unwrap();

                }
            }

        }
        else if play == true {
            if !filename_prt.ends_with(".prtgn_flac") {
                filename_prt.push_str(".prtgn_flac");
                flac_converter::prtgn_to_flac(filename_prt).unwrap();
            }
        }
        else {
            if !filename_prt.ends_with(".prtgn_flac") {
                filename_prt.push_str(".prtgn_flac");
                flac_converter::prtgn_to_flac(filename_prt).unwrap();
            }
        };

    }

// if play & convert are true, play convert. If one is true do one. If none is true do play.


