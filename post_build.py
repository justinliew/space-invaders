#!/usr/bin/env python

from shutil import copyfile
from subprocess import call

copyfile('target/wasm32-unknown-unknown/release/space-invaders.wasm', 'html/space-invaders.wasm')
call(['wasm-gc', 'html/space-invaders.wasm', 'html/program.wasm'])