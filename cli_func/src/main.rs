use clap::Parser;
// use config::Config;
// use std::collections::HashMap;
/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    /// Name of the person to greet
    #[clap(short, long)]
    name: String,

    /// Number of times to greet
    #[clap(short, long, default_value_t = 1)]
    count: u8,
}

fn main() {
    let args = Args::parse();

    for _ in 0..args.count {
        println!("Hello {}!", args.name)
    }

    // let settings = Config::builder()
    //     // Add in `./Settings.toml`
    //     .add_source(config::File::with_name("Settings"))
    //     // Add in settings from the environment (with a prefix of APP)
    //     // Eg.. `APP_DEBUG=1 ./target/app` would set the `debug` key
    //     .add_source(config::Environment::with_prefix("APP"))
    //     .build()
    //     .unwrap();

    // // Print out our settings (as a HashMap)
    // println!(
    //     "{:?}",
    //     settings
    //         .try_deserialize::<HashMap<String, String>>()
    //         .unwrap()
    // );
}
