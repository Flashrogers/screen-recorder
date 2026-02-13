pub mod dx_capture;
pub mod encoder;

use dx_capture::DxgiCapture;
use encoder::FfmpegEncoder;

pub struct Recorder {
    capture: DxgiCapture,
    encoder: Option<FfmpegEncoder>,
    running: bool,
}

impl Recorder {
    pub fn new() -> Self {
        let capture = DxgiCapture::new().expect("Failed to initialize DXGI");

        Self {
            capture,
            encoder: None,
            running: false,
        }
    }

    pub fn start(&mut self, output: &str) -> Result<(), String> {
        let width = self.capture.width();
        let height = self.capture.height();

        let encoder =
            FfmpegEncoder::new(width, height, output)?;

        self.encoder = Some(encoder);
        self.running = true;

        while self.running {
            let frame = self
                .capture
                .capture_frame()
                .map_err(|e| e.to_string())?;

            if let Some(enc) = &mut self.encoder {
                enc.write_frame(&frame)?;
            }
        }

        Ok(())
    }

    pub fn stop(&mut self) -> Result<(), String> {
        self.running = false;

        if let Some(enc) = &mut self.encoder {
            enc.finish()?;
        }

        Ok(())
    }
}
