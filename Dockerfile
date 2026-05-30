FROM ghcr.io/pyo3/maturin AS base

WORKDIR /wkbparse
COPY pyproject.toml poetry.lock README.md Cargo.toml /wkbparse/
COPY ./src /wkbparse/src
COPY ./wkbparse /wkbparse/wkbparse
COPY ./tests /wkbparse/tests
COPY ./scripts /wkbparse/scripts

RUN curl -fsSL https://www.sqlite.org/2024/sqlite-autoconf-3460000.tar.gz | tar xz \
    && cd sqlite-autoconf-3460000 && ./configure --prefix=/usr/local && make -j$(nproc) && make install \
    && cd .. && rm -rf sqlite-autoconf-3460000
RUN maturin build --all-features

FROM base AS test


COPY ./tox.ini /wkbparse/
RUN pip3 install tox && chmod +x /wkbparse/scripts/test.sh
ENTRYPOINT [ "/bin/bash", "/wkbparse/scripts/test.sh" ]
