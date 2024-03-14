#!/usr/bin/env python3

from shutil import copyfile
from subprocess import call

copyfile('target/wasm32-unknown-unknown/release/space_invaders.wasm', 'space_invaders.wasm')
#call(['wasm-gc', 'html/space_invaders.wasm', 'html/program.wasm'])