mod geolocate_lib;

use geolocate_lib::run::run;

fn main() -> anyhow::Result<()> {
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_io()
        .enable_time()
        .build()?;

    rt.block_on(async { run().await })
}
