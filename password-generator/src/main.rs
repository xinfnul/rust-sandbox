use password_generator::{cli::parse_args, generator::generate_password};

fn main() {
    match parse_args().and_then(|config| generate_password(&config)) {
        Ok(password) => println!("Password: {}", password),

        Err(error) => {
            eprintln!("Error: {}", error);
            std::process::exit(1);
        }
    }
}
