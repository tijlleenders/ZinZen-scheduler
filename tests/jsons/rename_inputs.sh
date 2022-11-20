#! /bin/bash

for d in */ ; do 
    # echo $(ls $d)
    mv "$d/input2.json" "$d/input.json"
done;