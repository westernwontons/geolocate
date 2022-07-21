#![feature(box_into_inner)]
pub mod location;

use crate::location::get_location::ip_points_to;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
  println!("{}", ip_points_to().await?);
  Ok(())
}
