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

    pub fn get_path() -> Vec<String> {
        let args = Args::parse();
        let mut vec: Vec<String> = Vec::new();
        if is_directory(&args.path) {
            vec = get_all_files(PathBuf::from(args.path.to_string()))
            return vec;
        } else {
            vec.push(args.path);
            return vec;
        }
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

    pub fn add_to_sink(paths: Vec<String>, sink: &Sink) {

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
