mod lib;

fn main() {
    lib::playing_sound::play_sound();
    // TODO: Improve error handling to show actual error messages.
    /*
    if result.is_err() {
        eprintln!("Error: Playing sound failed.");
    }
    */
}
