#!/bin/bash

if [[ $(uname) == "Linux" ]]; then
  PATH_IN=/home/shafu/huff-stack-generator/macro.huff
  PATH_OUT=/home/shafu/huff-stack-generator/out.huff
else # for mac
  PATH_IN=/Users/shafu/huff-stack-generator/macro.huff
  PATH_OUT=/Users/shafu/huff-stack-generator/out.huff
fi

cargo run $PATH_IN $PATH_OUT

if [ $? -eq 0 ]; then
  cat out.huff
else
  echo "Failure"
fi
