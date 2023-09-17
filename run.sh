#!/bin/bash
cargo run /home/shafu/huff-stack-generator/macro.huff /home/shafu/huff-stack-generator/out.huff

if [ $? -eq 0 ]; then
  cat out.huff
else
  echo "Failure"
fi
