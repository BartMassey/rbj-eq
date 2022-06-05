#!/bin/sh
rustc -vV | egrep '^host:' | awk '{print $2}'
