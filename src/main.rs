mod lib;

use crate::lib::{get_dhash, hamming_distance};

use ::structopt::StructOpt;

use std::path::PathBuf;

use std::env;

#[derive(Debug, StructOpt)]
#[structopt(name = "dhash", about = "dhash image generator")]
struct ConfigContext {
    #[structopt(parse(from_os_str))]
    input: PathBuf,

    #[structopt(parse(from_os_str))]
    compare: Option<PathBuf>,
}

fn main() {
    if let Err(_) = env::var("RUST_LOG") {
        env::set_var("RUST_LOG", "dhash=DEBUG");
    }

    pretty_env_logger::init_timed();

    let context = ConfigContext::from_args();

    let img = image::open(&context.input).expect("Could not open image");

    let input_dhash = get_dhash(&img);

    println!("dhash for {} is `{}`", context.input.display(), input_dhash);

    if let Some(compare) = context.compare {
        let compare_img = image::open(&compare).expect("Could not open compare image");

        let compare_dhash = get_dhash(&compare_img);

        println!("dhash for {} is `{}`", compare.display(), compare_dhash);

        println!(
            "distance is: {}",
            hamming_distance(input_dhash, compare_dhash)
        );
    }
}
