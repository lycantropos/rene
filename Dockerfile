ARG IMAGE_NAME
ARG IMAGE_VERSION

FROM ${IMAGE_NAME}:${IMAGE_VERSION}

RUN pip install --upgrade pip setuptools

RUN curl -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

WORKDIR /opt/rene

COPY Cargo.toml .
COPY pyproject.toml .
COPY rust-toolchain.toml .
COPY README.md .
COPY setup.py .
COPY rene rene
COPY src src
COPY tests tests

RUN pip install -e .[tests]
