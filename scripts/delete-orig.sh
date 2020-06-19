#!/usr/bin/env bash

echo ""
echo ""

shopt -s globstar nullglob

for file in ./tests/**/*.orig; do
  ACTUAL="$file"
  DIR=$(dirname "$file")

  echo "REMOVING: $ACTUAL";
  echo "-----"

  rm "$ACTUAL"
done
echo "All processed"
