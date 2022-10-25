#!/bin/bash
shopt -s expand_aliases
shopt -s extglob
EXC_MODULES="src/services/*.rs src/services/*/!(logic) src/services/*/logic/@(mod|factory).rs src/!(utils|services) src/utils/@(mod|test).rs"

cargo tarpaulin \
    `if [[ $1 == "ci" ]]; then echo "--out Xml"; else echo "--skip-clean --out Html"; fi` \
    --exclude-files echo $EXC_MODULES \
    --timeout 120 --output-dir ./target/tarpaulin/

shopt -u extglob

REST_PATH="$(pwd)/target/tarpaulin/tarpaulin-report.html"
echo 
echo $REST_PATH