#!/usr/bin/env bash

set -o nounset
set -o pipefail

year="${1}";shift
day="${1}";shift

cp -r "./template/" "./${year}/day${day}"
