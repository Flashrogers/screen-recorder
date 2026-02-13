use std::io::Write;
use std::process::{Child, ChildStdin, Command, Stdio};

pub struct FfmpegEncoder {
    child: Child,
    stdin: ChildStdin,
}

impl FfmpegEncoder {
    pub fn new(width: u32, height: u32, output: &str) -> Result<Self, String> {
        let mut child = Command::new("C:\\ffmpeg\\bin\\ffmpeg.exe")
            .args([
                "-y",
                "-f", "rawvideo",
                "-pixel_format", "bgra",
                "-video_size", &format!("{}x{}", width, height),
                "-framerate", "60",
                "-i", "pipe:0",
                "-c:v", "libx264",
                "-pix_fmt", "yuv420p",
                output,
            ])
            .stdin(Stdio::piped())
            .spawn()
            .map_err(|e| format!("Failed to start FFmpeg: {}", e))?;

        let stdin = child
            .stdin
            .take()
            .ok_or("Failed to open FFmpeg stdin")?;

        Ok(Self { child, stdin })
    }

    pub fn write_frame(&mut self, frame: &[u8]) -> Result<(), String> {
        self.stdin
            .write_all(frame)
            .map_err(|e| format!("Failed to write frame: {}", e))
    }

    pub fn finish(&mut self) -> Result<(), String> {
        drop(&self.stdin);

        self.child
            .wait()
            .map_err(|e| format!("Failed to finalize FFmpeg: {}", e))?;

        Ok(())
    }
}
