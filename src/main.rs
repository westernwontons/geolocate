use clap::Parser;
use serde_json::Value;

#[derive(Parser, Debug)]
#[clap(name = "Geolocator")]
#[clap(version = "0.1.0")]
#[clap(author = "Nagy Botond")]
#[clap(about = "Fetches geolocation data", long_about = None)]
struct Args {
    /// The IP you want to fetch geolocation data about
    #[clap(index = 1)]
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

    match reqwest::get(url).await {
        Ok(response) => {
            let serialized: serde_json::Map<String, Value> =
                serde_json::from_str(&response.text().await?)?;
            let printable = serde_json::to_string_pretty(&serialized).ok();
            println!(
                "{}",
                printable.unwrap_or("String prettifying failed".to_string())
            );
            return Ok(());
        }
        Err(error) => {
            eprintln!("{}", error);
            return Ok(());
        }
    };
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.api_key {
        Some(key_from_arg) => {
            get_location(key_from_arg, args.ip).await?;
            Ok(())
        }

        None => match std::env::var("GEO_TOKEN") {
            Ok(key_from_env) => {
                get_location(key_from_env, args.ip).await?;
                Ok(())
            }

            Err(msg) => {
                eprintln!("{}", msg);
                Ok(())
            }
        },
    }
}
