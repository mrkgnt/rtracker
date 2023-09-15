use clap::{self, command, Arg};

fn main() {
    let match_result = command!()
    .about("CLI tool to track ongoing tasks.")
    .arg(
        Arg::new("firstname")
        .short('f')
    )
    .get_matches();
}
