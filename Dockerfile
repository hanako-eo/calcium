FROM rustlang/rust:nightly

# copy the project in the container
COPY . /home
WORKDIR /home

RUN rm -rf /home/target

# update to the latest nightly version and setup the environment
RUN apt update
RUN yes | apt install qemu-kvm qemu
RUN rustup update nightly
RUN rustup target add thumbv7em-none-eabihf
RUN rustup component add rust-src llvm-tools-preview
RUN cargo install bootimage
