# Dynamic fish completion that discovers plugins at completion time
# This ensures plugins installed after nilla-cli will be included in completions
# Fish will source this file automatically when vendor completions are enabled
set nilla_cmd (command -v nilla 2>/dev/null)
if test -n "$nilla_cmd"
    set output ($nilla_cmd completions --shell fish 2>/dev/null)
    if test -n "$output"
        # Source the generated completion script (clap_complete generates fish completion functions)
        eval $output
    end
end
