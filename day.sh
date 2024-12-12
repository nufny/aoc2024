#!/bin/bash
cp templates/day_template.rs src/day$1.rs
mkdir -p inputs/$1/{1,2}
touch inputs/$1/{{input,test}.txt,{1,2}/test_result.txt}

echo "" >> src/main.rs
echo "pub mod day$1;" >> src/main.rs

bat -p templates/test_template.rs | sed s/x/$1/g >> src/tests.rs