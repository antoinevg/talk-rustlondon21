std:
	cargo build --example $(example)
	leaks --atExit -- target/debug/examples/$(example)

no_std:
	cargo build --example $(example) --no-default-features --features="alloc, no_std"
	leaks --atExit -- target/debug/examples/$(example)

qemu:
	cargo +nightly run --example $(example) --target thumbv7em-none-eabihf --no-default-features --features="qemu"

atsame54:
	cargo +nightly run --example $(example) --target thumbv7em-none-eabihf --no-default-features --features="atsame54"

stm32f4:
	cargo +nightly run --example $(example) --target thumbv7em-none-eabihf --no-default-features --features="stm32f4"

wasm:
	cargo build --target=wasm32-unknown-unknown --no-default-features --features="alloc, no_std"
