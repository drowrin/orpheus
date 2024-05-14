@clean:
    rm -rf generated

@lyre: clean
    cargo run -p lyre --features binary-deps

@run options="": lyre
    cargo shuttle run {{ options }}

@dev: lyre
    ORPHEUS_OPTIONS="live_reload,no_cache,simulate_lag" cargo shuttle run --external

@watch:
    cargo watch -cq -- just dev

@deploy args="": lyre
    cargo shuttle deploy {{ args }}
