use geolocate_lib::run::run;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    run().await
}
