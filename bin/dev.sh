#!/bin/bash
DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
cargo watch -s "$DIR/cargo_watch.sh"
