FROM gitpod/workspace-base

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y && \
    export PATH="$HOME/.cargo/bin:$PATH" && \
    cargo install diesel_cli --no-default-features --features sqlite && \
    diesel completions bash  > .bash_completion

