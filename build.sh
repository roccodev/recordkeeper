#!/bin/sh

# Syntax:
# ./build.sh <bdat dir>
#
# On success, the webapp output will be in webapp/dist.
set -e

cwd=$(pwd)
commit=$(git describe --always --dirty)
commitmsg=$(git log -1 --pretty=%B)

# Run app builder and pass arguments
cargo run --release -p app-builder -- $1 webapp/res

# Build webapp after reading game data
cd webapp
trunk build --release --public-url recordkeeper

cd dist

# GH pages: catch 404s for links (technically not necessary since
# Recordkeeper uses an in-memory router
cp index.html 404.html

# Prepare for deploy (this script doesn't actually deploy, though)
git init
git checkout --force -b gh-pages
git add --all
git commit -m "[$commit] $commitmsg"

cd $cwd


