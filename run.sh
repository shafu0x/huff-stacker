#!/bin/bash

PATH_IN=$HOME/huff-stacker/macro.huff
PATH_OUT=$HOME/huff-stacker/out.huff

cargo run $PATH_IN $PATH_OUT && cat out.huff 
