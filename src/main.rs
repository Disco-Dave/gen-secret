use rand::{distributions::Alphanumeric, thread_rng, Rng};
use structopt::StructOpt;

/// Generate a cryptographically strong secret.
#[derive(StructOpt, Debug)]
#[structopt()]
struct Options {
    /// Length of the secret to generate.
    #[structopt(name = "LENGTH", default_value = "64")]
    length: usize,
}

fn generate_secret(length: usize) -> String {
    thread_rng()
        .sample_iter(Alphanumeric)
        .take(length)
        .collect()
}

fn main() {
    let Options { length } = Options::from_args();
    println!("{}", generate_secret(length));
}
