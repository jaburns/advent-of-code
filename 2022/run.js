#!/usr/bin/env node
const cproc = require('child_process')
const fs = require('fs')
process.chdir(__dirname);

const day = process.argv[2]

const count = fs.readFileSync(`crates/day_${day}/src/main.rs`,'utf8')
    .split('\n')
    .map(x => x
        .trim()
        .replace(/\/\/.*/, '')
        .replace(/#\[allow\(.*\)\]/g,'')
        .replace('#[rustfmt::skip]','')
        .replace(/\s/g,''))
    .join('')
    .trim()
    .length

console.log(`\nCharacters: ${count}\n`)
cproc.spawnSync('cargo', ['run'], { cwd: `crates/day_${day}`, stdio: 'inherit' })
console.log('')
