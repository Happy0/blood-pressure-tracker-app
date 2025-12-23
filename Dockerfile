FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder
COPY --from=planner /app/recipe.json recipe.json
RUN apt update && apt install --assume-yes libopencv-dev clang libclang-dev libopencv-imgcodecs-dev llvm-dev 
# Build dependencies - this is the caching Docker layer!
RUN cargo chef cook --release --recipe-path recipe.json
# Build application
COPY . .
COPY client /app/target/release
COPY ./entrypoint.sh /app/target/release/entrypoint.sh
RUN cargo build --release --bin blood-pressure-tracker-app

# We do not need the Rust toolchain to run the binary!
FROM debian:trixie AS runtime
WORKDIR /app
RUN apt update && apt install --assume-yes libopencv-dev clang libclang-dev libopencv-imgcodecs-dev llvm-dev zip sqlite3

ENV LD_LIBRARY_PATH=${LD_LIBRARY_PATH}:/usr/lib:/lib:/lib/x86_64-linux-gnu

COPY --from=builder /app/target/release/blood-pressure-tracker-app /usr/local/bin
COPY --from=builder /app/client/dist /usr/local/bin/client
COPY --from=builder /app/target/release/entrypoint.sh /usr/local/bin/entrypoint.sh

WORKDIR /usr/local/bin
RUN chmod +x entrypoint.sh
ENTRYPOINT ["entrypoint.sh"]

CMD []