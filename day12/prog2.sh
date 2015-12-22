#!/bin/bash

jq '[recurse(.[]?;select(type != "object" or ([.[]? == "red"]|any|not)))|numbers]|add' $1

