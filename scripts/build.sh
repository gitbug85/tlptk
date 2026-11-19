# Compile binaries into bin
cargo build --manifest-path ./pdf/Cargo.toml --target-dir ./bin
cp ./bin/debug/pdf ./bin/pdf
rm -r ./bin/debug
rm ./bin/.rustc_info.json
