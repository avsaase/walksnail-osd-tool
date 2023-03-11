#!/bin/sh -x

SIZES="
16,16x16
24,24x24
32,16x16@2x
32,32x32
48,48x48
64,32x32@2x
128,128x128
256,128x128@2x
256,256x256
512,256x256@2x
512,512x512
1024,512x512@2x
"

SVG=app-icon.svg
for PARAMS in $SIZES; do
    SIZE=$(echo $PARAMS | cut -d, -f1)
    LABEL=$(echo $PARAMS | cut -d, -f2)
    svg2png -w $SIZE -h $SIZE $SVG $LABEL.png
done
