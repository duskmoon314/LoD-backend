FROM rustlang/rust:nightly-alpine AS build

WORKDIR /usr/src/LoD-backend
COPY . .
RUN apk add --no-cach musl-dev && \
    cargo install --path .

# FINAL

FROM rustlang/rust:nightly-alpine

COPY --from=build /usr/local/cargo/bin/lod-backend /usr/local/bin/lod-backend
ENTRYPOINT [ "lod-backend" ]