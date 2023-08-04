FROM ghcr.io/pyo3/maturin as base

WORKDIR /wkbparse
COPY pyproject.toml poetry.lock README.md Cargo.toml /wkbparse/
COPY ./src /wkbparse/src
COPY ./wkbparse /wkbparse/wkbparse
COPY ./tests /wkbparse/tests
COPY ./scripts /wkbparse/scripts

RUN maturin build --all-features

FROM base as test

COPY ./tox.ini /wkbparse/
RUN pip3 install tox && chmod +x /wkbparse/scripts/test.sh
ENTRYPOINT [ "/bin/bash", "/wkbparse/scripts/test.sh" ]
