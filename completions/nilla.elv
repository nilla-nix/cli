# Dynamic elvish completion that discovers plugins at completion time
# This ensures plugins installed after nilla-cli will be included in completions
# Elvish will source this file from $E:XDG_DATA_DIRS/elvish/lib/ automatically
var nilla-cmd = (which nilla)
if (not-eq $nilla-cmd $nil) {
    var output = ($nilla-cmd completions --shell elvish 2>/dev/null)
    if (not-eq $output $nil) {
        # Eval the generated completion script (clap_complete generates elvish completion code)
        eval $output
    }
}
