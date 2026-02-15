pub struct AudioCapture;

impl AudioCapture {
    pub fn new() -> Self {
        AudioCapture
    }

    pub fn capture_audio_frame(&mut self) -> Result<Vec<u8>, String> {
        Ok(Vec::new())
    }
}
