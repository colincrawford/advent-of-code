#!/bin/bash

SESSION=$(cat session.txt)

if [ ! "$SESSION" ]
then
  echo "SESSION not set, exiting"
  exit 1
fi

if [ ! "$DAY" ]
then
  echo "DAY not set, exiting"
  exit 1
fi

YEAR=2022

curl -b "session=$SESSION" "https://adventofcode.com/$YEAR/day/$DAY/input" > ./puzzle_inputs/day$DAY.txt
