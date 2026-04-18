#!/bin/bash

# This script sets up the prerequisites for the project, including
# the required toolchains and dependencies.

# 1. Installs Rust toolchain if not already installed
if ! command -v rustc &> /dev/null
then
    # https://rust-lang.org/tools/install/
    echo "Rust is not installed. Installing Rust..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
    source $HOME/.cargo/env
else
    rustc --version
    cargo --version
fi
