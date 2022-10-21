#!/bin/bash

shopt -s extglob
cargo tarpaulin --verbose \
    --exclude-files \
        src/services/*.rs \
        src/services/*/!(logic) \
        src/services/*/logic/factory.rs \
        src/!(utils|services) \
    --timeout 120 --out Xml --skip-clean --output-dir ./target/tarpaulin/