pub mod take_args {
    use clap::Parser;
    #[derive(Parser)]
    pub struct Args {
        /// The path to the sound file to play.
        #[arg(short, long)]
        path: String,
    }
    pub fn get_path() -> String {
        let args = Args::parse();
        args.path
    }
}

pub mod playing_sound {

    use rodio::{source::Source, Decoder, OutputStream, Sink};
    use std::env::var;
    use std::fs::File;
    use std::io::BufReader;

    struct Configuration {
        volume: f32,
    }

    fn extract_configuration() -> Configuration {
        let volume = var("VOLUME")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1.0);
        Configuration { volume }
    }

    pub fn get_source() -> Decoder<BufReader<File>> {
        let file_path: String = super::take_args::get_path();
        let file = File::open(file_path).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();
        source
    }

    pub fn play_sound() {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let source = get_source();
        let sink = Sink::try_new(&stream_handle).unwrap();
        let configuration = extract_configuration();

        // Sink initialisation and start.
        sink.set_volume(configuration.volume);
        sink.append(source);
        sink.sleep_until_end();
    }
}
