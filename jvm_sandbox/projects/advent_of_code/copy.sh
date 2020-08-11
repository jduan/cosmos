#!/bin/bash

if [[ -z "${DAY}"  ]]; then
  echo "env var DAY isn't set!"
  exit 1
fi

cp src/main/kotlin/year2018/Day1.kt src/main/kotlin/year2018/Day"${DAY}".kt
cp src/test/kotlin/year2018/Day1Test.kt src/test/kotlin/year2018/Day"${DAY}"Test.kt
