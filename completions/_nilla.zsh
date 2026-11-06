#compdef nilla

# Dynamic completion that discovers plugins at completion time
# This ensures plugins installed after nilla-cli will be included in completions
local nilla_cmd
nilla_cmd=$(command -v nilla 2>/dev/null)
if [[ -n "$nilla_cmd" ]]; then
    local output
    output=$($nilla_cmd completions --shell zsh 2>/dev/null)
    if [[ -n "$output" ]]; then
        # Eval the generated completion script (clap_complete generates a _nilla function)
        eval "$output"
    fi
fi
