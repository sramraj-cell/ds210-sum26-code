#!/bin/bash
# Download NASA's Astronomy Picture of the Day
# Usage: ./nasa.sh

API_KEY="DEMO_KEY"   # free demo key — works for classroom use

echo "Fetching today's astronomy picture..."

DATA=$(curl -s "https://api.nasa.gov/planetary/apod?api_key=${API_KEY}")

TITLE=$(echo "$DATA" | grep -o '"title":"[^"]*"' | cut -d'"' -f4)
URL=$(echo "$DATA"   | grep -o '"url":"[^"]*"'   | cut -d'"' -f4)

echo "Title : $TITLE"
echo "URL   : $URL"

curl -sL "$URL" -o apod.jpg
echo "Saved  : apod.jpg"
