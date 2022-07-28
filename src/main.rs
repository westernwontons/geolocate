pub mod location;

use crate::location::get_location::ip_points_to;

// #[tokio::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
  match ip_points_to() {
    Ok(ip_points_to) => {
      println!("{}", ip_points_to);
      Ok(())
    }
    Err(err) => {
      eprintln!("{}", err.source().expect(
        "Failed to print source of error. This is likely a bug. Report it at https://github.com/westernwontons/geolocate/issues"
        ));
      Ok(())
    }
  }
}
