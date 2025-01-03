#!/bin/bash

cargo build -q
mv ./target/debug/ruti ~/.cargo/bin/ruti
