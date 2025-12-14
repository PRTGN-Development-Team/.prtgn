
// code for the 'new' sub-command
pub mod text_editor;

pub fn init(filename: String) {
    let mut filename_prt = filename;
    
        if !filename_prt.ends_with(".prtgn") {
       filename_prt.push_str(".prtgn");
        text_editor::editor(filename_prt).unwrap();
      }
    else {
        text_editor::editor(filename_prt).unwrap();
    }
    
}
