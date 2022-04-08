#!/usr/bin/env bash
set -e
set -o pipefail

projectPath=$(cd "$(dirname "${0}")" && cd ../ && pwd)

for c in "$projectPath"/contracts/*; do
  if [[ "$c" != *"cw20-base" ]]; then
      (cd $c && cargo schema)
done
