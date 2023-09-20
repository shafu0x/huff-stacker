#!/bin/bash

PATH_IN=$HOME/huff-stacker/macro.huff
PATH_OUT=$HOME/huff-stacker/out.huff

cargo run $PATH_IN $PATH_OUT

if [ $? -eq 0 ]; then
  cat out.huff
else
  echo "Failure"
fi
