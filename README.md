# Geolocate

## Install

If you have `cargo` installed, you can `cargo install --git https://github.com/westernwontons/geolocate`. I would also recommend [topgrade](http://github.com/r-darwish/topgrade) if you haven't heard of it. 

Give it an IP and get back geolocation data. It uses the https://ipgeolocation.io API to fetch the geodata.
You need an API key and there's a free tier. Save the API key into an environment variable named `GEO_DATA` and `geolocate` will read it from there.

If you have `jq` on your system, you can pipe the returned JSON there for pretty colors. If you wish to do this, paste this into your `zshrc, bashrc` or whichever shell you use:
```bash
# geolocate #
geoloc() {
	if  [[ -z $1 ]]; then
		geolocate --help

	else
		geolocate "$@" 2>&1 | jq
	fi
}
```

### ip2location.io support

I received a feature request on Reddit and since I would have wanted to integrate other services as well, it's time to do that!
Since probably this won't be the last of API's that `geolocate` will support, it has become ripe for an internal redesign.
