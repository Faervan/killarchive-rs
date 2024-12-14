#/usr/bin/env bash

complete -W "self-setup self-remove start stop build log send" killarchive-rs

_killarchive-rs() {
    local cur prev opts
    COMPREPLY=()
    cur="${COMP_WORDS[COMP_CWORD]}"
    prev="${COMP_WORDS[COMP_CWORD-1]}"
    opts="self-setup self-remove start stop build log send"

    COMPREPLY=( $(compgen -W "${opts}" -- ${cur}) )
    return 0
}
