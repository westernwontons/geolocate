pub mod get_location {

  use clap::Parser;
  use serde_json::{from_str, to_string_pretty, Map, Value};
  use std::borrow::Cow;

  #[derive(Parser, Debug)]
  #[clap(name = "Geolocate")]
  #[clap(version = "0.2.0")]
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

  /// Fetches the JSON response and returns that or any errors that might have occured
  fn get_location<'a>(api_key: &str, ip: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = format!(
      "https://api.ipgeolocation.io/ipgeo?apiKey={}&ip={}",
      api_key, ip
    );

    match reqwest::blocking::get(url)?.text() {
      Ok(response) => {
        let serialized: Map<String, Value> = from_str(&response)?;
        Ok(to_string_pretty(&serialized)?)
      }

      Err(err) => Err(err.into()),
    }
  }

  /// Shows where the given IP address points to
  pub fn ip_points_to<'a>() -> Result<Cow<'a, str>, Box<dyn std::error::Error>> {
    let args = Args::parse();

    // if the GEO_TOKEN env is not set, get it from args
    match args.api_key {
      None => match std::env::var("GEO_TOKEN").ok() {
        Some(api_key_from_env) => Ok(get_location(&api_key_from_env, &args.ip)?.into()),
        None => Ok("API key is not set in env. Get it from `https://ipgeolocation.io`".into()),
      },
      Some(api_key_from_args) => Ok(get_location(&api_key_from_args, &args.ip)?.into()),
    }
  }
}
