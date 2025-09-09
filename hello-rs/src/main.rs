fn main() {
    if let Err(e) = real_main() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}

fn real_main() -> Result<(), String> {
    let mut args = std::env::args();
    let maybe_name = args.nth(1);

    match maybe_name {
        Some(name) => {
            println!("Hello, {name}!");
            Ok(())
        }
        None => Err("Please provide a name as a command-line argument.".to_string()),
    }
}
