LEPTOS=cargo leptos

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
	export LEPTOS_WASM_OPT_VERSION=version_119
	$(LEPTOS) serve

watch:
	export LEPTOS_SASS_VERSION=1.71.0
	export LEPTOS_WASM_OPT_VERSION=version_119
	$(LEPTOS) watch

serve_release:
	export LEPTOS_SASS_VERSION=1.71.0
	export LEPTOS_WASM_OPT_VERSION=version_119
	$(LEPTOS) serve --release

build_release:
	export LEPTOS_SASS_VERSION=1.71.0
	export LEPTOS_WASM_OPT_VERSION=version_119
	$(LEPTOS) build --release

clean:
	cargo clean
