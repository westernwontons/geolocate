# Geolocate

Geolocate is a CLI app to fetch geolocation data from various providers. Currently, we support (ip2location)[https://www.ip2location.com/] and (ipgeolocation)[https://ipgeolocation.io/].
If you'd like others to be supported, you're welcome to create an issue.

## Installation

Geolocate compiles on `stable`. There are multiple ways to install it:

- cargo install geolocate
- cargo install --git https://github.com/westernwontons/geolocate

## Usage

You should start with getting some API tokens for at least one of the supported providers. Both (ip2location)[https://www.ip2location.com/] and (ipgeolocation)[https://ipgeolocation.io/]
have a free tier.
You can save them with `geolocate config --edit`. This command will open the configuration file with you favourite editor where you will have to add the tokens.
IMPORTANT: The key of the token has to be name of the provider. For example:

```toml
ip2location = "yourtokengoeshere"
ipgeolocation = "yourothertokengoeshere"
```

If you don't do this, `geolocate` will not recognise them and won't be very useful. Be sure you write correct `toml` data, but

A subcommand exists for each provider. For example, to fetch geo data for an IP address of 1.2.3.4 from the `ipgeolocation` provider, you would run the following:

```bash
geolocate ipgeolocation --addrs 1.2.3.4
```

Multiple IP addresses may be passed delimited with spaces:

```bash
geolocate ipgeolocation --addrs 1.2.3.4 1.2.3.4 1.2.3.4
```

In each case you get a JSON array response.

Geolocate can also read a file that contains IP addresses. There are some rules:

- each IP address has to be delimited by a new line
- they must be correct IP addresses

In the latter case, if the IP address format is wrong, you will get an error with the number of the line where the incorrect IP address is.
The first incorrect IP address will make `geolocate` exit.

The IP addresses may be `IPv4` or `IPv6`. Geolocate will accept either so long they're correct, but makes no guarantees that the provider
accepts them.
