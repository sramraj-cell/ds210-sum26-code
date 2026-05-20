#!/bin/bash
# Download a random cat photo
# Usage: ./cat.sh

curl -sL "https://cataas.com/cat" -o cat.jpg
echo "Saved: cat.jpg"
