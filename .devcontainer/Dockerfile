FROM mcr.microsoft.com/devcontainers/rust:0-1-bullseye

USER vscode

RUN bash -c "$(curl -sSfL https://release.solana.com/v1.16.0/install)"
RUN echo 'PATH="/home/vscode/.local/share/solana/install/active_release/bin:$PATH"' >> ~/.zshrc

RUN cargo install --git https://github.com/coral-xyz/anchor avm --locked --force
RUN avm install latest
RUN avm use latest

RUN bash -c "$(curl -sSfL https://raw.githubusercontent.com/nvm-sh/nvm/v0.39.3/install.sh)"
RUN echo 'export NVM_DIR="$HOME/.nvm"' >> ~/.zshrc
RUN echo '[ -s "$NVM_DIR/nvm.sh" ] && \. "$NVM_DIR/nvm.sh"' >> ~/.zshrc

RUN zsh -c "source  ~/.zshrc && nvm install node && nvm use node"
