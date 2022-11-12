#!/bin/bash
set -e

echo "** trunk action **"
pwd
echo "** to execute: $1 **"
eval $1
