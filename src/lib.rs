pub mod take_args {
    use clap::Parser;
    use glob::glob;
    use std::env::var;
    use std::fs::metadata;
    use std::path::PathBuf;

    #[derive(Parser)]
    pub struct Args {
        /// The path to the sound file/directory to play.
        #[arg(short, long)]
        pub path: String,
        /// If the imported tracks should be randomised.
        #[arg(short, long)]
        pub random: bool,
    }

    pub struct Configuration {
        pub volume: f32,
        pub is_random: bool,
    }

    pub fn extract_configuration() -> Configuration {
        let volume = var("VOLUME")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(1.0);
        let is_random = get_args().random;
        Configuration { volume, is_random }
    }

    pub fn is_directory(path: &String) -> bool {
        let path = metadata(path);
        let file_type = path.unwrap().file_type();
        return if file_type.is_dir() { true } else { false };
    }

    pub fn get_all_files(dir: PathBuf) -> Vec<PathBuf> {
        let search_glob = glob(format!("{}/**/*.flac", dir.to_str().unwrap()).as_str());
        return search_glob.unwrap().flatten().collect();
    }

    pub fn get_path() -> Vec<PathBuf> {
        let args = get_args();
        let mut vec: Vec<PathBuf> = Vec::new();
        let mut path = PathBuf::new();
        path.push(args.path.clone());
        return if is_directory(&args.path) {
            get_all_files(path)
        } else {
            vec.push(path);
            vec
        };
    }

    pub fn get_args() -> Args {
        let args = Args::parse();
        args
    }
}

pub mod playing_sound {

    use super::take_args::{extract_configuration, Configuration};
    use rand::prelude::SliceRandom;
    use rodio::{Decoder, OutputStream, Sink};
    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;

    pub fn get_source(file_path: &PathBuf) -> Decoder<BufReader<File>> {
        let file = File::open(file_path).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();
        source
    }

    pub fn add_to_sink(config: Configuration, mut paths: Vec<PathBuf>, sink: &Sink) {
        if config.is_random {
            let mut rng = rand::thread_rng();
            paths.shuffle(&mut rng);
        }
        for i in 0..paths.len() {
            sink.append(get_source(&paths[i]))
        }
    }

    pub fn play_sound() {
        let file_path: Vec<PathBuf> = super::take_args::get_path();
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();
        let configuration = extract_configuration();

        sink.set_volume(configuration.volume);
        add_to_sink(configuration, file_path, &sink);
        sink.sleep_until_end();
    }
}
