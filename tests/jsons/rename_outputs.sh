#! /bin/bash

for d in */ ; do 
    # echo $(ls $d)
    mv "$d/output.json" "$d/expected.json"
    mv "$d/actual_output.json" "$d/observed.json"
done;