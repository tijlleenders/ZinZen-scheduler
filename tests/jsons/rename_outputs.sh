#! /bin/bash

for d in */ ; do 
    # echo $(ls $d)
    mv "$d/output2.json" "$d/output.json"
done;