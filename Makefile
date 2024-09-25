clean:
	leptosfmt src/*
	rustfmt --edition 2021 src/*.rs
	rustfmt --edition 2021 src/**/*.rs
	rustfmt --edition 2021 src/**/**/*.rs
	rustfmt --edition 2021 src/**/**/**/*.rs

serve:
	cargo leptos serve

watch:
	cargo leptos watch
