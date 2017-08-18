#!/bin/bash

MY_DIR=$(dirname $0)

# export CARGO_INCREMENTAL=1
watchexec -f '*.html' -f '*.css' "$MY_DIR/reload-browser.sh"
