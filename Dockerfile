FROM rust:1.89.0-bookworm AS build-stage

RUN rustup target add wasm32-unknown-unknown && \
    cargo install trunk && \
    curl -fsSL https://deb.nodesource.com/setup_22.x | bash - && \
    apt-get update && \
    apt-get install -y nodejs

ADD . /sources

WORKDIR /sources

RUN npm install && \
    trunk build --release

FROM nginx:1.28.0-bookworm

COPY --from=build-stage /sources/dist /usr/share/nginx/html

EXPOSE 80

ADD nginx/default.conf /etc/nginx/conf.d/default.conf

RUN echo "{\"management_url\": \"/connector\", \"api_key\": \"123456\"}" >> /usr/share/nginx/html/configuration.json
