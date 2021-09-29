FROM rustlang/rust:nightly-alpine AS build

WORKDIR /usr/src/LoD-backend
COPY . .
RUN --mount=type=secret,id=ROCKET_DATABASES \
    export ROCKET_DATABASES=$(cat /run/secrets/ROCKET_DATABASES) && \
    apk add --no-cach musl-dev && \
    cargo install --path .

# FINAL

FROM rustlang/rust:nightly-alpine

COPY --from=build /usr/local/cargo/bin/lod-backend /usr/local/bin/lod-backend
ENTRYPOINT [ "lod-backend" ]