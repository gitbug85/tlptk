
cleanup_rust() {
    local name="$1"
    cp "./bin/debug/${name}" "./bin/${name}"
    rm -r ./bin/debug
    rm ./bin/.rustc_info.json
}

# Compile binaries into bin
cargo build --manifest-path ./libs/pdf/Cargo.toml --target-dir ./bin
cleanup_rust pdf
nim c --outdir:bin ./libs/get_svg/get_svg.nim
cargo build --manifest-path ./libs/ui/Cargo.toml --target-dir ./bin
cleanup_rust ui
