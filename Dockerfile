# Builds and tests the plugin without installing extism-py or Binaryen locally.
# `docker build --output dist .` writes a ready-to-install plugin folder to dist/.
FROM python:3.13.17-slim-trixie AS build
ARG EXTISM_PY_VERSION=v0.1.5
ARG BINARYEN_VERSION=133
RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates curl \
    && rm -rf /var/lib/apt/lists/*
# extism-py looks for its Python engine in ~/.local/share/extism-py.
RUN curl -fsSL "https://github.com/extism/python-pdk/releases/download/${EXTISM_PY_VERSION}/extism-py-x86_64-linux-${EXTISM_PY_VERSION}.tar.gz" \
    | tar -xz -C /tmp \
    && mv /tmp/extism-py/bin/extism-py /usr/local/bin/ \
    && mkdir -p /root/.local/share \
    && mv /tmp/extism-py/share/extism-py /root/.local/share/
RUN curl -fsSL "https://github.com/WebAssembly/binaryen/releases/download/version_${BINARYEN_VERSION}/binaryen-version_${BINARYEN_VERSION}-x86_64-linux.tar.gz" \
    | tar -xz -C /opt \
    && ln -s /opt/binaryen-version_${BINARYEN_VERSION}/bin/* /usr/local/bin/
WORKDIR /plugin
COPY requirements-test.txt ./
RUN pip install --no-cache-dir -r requirements-test.txt
COPY . .
RUN mkdir -p build && extism-py src/plugin.py -o build/plugin.wasm && python -m unittest discover -s tests -v

FROM scratch
COPY --from=build /plugin/build/plugin.wasm /plugin.wasm
COPY metadata.json icon.png /
