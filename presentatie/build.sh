#!/bin/sh
rm -rf dist
mkdir -p dist
cp -r img dist
cp -r themes/font/ dist
npx @marp-team/marp-cli@latest -o "dist/index.html" --theme "./themes/topicus.css" ./index.md
npx @marp-team/marp-cli@latest -o "dist/rust-voor-topicanen.pdf" --allow-local-files --pdf  --theme "./themes/topicus.css" ./index.md
npx @marp-team/marp-cli@latest -o "dist/rust-voor-topicanen.pptx" --allow-local-files --pptx --theme "./themes/topicus.css" ./index.md