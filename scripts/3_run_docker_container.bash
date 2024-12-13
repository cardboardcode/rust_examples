#!/usr/bin/env bash

xhost +local:docker

docker run -it --rm \
    --name rust_examples_c \
    --network host \
    -e DISPLAY=$DISPLAY \
    -v /tmp/.X11-unix:/tmp/.X11-unix \
rust_examples:slim-buster bash
