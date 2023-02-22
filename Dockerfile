#FROM ros:foxy as base
FROM microros/base:foxy
ARG DEBIAN_FRONTEND=noninteractive

# Install dependencies
RUN apt-get update && apt-get install -y \
    curl \
    git \
    libclang-dev \
    tmux \
    python3-pip \
    && rm -rf /var/lib/apt/lists/*

# Install Rust and the cargo-ament-build plugin
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- --default-toolchain 1.63.0 -y
ENV PATH=/root/.cargo/bin:$PATH
RUN cargo install cargo-ament-build

RUN pip install --upgrade pytest 

# Install the colcon-cargo and colcon-ros-cargo plugins
RUN pip install git+https://github.com/colcon/colcon-cargo.git git+https://github.com/colcon/colcon-ros-cargo.git

WORKDIR /

RUN apt-get update && apt-get -y install curl git python3 python3-pip g++ tmux software-properties-common unzip

RUN echo "set -o vi" >> /root/.bashrc
RUN echo "export TERM=screen-256color-bce" >> /root/.bashrc

# Get tmux config
RUN curl https://raw.githubusercontent.com/xRetry/arch-setup/main/roles/packages/files/tmux.conf > /root/.tmux.conf

RUN curl -LO https://github.com/neovim/neovim/releases/latest/download/nvim.appimage
RUN chmod u+x nvim.appimage
RUN ./nvim.appimage --appimage-extract
RUN ln -s /squashfs-root/AppRun /usr/bin/nvim

# Install Packer
RUN git clone --depth 1 https://github.com/wbthomason/packer.nvim /root/.local/share/nvim/site/pack/packer/start/packer.nvim

# Create directory for Neovim configuration files.
RUN mkdir -p /root/.config/nvim
RUN git clone https://github.com/xRetry/nvim.git /root/.config/nvim

# Install Neovim extensions.
RUN nvim --headless +PackerSync +sleep10 +qall

# Project specific installs
RUN apt-get -y install cargo

# Install Neovim extensions.
RUN nvim --headless +'TSInstall rust' +'MasonInstall rust-analyzer' +'sleep 10' +qall

# Create directory for projects (there should be mounted from host).
RUN mkdir -p /ws

# Set default location after container startup.
WORKDIR /ws
