#!/bin/bash

function calc() {
  x_c=0
  y_c=0
  x_r=0
  y_r=0
  santa=0
  declare -A positions
  positions["p-${x_c}-${y_c}"]=1
  positions["p-${x_r}-${y_r}"]=1

  grep -o . $1 | while read ch; do 
    if [ "$ch" == "^" ]; then
      if [ "$santa" == "0" ]; then
        y_c=$((y_c + 1))
      else
        y_r=$((y_r + 1))
      fi
    elif [ "$ch" == "v" ]; then
      if [ "$santa" == "0" ]; then
        y_c=$((y_c - 1))
      else
        y_r=$((y_r - 1))
      fi
    elif [ "$ch" == ">" ]; then
      if [ "$santa" == "0" ]; then
        x_c=$((x_c + 1))
      else
        x_r=$((x_r + 1))
      fi
    elif [ "$ch" == "<" ]; then
      if [ "$santa" == "0" ]; then
        x_c=$((x_c - 1))
      else
        x_r=$((x_r - 1))
      fi
    else
      echo "Bad char $ch"
      exit 1;
    fi
    positions["p-${x_c}-${y_c}"]=1
    positions["p-${x_r}-${y_r}"]=1
    if [ "$santa" == "0" ]; then
      santa=1
    else
      santa=0
    fi
    echo "Total positions: ${#positions[@]}"
  done
  # for whatever reason, positions is reset here, hence echoing earlier
}

calc $1


