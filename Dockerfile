FROM rust:1.80 AS rust_builder

WORKDIR /home/working

COPY ./.cargo ./.cargo
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./crates ./crates
COPY ./tests ./tests

RUN cargo build -p gateway --release

FROM node:20.15.1

ENV NODE_ENV=production
WORKDIR /home/working

RUN npm install -g pnpm@9.5.0
RUN apt-get update -y && apt-get install -y ffmpeg

COPY ./package.json ./package.json
COPY ./pnpm-workspace.yaml ./pnpm-workspace.yaml
COPY ./pnpm-lock.yaml ./pnpm-lock.yaml
COPY ./packages ./packages

RUN pnpm install
RUN pnpm --filter media-downloader run build

COPY --from=rust_builder /home/working/target/release/gateway ./media-downloader
COPY ./scripts/entrypoint.sh ./entrypoint.sh

RUN chmod +x ./entrypoint.sh

ENTRYPOINT ["./entrypoint.sh"]
