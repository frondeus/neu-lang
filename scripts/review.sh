#!/bin/bash

echo ""
echo ""

for file in $(find ./tests -type f -name "*.new"); do
  ACTUAL="$file"
  DIR=$(dirname "$file")

  echo "Accepting: $ACTUAL";
  echo "-----"

  cat "$ACTUAL" | colordiff

  echo ""
  echo ""
  echo "-----"
  read -p "[A]ccept, [R]reject or [S]kip" -n 1 -r
  echo

  if [[ $REPLY =~ ^[Aa]$ ]]
  then
    cwd=$(pwd)
    cd "$DIR" || exit
    filename=$(basename -- "$ACTUAL")
    patch < "$filename"
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
