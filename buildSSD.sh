#!/bin/bash


cargo build --release --target x86_64-unknown-linux-musl
cp /home/louro/Documents/SSD/Project/G1P2P/target/x86_64-unknown-linux-musl/release/G1P2P  ../SSDbins/G1P2P
cp /home/louro/Documents/SSD/Project/G1P2P/target/x86_64-unknown-linux-musl/release/G1P2P.d  ../SSDbins/G1P2P.d






