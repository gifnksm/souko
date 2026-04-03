# souko zsh plugin entrypoint
# Supports plugin managers and manual source.

_souko_plugin_file="${ZERO:-${${0:#$ZSH_ARGZERO}:-${(%):-%N}}}"
_souko_plugin_file="${${(M)_souko_plugin_file:#/*}:-$PWD/$_souko_plugin_file}"

_souko_plugin_root="${_souko_plugin_file:h}"

# Load key bindings only in interactive shells.
if [[ -o interactive ]]; then
    source "${_souko_plugin_root}/shell/key-bindings.zsh"
fi

unset _souko_plugin_file _souko_plugin_root
