set dotenv-filename := "secrets.env"

@clean:
    rm -rf generated

@lyre args="build":
    cargo run --release -p lyre {{ args }}

@gen: (lyre "gen")

@dev: lyre
    ORPHEUS_OPTIONS="live_reload,no_cache,simulate_lag" cargo run

@watch recipes="":
    cargo watch -cq -- just {{ recipes }} dev

@run:
    ./target/release/lyre build && ORPHEUS_OPTIONS="live_reload,no_cache" ./target/release/orpheus

@author:
    cargo build -p lyre --release
    cargo build --release
    cargo watch -w content -cq -- just run

@deploy: lyre
    echo "copying files..."
    scp -rq ./generated/* $ORPHEUS_HOST:$ORPHEUS_DIR
    echo "restarting container"
    ssh $ORPHEUS_HOST $ORPHEUS_RESTART
    echo "done"

@update:
    if [[ `git status --porcelain` ]]; then \
        echo "you have local uncomitted changes"; \
        exit 1; \
    fi
    if [[ `git diff origin/main..HEAD` ]]; then \
        echo "you have unpushed commits"; \
        exit 1; \
    fi
    echo "updating from git..."
    ssh $ORPHEUS_HOST $ORPHEUS_UPDATE
    echo "rebuilding..."
    ssh $ORPHEUS_HOST $ORPHEUS_BUILD
    echo "done"
