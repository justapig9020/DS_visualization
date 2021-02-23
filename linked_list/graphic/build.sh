#!/bin/bash

files=`ls *.gv`

for file in $files; do
    file=${file%.*}
    dot -Tjpg $file.gv -o $file.jpg
done
