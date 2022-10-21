#open project folder
cd simple-program

# add arm64 target
rustup target add aarch64-unknown-linux-gnu

#build for Amazon Linux 2 (arm64 target architecture) 
cargo zigbuild --target aarch64-unknown-linux-gnu --release

# copy to root as and rename to bootstrap
cp target/aarch64-unknown-linux-gnu/release/simple-program bootstrap

#deploy to aws
cd ../
sam deploy --guided

