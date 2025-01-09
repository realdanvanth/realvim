#!/bin/bash
cols=$(tput cols)
rows=$(tput lines)
gcc -Wall "$1.c" -o $1 -lncurses
./$1 $rows $cols "text.txt"
