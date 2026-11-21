#!/bin/bash

INDIVIDUAL=false
BINARY=""

while [[ $# -gt 0 ]]; do
    case "$1" in
        -i|--individual)
            INDIVIDUAL=true
            shift
            ;;
        *)
            BINARY="$1"
            shift
            ;;
    esac
done

cleanup_rust() {
    local name="$1"
    cp "./bin/debug/${name}" "./bin/${name}"
    rm -r ./bin/debug
    rm ./bin/.rustc_info.json
}

compile_pdf() {
    cargo build --manifest-path ./libs/pdf/Cargo.toml --target-dir ./bin
    cleanup_rust pdf
}

compile_get_svg() {
    nim c --outdir:bin ./libs/get_svg/get_svg.nim
}

compile_ui() {
    cargo build --manifest-path ./libs/ui/Cargo.toml --target-dir ./bin
    cleanup_rust ui
}

compile_render() {
    g++ ./libs/render/main.cpp -o ./bin/render \
        $(pkg-config --cflags --libs poppler-cpp)
}

if [[ "$INDIVIDUAL" == true ]]; then
    case "$BINARY" in
        pdf)     compile_pdf ;;
        get_svg) compile_get_svg ;;
        ui)      compile_ui ;;
        render)  compile_render ;;
        *)
            echo "Usage: $0 -i|--individual <pdf|get_svg|ui|render>"
            exit 1
            ;;
    esac
else
    compile_pdf
    compile_get_svg
    compile_ui
    compile_render
fi
