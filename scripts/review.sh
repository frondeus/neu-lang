#!/bin/bash

echo ""
echo ""

shopt -s globstar nullglob

for file in ./tests/**/*.new; do
  ACTUAL="$file"
  DIR=$(dirname "$file")

  echo "Accepting: $ACTUAL";
  echo "-----"

  cat "$ACTUAL" | colordiff

  echo ""
  echo ""
  echo "-----"
  read -p "[Aa]ccept, [Rr]reject or [Ss]kip: " -n 1 -r
  echo

  if [[ $REPLY =~ ^[Aa]$ ]]
  then
    cwd=$(pwd)
    cd "$DIR" || exit
    filename=$(basename -- "$ACTUAL")
    #patch --ignore-whitespace --verbose < "$filename" || exit
    patch --ignore-whitespace < "$filename" || exit
    rm ./*.orig || exit
    cd "$cwd" || exit
  elif [[ $REPLY =~ ^[Rr]$ ]]
  then
    rm -- "$ACTUAL"
  elif [[ $REPLY =~ ^[Ss]$ ]]
  then
    echo "Skipping"
  fi
done
echo "All processed"
