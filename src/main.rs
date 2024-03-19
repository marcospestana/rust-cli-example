use clap::{App, Arg, ColorChoice};
use blkrs::run_lsblk;

fn main() {
    let matches = App::new("lsblk")
        .version("0.0.1")
        .author("Alfredo Deza")
        .about("lsblk in Rust")
        .color(ColorChoice::Always)
        .arg(
            Arg::new("device")
                .help("Device to query")
                .required(true)
                .index(1)
        )
        .get_matches();

    if let Some(device) = matches.value_of("device") {
        let output = serde_json::to_string(&run_lsblk(device)).unwrap();
        println!("{}", output);
    } else {
        println!("No device provided");
    }
}
