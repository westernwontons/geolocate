pub mod get_location {

  use std::borrow::Cow;

  use clap::Parser;
  use serde_json::{from_str, to_string_pretty, Map, Value};

  #[derive(Parser, Debug)]
  #[clap(name = "Geolocate")]
  #[clap(version = "0.2.0")]
  #[clap(author = "Nagy Botond")]
  #[clap(about = "Fetches geolocation data", long_about = None)]
  pub struct Args {
    /// The IP you want to fetch geolocation data about
    #[clap(index = 1)]
    pub ip: String,

    /// Your API key from `https://ipgeolocation.io`
    #[clap(short = 'k', long = "key")]
    pub api_key: Option<String>,
  }

  /// Fetches the JSON response or prints an error message if any
  async fn get_location<'a>(
    api_key: &str,
    ip: &str,
  ) -> Result<Cow<'a, str>, Box<dyn std::error::Error>> {
    let url = format!(
      "https://api.ipgeolocation.io/ipgeo?apiKey={}&ip={}",
      api_key, ip
    );

    match reqwest::get(&url).await?.text().await {
      Ok(response) => {
        let serialized: Map<String, Value> = from_str(&response)?;

        match to_string_pretty(&serialized) {
          Ok(printable) => return Ok(Cow::Owned(printable)),
          Err(_) => return Ok(Cow::Borrowed(r#"¯\_(ツ)_/¯"#)),
        }
      }

      Err(_) => {
        return Ok(Cow::Borrowed(
          r#"
        ¯\_(ツ)_/¯ at mod location, line 42
        It's likely that something went wrong with the request.
        "#,
        ));
      }
    };
  }

  /// Returns the `GEO_TOKEN` environment variable if it exists, otherwise None.
  /// TODO: On the first run, offer to open the website and save the token to a config file instead of an env
  fn get_api_key_from_env() -> Option<String> {
    match std::env::var("GEO_TOKEN") {
      Ok(geo_token) => Some(geo_token),
      Err(_) => None,
    }
  }

  /// Shows where the given IP address points to
  pub async fn ip_points_to() -> Result<Cow<'static, str>, Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.api_key {
      None => match get_api_key_from_env() {
        Some(api_key_from_env) => Ok(get_location(&api_key_from_env, &args.ip).await?),
        None => Ok(Cow::Borrowed(
          "API key is not set in env. Get it from `https://ipgeolocation.io`",
        )),
      },
      Some(api_key_from_args) => Ok(get_location(&api_key_from_args, &args.ip).await?),
    }
  }
}
