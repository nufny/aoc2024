#!/bin/bash
cp src/{day_template,day$1}.rs
mkdir -p inputs/$1/{1,2}
touch inputs/$1/{{input,test}.txt,{1,2}/test_result.txt}
