#[derive(Debug)]
pub struct Config {
    pub path: String,
    pub query: String,
    pub ignore_case: bool,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Self, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].to_owned();
        let path = args[2].to_owned();

        let ignore_case = args.iter().any(|arg| arg == "--ignore-case" || arg == "-i");

        Ok(Config {
            path,
            query,
            ignore_case,
        })
    }
}
