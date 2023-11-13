fn main() {
    if let Err(e) = deptr::get_args().and_then(deptr::run) {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}
