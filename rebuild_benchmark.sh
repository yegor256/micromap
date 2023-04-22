#!/bin/bash
set -x
set -e

rm -rf src/bin
mkdir src/bin
cp tests/benchmark.rs src/bin/benchmark.rs

sed -E -i .bak 's/\[dev-dependencies\]//g' Cargo.toml

capacities="2 4"

rm -rf tmp
mkdir tmp
for capacity in ${capacities}; do
  sed -E -i .bak "s/CAPACITY: usize = [0-9]+/CAPACITY: usize = ${capacity}/g" src/bin/benchmark.rs
  cargo build --release
  ./target/release/benchmark 100000 > tmp/${capacity}.out
done

{
  echo -n '| | '
  for capacity in ${capacities}; do
    echo -n "${capacity} | "
  done
  echo ''
  echo -n '| --- | '
  for capacity in ${capacities}; do
    echo -n "--- | "
  done
  echo ''
  maps=$(cut -f 1 tmp/2.out)
  for map in ${maps}; do
    if [ "${map}" -eq "micromap::Map" ]; then
      break
    fi
    echo -n "| \`${map}\` | "
    for capacity in ${capacities}; do
      our=$(grep "micromap::Map" "tmp/${capacity}.out" | cut -f 2)
      their=$(grep "${map}" "tmp/${capacity}.out" | cut -f 2)
      perl -e "printf(\"%.02f | \", ${their} / ${our});"
    done
    echo ''
  done
} > tmp/table.md

perl -e

