#!/bin/bash

jq '[..|numbers]|add' $1
