use std::io::Write;
use std::process::{Child, Command, Stdio};

pub struct FfmpegEncoder {
    child: Child,
}

impl FfmpegEncoder {
    pub fn new(width: u32, height: u32, output: &str) -> Result<Self, String> {
        let child = Command::new("ffmpeg")
            .args([
                "-y",
                "-f", "rawvideo",
                "-pixel_format", "bgra",
                "-video_size", &format!("{}x{}", width, height),
                "-framerate", "60",
                "-i", "-",
                "-c:v", "libx264",
                "-pix_fmt", "yuv420p",
                output,
            ])
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to start FFmpeg: {}", e))?;

        Ok(Self { child })
    }

    pub fn write_frame(&mut self, frame: &[u8]) -> Result<(), String> {
        if let Some(stdin) = self.child.stdin.as_mut() {
            stdin
                .write_all(frame)
                .map_err(|e| format!("Failed to write frame: {}", e))?;
        }
        Ok(())
    }

    pub fn finish(&mut self) -> Result<(), String> {
        let _ = self.child.stdin.take();
        self.child
            .wait()
            .map_err(|e| format!("FFmpeg wait failed: {}", e))?;
        Ok(())
    }
}
