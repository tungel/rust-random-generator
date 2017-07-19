echo "Installing rustup... and set default toolchain to $TRAVIS_RUST_VERSION"
curl https://sh.rustup.rs -sSf | sh -s -- -y --default-toolchain=$TRAVIS_RUST_VERSION
rustc -V
cargo -V

echo "Adding $TARGET target..."
rustup target add $TARGET
