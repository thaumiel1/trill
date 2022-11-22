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

// TODO: Accept environment variables as options for volume control.

pub mod playing_sound {

    use std::fs::File;
    use std::io::BufReader;
    use rodio::{Decoder, OutputStream, Sink, source::Source};

    struct Configuration {
        volume: f32,
    }

    fn extract_configuration() -> Configuration {
        let volume = option_env!("VOLUME")
            .unwrap()
            .parse::<f32>()
            .unwrap_or(1.0);

        Configuration {
            volume,
        }
    }

    pub fn play_sound() {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let file_path: String = super::take_args::get_path();
        let file = File::open(file_path).unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();

        let configuration = extract_configuration();

        sink.set_volume(configuration.volume);
        sink.append(source);
        sink.sleep_until_end();
    }
}