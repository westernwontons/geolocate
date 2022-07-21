alias w := watch
alias t := test

watch:
	cargo watch -s "cargo run | bat -pl rs"

test:
	cargo test
