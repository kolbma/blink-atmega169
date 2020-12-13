#!/bin/sh

TARGET="avr-unknown-gnu-atmega169.json"
BUILD_STD="core"

target=
build_std=

idx=-1
for x in "$@" ; do
    idx=$((idx+1))
    [ -n "$x" ] && { [ "$x" = "check" ] || [ "$x" = "build" ] || [ "$x" = "clean" ] ; } && {
        target="$TARGET"
        build_std="$BUILD_STD"
        break
    }
done

cmd_args=()

[ "$idx" -ge 0 ] && {
    cmd_args+=(${@:1:$idx+1})
    [ -n "$target" ] && {
        cmd_args+=("--target" "$target")
    }
    [ -n "$build_std" ] && {
        cmd_args+=("-Z" "build-std=$build_std")
    }
    cmd_args+=(${@:$idx+2})
}

exec cargo "${cmd_args[@]}"

exit 0
