# Dynamic bash completion that discovers plugins at completion time
# This ensures plugins installed after nilla-cli will be included in completions
_nilla() {
    local nilla_cmd
    nilla_cmd=$(command -v nilla 2>/dev/null)
    if [[ -n "$nilla_cmd" ]]; then
        local output
        output=$($nilla_cmd completions --shell bash 2>/dev/null)
        if [[ -n "$output" ]]; then
            # Source the generated completion script (clap_complete generates a _nilla function)
            eval "$output"
        fi
    fi
}

complete -F _nilla nilla
