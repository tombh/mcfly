extern crate bash_wizard;

use bash_wizard::history::History;
use bash_wizard::settings::Settings;
use bash_wizard::settings::Mode;

fn handle_addition(settings: &Settings, history: &mut History) {
    println!("Implement me!");
}

fn handle_search(settings: &Settings, history: &History) {
    for history_match in history.find_matches(&settings.command) {
        println!("{} - {} - {}", history_match.id, history_match.rank, history_match.cmd);
    }
}

fn main() {
    let settings = Settings::parse_args();

    let mut history = History::load();

    println!("{}", settings.command);

    match settings.mode {
        Mode::Add => {
            handle_addition(&settings, &mut history);
        },
        Mode::Search => {
            handle_search(&settings, &history);
        }
    }
}
