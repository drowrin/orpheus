@clean:
    rm -rf generated

@lyre:
    cargo run -p lyre --profile release

@run options="": lyre
    cargo run {{ options }}

@dev: lyre
    ORPHEUS_OPTIONS="live_reload,no_cache,simulate_lag" cargo run

@watch:
    cargo watch -cq -- just dev

@deploy args="": lyre
    cargo shuttle deploy {{ args }}
