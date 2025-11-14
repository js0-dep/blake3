#!/usr/bin/env bash

set -e
DIR=$(realpath $0) && DIR=${DIR%/*}
cd $DIR
set -x

git add .
git commit -m $(cat package.json | jq .version -r)
npm version patch
git push
