#!/usr/bin/env bash
cpp day_$1.js | grep -v '^#' > run.js
node run.js
