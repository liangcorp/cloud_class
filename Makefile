format:
	leptosfmt src/*
	rustfmt --edition 2021 src/*.rs
	rustfmt --edition 2021 src/**/*.rs
	rustfmt --edition 2021 src/**/**/*.rs
	rustfmt --edition 2021 src/**/**/**/*.rs

fmt:
	leptosfmt src/*
	rustfmt --edition 2021 src/*.rs
	rustfmt --edition 2021 src/**/*.rs
	rustfmt --edition 2021 src/**/**/*.rs
	rustfmt --edition 2021 src/**/**/**/*.rs

serve:
	export LEPTOS_SASS_VERSION=1.71.0
	cargo leptos serve

watch:
	export LEPTOS_SASS_VERSION=1.71.0
	cargo leptos watch

clean:
	cargo clean
