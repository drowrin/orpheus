FROM lukemathwalker/cargo-chef:latest-rust-latest AS chef
WORKDIR /orpheus

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /orpheus/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim AS runtime
WORKDIR /orpheus
VOLUME [ "/orpheus/generated" ]
RUN apt-get update && apt-get install dumb-init
COPY --from=builder /orpheus/target/release/orpheus /usr/local/bin/
ENTRYPOINT ["dumb-init", "--", "/usr/local/bin/orpheus"]