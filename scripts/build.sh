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

compile_ui() {
    cargo build --manifest-path ./libs/ui/Cargo.toml --target-dir ./bin
    cleanup_rust ui
}

compile_render() {
    g++ ./libs/render/main.cpp -o ./bin/render \
        $(pkg-config --cflags --libs poppler-cpp) \
        -ljpeg
}

compile_lex() {
    nim c -o:./bin/lex ./libs/lex/main.nim
}

if [[ "$INDIVIDUAL" == true ]]; then
    case "$BINARY" in
        ui)      compile_ui ;;
        render)  compile_render ;;
        lex)  compile_lex ;;
        *)
            echo "Usage: $0 -i|--individual <pdf|get_svg|ui|render>"
            exit 1
            ;;
    esac
else
    rm -rf ./bin/*
    compile_ui
    compile_render
    compile_lex
fi
