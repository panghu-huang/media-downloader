FROM rust:1.80 AS rust_builder

WORKDIR /home/working

COPY ./.cargo ./.cargo
COPY ./Cargo.toml ./Cargo.toml
COPY ./Cargo.lock ./Cargo.lock
COPY ./crates ./crates
COPY ./tests ./tests

RUN cargo build -p gateway --release

FROM node:20.15.1 AS node_builder

ENV NODE_ENV=production
WORKDIR /home/working

COPY ./package.json ./package.json
COPY ./pnpm-workspace.yaml ./pnpm-workspace.yaml
COPY ./pnpm-lock.yaml ./pnpm-lock.yaml
COPY ./packages ./packages

RUN npm install -g pnpm@9.5.0

RUN pnpm install
RUN pnpm --filter media-downloader run build

FROM node:20.15.1

WORKDIR /home/working

COPY --from=rust_builder /home/working/target/release/gateway ./media-downloader
COPY --from=node_builder /home/working/packages/media-downloader/dist ./web
COPY ./scripts/entrypoint.sh ./entrypoint.sh

RUN chmod +x ./entrypoint.sh

ENTRYPOINT ["./entrypoint.sh"]
