@lyre:
    cargo run --manifest-path lyre/Cargo.toml --features binary-deps --bin lyre

@run options="": lyre
    cargo shuttle run {{options}}

@dev: lyre
    ORPHEUS_OPTIONS="live_reload,no_cache,simulate_lag" cargo shuttle run --external

@watch:
    cargo watch -cq -- just dev

@deploy args="": lyre
    cargo shuttle deploy {{args}}
