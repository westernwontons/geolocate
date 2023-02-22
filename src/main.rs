mod geolocate_lib;

use geolocate_lib::run::run;

fn main() -> anyhow::Result<()> {
    run()
}
