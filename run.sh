#!/bin/bash
cargo run /Users/shafu/huff-stack-generator/macro.huff /Users/shafu/huff-stack-generator/out.huff

if [ $? -eq 0 ]; then
  cat out.huff
else
  echo "Failure"
fi

