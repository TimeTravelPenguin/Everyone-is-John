[no-cd]
build_assets:
  #!/usr/bin/env bash
  set -e

  cd assets 
  rm -f eij_*.svg favicon.ico

  echo "Building assets..."
  typst c eij.typ  --input theme=mocha eij_mocha.svg 
  typst c eij.typ  --input theme=latte eij_latte.svg 
  typst c eij.typ  --input theme=plain eij_plain.svg 
  
  echo "Generating icons..."
  magick -density 300 \
    -define icon:auto-resize=256,128,96,64,48,32,16 \
    -background none \
    eij_plain.svg \
    favicon.ico

  rm eij_plain.svg
