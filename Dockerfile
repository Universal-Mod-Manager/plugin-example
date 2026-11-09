# Builds and tests the plugin without installing Node, extism-js or Binaryen locally.
# `docker build --output dist .` writes a ready-to-install plugin folder to dist/.
FROM node:24.22.2-trixie-slim AS build
ARG EXTISM_JS_VERSION=v1.7.0
ARG BINARYEN_VERSION=133
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*
RUN curl -fsSL "https://github.com/extism/js-pdk/releases/download/${EXTISM_JS_VERSION}/extism-js-x86_64-linux-${EXTISM_JS_VERSION}.gz" \
    | gunzip > /usr/local/bin/extism-js \
    && chmod +x /usr/local/bin/extism-js
RUN curl -fsSL "https://github.com/WebAssembly/binaryen/releases/download/version_${BINARYEN_VERSION}/binaryen-version_${BINARYEN_VERSION}-x86_64-linux.tar.gz" \
    | tar -xz -C /opt \
    && ln -s /opt/binaryen-version_${BINARYEN_VERSION}/bin/* /usr/local/bin/
WORKDIR /plugin
COPY package.json package-lock.json ./
RUN npm ci
COPY . .
RUN npm run typecheck && npm run build && npm test

FROM scratch
COPY --from=build /plugin/build/plugin.wasm /plugin.wasm
COPY metadata.json icon.png /
