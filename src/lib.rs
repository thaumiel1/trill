pub mod take_args {
    use clap::Parser;
    #[derive(Parser)]
    pub struct Args {
        #[arg(short, long)]
        path: String,
    }
    pub fn get_path() -> String {
        let args = Args.parse();
        args.path
    }
}

pub mod playing_sound {
    use std::fs::File;
    use std::io::BufReader;
    use rodio::{Decoder, OutputStream, source::Source};

    pub fn play_sound() {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let file = File::open("examples/Fox Stevenson - Can't Even Tell (Original Mix).flac").unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();
        stream_handle.play_raw(source.convert_samples()).unwrap();
        std::thread::sleep(std::time::Duration::from_secs(20));
    }

    use std::time::Duration;
    use rodio::Sink;
    use rodio::source::SineWave;

    pub fn play_sound_sink() {
        let file_path: String =
        let sink = Sink::try_new(&stream_handle).unwrap();
        let source = SineWave::new(440.0).take_duration(Duration::from_secs_f32(0.25)).amplify(0.20);
        sink.append(source);
        sink.sleep_until_end();
    }
}

fn main() {}