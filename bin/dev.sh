#!/bin/bash
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
RUST_BACKTRACE=full RUST_LOG="droid=debug" cargo watch -s "$DIR/cargo.sh"
