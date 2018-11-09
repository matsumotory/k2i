#/bin/sh

sudo apt-get update
sudo apt-get -y install build-essential rake bison git gperf automake m4 \
                autoconf libtool cmake pkg-config libcunit1-dev ragel libprocps4-dev
sudo apt-get -y upgrade

curl https://sh.rustup.rs -sSf > rustup.sh
sh rustup.sh -y
source $HOME/.cargo/env
rustup update
rustup install nightly
rustup component add rustfmt-preview --toolchain nightly

git clone https://github.com/matsumotory/k2i
cd k2i && cargo build
