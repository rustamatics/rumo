#!/bin/bash

echo 'Clearing Scrollback'
clear
printf '\e]50;ClearScrollback\a'

if [ -z "$1" ]; then
  cargo run
else
  cargo run -- $@
fi
