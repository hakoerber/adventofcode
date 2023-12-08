#/usr/bin/env bash

set -o nounset
set -o errexit

for day in ./day* ; do
  (cd "${day}" && cargo build --release)
done

run() {
  echo "day1"
  ./day1/target/release/day1

  echo "day2"
  ./day2/target/release/day2

  echo "day3"
  ./day3/target/release/day3

  echo "day4"
  ./day4/target/release/day4

  echo "day5"
  ./day5/target/release/day5 1 
  ./day5/target/release/day5 2 ranges

  echo "day6"
  ./day6/target/release/day6 1 quadraticformula
  ./day6/target/release/day6 2 quadraticformula

  echo "day7"
  ./day7/target/release/day7 1
  ./day7/target/release/day7 2

  echo "day8"
  ./day8/target/release/day8 1
  ./day8/target/release/day8 2
}

time run
