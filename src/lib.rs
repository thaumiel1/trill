// TODO: Remove public keywords from functions that do not need to be public.

pub mod take_args {
    use clap::Parser;
    #[derive(Parser)]
    pub struct Args {
        /// The sound file to play.
        #[arg(short, long)]
        path: String,
    }
    pub fn get_path() -> String {
        let args = Args::parse();
        args.path
    }
}

// TODO: Accept environment variables as options for volume control, amplification, etc.

pub mod playing_sound {
    use std::fs::File;
    use std::io::BufReader;
    use rodio::{Decoder, OutputStream, Sink, source::Source};

    pub fn play_sound() {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let file_path: String = super::take_args::get_path();
        let file = File::open(file_path).unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        let source = rodio::Decoder::new(BufReader::new(file)).unwrap();
        sink.append(source);
        sink.sleep_until_end();
    }
}