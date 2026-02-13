mod recorder;

use recorder::Recorder;

fn main() -> Result<(), String> {
    let mut recorder = Recorder::new();

    println!("Starting recording...");
    recorder.start("output.mp4")?;

    std::thread::sleep(std::time::Duration::from_secs(5));

    println!("Stopping recording...");
    recorder.stop()?;

    println!("Recording finished.");

    Ok(())
}
