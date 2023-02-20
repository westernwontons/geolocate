alias r := run
alias b := build
alias h := help
alias t := test

run:
	cargo run

build:
	cargo build

help:
	cargo run -- help

test:
	cargo test -p geolocate_lib -- --show-output