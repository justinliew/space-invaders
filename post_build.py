#!/usr/bin/env python

from shutil import copyfile
from subprocess import call

copyfile('target/wasm32-unknown-unknown/release/space_invaders.wasm', 'html/space_invaders.wasm')
#call(['wasm-gc', 'html/space_invaders.wasm', 'html/program.wasm'])