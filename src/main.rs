use cdt_rs::Config;

fn main() {
    let config = Config::build();

    if let Err(e) = cdt_rs::run(config) {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}
