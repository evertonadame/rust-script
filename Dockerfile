# Use a imagem oficial do Rust como base
FROM rust:latest

# Defina o diretório de trabalho dentro do contêiner
WORKDIR /usr/src/app

# Copie o arquivo Cargo.toml e o arquivo Cargo.lock para o contêiner
COPY Cargo.toml Cargo.lock ./

# Copie todo o restante do projeto para o contêiner
COPY . .

# Compile o projeto
RUN cargo build

# Comando padrão a ser executado quando o contêiner for iniciado
CMD cargo run