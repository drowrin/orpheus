
@generate-folders:
    mkdir -p ./generated/posts/
    mkdir -p ./generated/static/
    cp ./web/*.js ./generated/static/
    cp ./content/favicon.ico ./generated/static/favicon.ico 2>/dev/nul \
        || echo -e "\033[91mMissing content/favicon.ico\033[0m"

@tailwind: generate-folders
    tailwindcss -m -i ./web/styles.css -o ./generated/static/styles.css

@pandoc: generate-folders
    for file in `ls ./content/posts/`; do \
        pandoc --from commonmark+attributes-smart+yaml_metadata_block+implicit_figures -o "./generated/posts/${file%.md}.html" "./content/posts/$file"; \
    done

@metadata:
    cargo run --manifest-path lyre/Cargo.toml 

@prepare: pandoc tailwind metadata

@run options="": prepare
    ORPHEUS_OPTIONS={{options}} cargo shuttle run --external

@dev: (run "live_reload,no_cache,simulate_lag")

@watch:
    cargo watch -cq -- just dev

@run-release: prepare
    cargo shuttle run --release

@deploy args="": prepare
    cargo shuttle deploy {{args}}
