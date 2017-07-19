# installing rustup
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain=$TRAVIS_RUST_VERSION
# curl https://sh.rustup.rs -sSf | sh -s -- -y
rustc -V
cargo -V

echo "Adding x86_64-unknown-linux-musl target..."
rustup target add $TARGET
