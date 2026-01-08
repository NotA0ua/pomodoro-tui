use rodio::{source::Source, Decoder, OutputStream};
use std::fs::File;
use std::io::BufReader;
use std::thread;

pub fn play_timer_sound() {
    thread::spawn(|| {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let file = BufReader::new(File::open("sounds/timer_end_sound.mp3").unwrap());
        let source = Decoder::new(file).unwrap();
        stream_handle.play_raw(source.convert_samples()).unwrap();

        std::thread::sleep(std::time::Duration::from_secs(5));
    });
}
