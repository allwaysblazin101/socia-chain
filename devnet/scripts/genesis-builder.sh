#!/bin/bash

GENESIS="genesis.json"
WASM_DIR="../modules"

echo '{ "runtime_modules": [' > $GENESIS

# Loop through WASM files and include metadata
for mod in $(find $WASM_DIR -name "*.wasm" | grep release | sort); do
  NAME=$(basename "$mod" .wasm)
  echo "  { \"name\": \"$NAME\", \"wasm_path\": \"$mod\" }," >> $GENESIS
done

# Remove trailing comma from last entry
sed -i '$s/,$//' $GENESIS
echo ']}' >> $GENESIS

echo "✅ Genesis built at: $GENESIS"
