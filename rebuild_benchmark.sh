#!/bin/bash
set -x
set -e

rm -rf src/bin
mkdir src/bin
cp tests/benchmark.rs src/bin/benchmark.rs

gsed -E -i 's/\[dev-dependencies\]//g' Cargo.toml

capacities="2 4"

rm -rf tmp
mkdir tmp
for capacity in ${capacities}; do
  gsed -E -i "s/CAPACITY: usize = [0-9]+/CAPACITY: usize = ${capacity}/g" src/bin/benchmark.rs
  cargo build --release
  ./target/release/benchmark 100000 > tmp/${capacity}.out
done

{
  echo -n '| |'
  for capacity in ${capacities}; do
    echo -n " ${capacity} |"
  done
  echo ''
  echo -n '| --- |'
  for capacity in ${capacities}; do
    echo -n " --- |"
  done
  echo ''
  maps=$(cut -f 1 tmp/2.out)
  for map in ${maps}; do
    if [ "${map}" == "micromap::Map" ]; then
      continue;
    fi
    echo -n "| \`${map}\` |"
    for capacity in ${capacities}; do
      our=$(grep "micromap::Map" "tmp/${capacity}.out" | cut -f 2)
      if [ "${our}" -eq "0" ]; then
        our=1
      fi
      their=$(grep "${map}" "tmp/${capacity}.out" | cut -f 2)
      perl -e "printf(\" %.02f |\", ${their} / ${our});"
    done
    echo ''
  done
} > tmp/table.md

perl -e '
  my $readme;
  my $file = "README.md";
  open(my $r, "<", $file);
  { local $/; $readme = <$r>; }
  close($r);
  my $sep = "<!-- benchmark -->";
  my @p = split(/\Q$sep\E/, $readme);
  my $table = "tmp/table.md";
  open(my $t, "<", $table);
  { local $/; $table = <$t>; }
  close($t);
  $p[1] = "\n\n" . $table . "\n\n";
  my $new = join($sep, @p);
  open(my $w, ">", $file);
  print $w join($sep, @p);
  close($w);
'
