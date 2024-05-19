FROM lukemathwalker/cargo-chef:latest-rust-latest AS chef
WORKDIR /orpheus

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
RUN cargo install cargo-shuttle
COPY --from=planner /orpheus/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release
ENTRYPOINT ["cargo", "shuttle", "run", "--release", "--external"]