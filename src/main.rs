mod lib;

fn main() {
    lib::playing_sound::play_sound_sink();
    /*
    if result.is_err() {
        eprintln!("Error: Playing sound failed.");
    }
    */
}
