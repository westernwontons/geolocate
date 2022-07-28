pub mod location;

use crate::location::get_location::ip_points_to;

// #[tokio::main]
fn main() -> reqwest::Result<()> {
  match ip_points_to() {
    Ok(ip_points_to) => {
      println!("{}", ip_points_to);
      Ok(())
    }
    Err(err) => {
      eprintln!("{}", err.without_url());
      Ok(())
    }
  }
}
