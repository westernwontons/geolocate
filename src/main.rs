use clap::Parser;
use serde_json::Value;

#[derive(Parser, Debug)]
#[clap(name = "Geolocator")]
#[clap(version = "0.1.0")]
#[clap(author = "Nagy Botond")]
#[clap(about = "Fetches geolocation data", long_about = None)]
struct Args {
    /// The IP you want to fetch geolocation data about
    #[clap(short = 'i', long = "ip")]
    ip: String,

    /// Your API key from `https://ipgeolocation.io`
    #[clap(short = 'k', long = "key")]
    api_key: Option<String>,
}

async fn get_location(api_key: String, ip: String) -> Result<(), Box<dyn std::error::Error>> {
    let url = format!(
        "https://api.ipgeolocation.io/ipgeo?apiKey={}&ip={}",
        api_key, ip
    );

    let response: String = reqwest::get(url).await?.text().await?;

    let serialized: serde_json::Map<String, Value> = serde_json::from_str(&response).unwrap();

    println!("{}", serde_json::to_string_pretty(&serialized).unwrap());

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.api_key {
        Some(key_from_arg) => {
            get_location(key_from_arg, args.ip).await?;
            return Ok(());
        }
        None => match std::env::var("GEO_TOKEN") {
            Ok(key_from_env) => {
                get_location(key_from_env, args.ip).await?;
                return Ok(());
            }
            Err(msg) => {
                println!("{}", msg);
                return Ok(());
            }
        },
    }

    // match std::env::var("GEO_TOKEN") {
    //     Ok(key) => {
    //         args.api_key = Some(key);
    //         get_location(args.api_key.unwrap(), args.ip).await?;
    //         return Ok(());
    //     }
    //     Err(_) => {
    //         // println!(
    //         //     "`{}`. Make sure you set GEO_TOKEN or supply an API key",
    //         //     msg
    //         // );

    //         let try_key = args.api_key.unwrap_or(String::from(""));
    //         println!("{}", try_key);
    //         return Ok(());
    //         // if try_key == None {
    //         //     println!("Please provide an API key or supply a GEO_TOKEN environment variable");
    //         //     return Ok(());
    //         // }
    //         get_location(try_key, args.ip).await?;
    //         Ok(())
    //     }
    // }
}
