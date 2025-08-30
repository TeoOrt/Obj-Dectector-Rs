use std::sync::mpsc::Receiver;
use opencv::prelude::*;
use opencv::highgui;



    // pub fn display_video(&mut self, frame: &Mat) -> Result<char> {
    //     highgui::imshow(&self.display_window.name, frame)?;
    //     let key_pressed = highgui::wait_key(1)? as u32;
    //     Ok(char::from_u32(key_pressed.into()).unwrap_or_default())
    // }
pub fn display_video(receiver : Receiver<Mat>) {
    loop {
        let frame = receiver.recv().expect("Error receiving MAt");
        highgui::imshow("Camera1", &frame ).unwrap();
        if highgui::wait_key(1).unwrap() == 'q' as i32{
            break;
        }
    }
}
