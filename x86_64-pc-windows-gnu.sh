#!/bin/sh

apk update && apk add --assume-yes --no-install-recommends libclang-10-dev clang-10 libclang-cpp10
