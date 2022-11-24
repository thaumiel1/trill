pub mod take_args {
    use clap::Parser;
    use std::fs::{FileType, ReadDir, metadata};
    use std::io::Result;
    use std::path::PathBuf;
    use glob::glob;


    #[derive(Parser)]
    pub struct Args {
        /// The path to the sound file to play.
        #[arg(short, long)]
        path: String,
    }

    pub fn is_directory(path: &String) -> bool {
        let path = metadata(path);
        let file_type = path.unwrap().file_type();
        return if file_type.is_dir() {
            true
        } else {
            false
        }
    }

    pub fn get_all_files(dir: PathBuf) -> Vec<PathBuf> {
        let search_glob = glob(format!("{}/**/*.flac", dir.to_str().unwrap()).as_str());
        return search_glob.unwrap().flatten().collect();
    }

    pub fn get_path() -> Vec<PathBuf> {
        let args = Args::parse();
        let mut vec: Vec<PathBuf> = Vec::new();
        let mut path = PathBuf::new();
        path.push(args.path.clone());
        if is_directory(&args.path) {
            vec = get_all_files(path);
            return vec;
        } else {
            vec.push(path);
            return vec;
        }
    }
}

pub mod playing_sound {

    use rodio::{source::Source, Decoder, OutputStream, Sink};
    use std::env::var;
    use std::fs::File;
    use std::io::BufReader;
    use std::path::PathBuf;

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

    pub fn get_source(file_path: &PathBuf) -> Decoder<BufReader<File>> {
        let file = File::open(file_path).unwrap();
        let source = Decoder::new(BufReader::new(file)).unwrap();
        source
    }

    pub fn add_to_sink(paths: Vec<PathBuf>, sink: &Sink) {
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
        add_to_sink(file_path, &sink);
        sink.sleep_until_end();
    }
}
