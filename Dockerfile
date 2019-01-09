FROM rust:latest

RUN apt-get update && apt-get install clang libclang-dev

