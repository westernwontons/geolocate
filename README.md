## Geolocate

Give it an IP and get back geolocation data. It uses the https://ipgeolocation.io API to fetch the geodata.
You need an API key and there's a free tier. Save the API key into an environment variable named GEO_DATA and `geolocate` will read it from there.

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