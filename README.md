# Geolocate

## Overview

The `geolocate` program uses the https://ipgeolocation.io API to fetch geodata of a given IP (IPv4) address. The data returned by the API is in JSON format.

## Install

The easiest way to install the `geolocate` program is via `cargo`. If you have `cargo` installed on your system, you can simply run `cargo install --git https://github.com/westernwontons/geolocate` in a terminal. I would also recommend [topgrade](https://github.com/r-darwish/topgrade) if you haven't heard of it. 

In case you do not have `cargo` on your system, you can get it from the [official site](https://rustup.rs) .

## Usage

To fetch geodata about an IP address you first need to register yourslef at https://ipgeolocation.io/signup.html to receive an API key, there's a free tier. After you have the key open a terminal and type in the follownig command:

`geolocate IP_Address -k API_Token`

In case you do not want to copy - paste the token every time you want to use the program, save the API key into an environment variable named `GEO_DATA` and `geolocate` will read it from there.

If you have `jq` on your system, you can pipe the returned JSON data there for pretty colors: `geolocation IP -k API_Token | jq`. 

If you wish to do this via `bash` function, paste the below lines into your `.zshrc, .bashrc` or whichever shell you use:
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

Save and exit the file. Now you either restart your terminal session or simply reload the \.\*rc file with the `source ~/.*rc` command.
Please note, that for the function to work you have to save your token into the `GEO_DATA` environment variable.

### ip2location.io support

I received a feature request on Reddit and since I would have wanted to integrate other services as well, it's time to do that!
Since probably this won't be the last of API's that `geolocate` will support, it has become ripe for an internal redesign.
