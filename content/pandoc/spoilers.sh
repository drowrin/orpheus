#!/usr/bin/env bash

awk '
    BEGIN { count = 0 }
    {
    line = $0
    output = ""
    while (match(line, /\|\|/)) {
        count++
        if (count % 2 == 0) {
            output = output substr(line, 1, RSTART - 1) "</span>"
        } else {
            output = output substr(line, 1, RSTART - 1) \
                "<span \
                    class=\"spoiler\" \
                    hx-on:click=\"this.classList.toggle('\''revealed'\'')\" \
                >"
        }
        line = substr(line, RSTART + 2)
    }

    output = output line
    print output
    }
    ' \
    "$1" \
    > "$2"
