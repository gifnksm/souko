# souko key bindings for zsh
# Customizable env vars (set before source):
#     SOUKO_COMMAND          : souko command to execute (default: souko)
#     SOUKO_SELECTOR         : auto|sk|fzf (default: auto)
#     SOUKO_LIST_TEMPLATE    : template for `souko list --template` (default: '{repo_canonical_path}\t{root_name} {repo_relative_path}')
#     SOUKO_KEY_CD_REPO      : bind key for cd widget (default: '^G' = Ctrl-g)
#     SOUKO_SK_OPTS          : extra args for sk
#     SOUKO_SK_TMUX_OPTS     : sk --tmux option value (default: center,80%)
#     SOUKO_FZF_OPTS         : extra args for fzf
#     SOUKO_FZF_TMUX_OPTS    : fzf --tmux option value (default: center,80%)

: ${SOUKO_COMMAND:=souko}
: ${SOUKO_SELECTOR:=auto}
: ${SOUKO_LIST_TEMPLATE:=$'{repo_canonical_path}\t{root_name} {repo_relative_path}'}
: ${SOUKO_KEY_CD_REPO='^G'}
: ${SOUKO_SK_OPTS:=}
: ${SOUKO_SK_TMUX_OPTS:=center,80%}
: ${SOUKO_FZF_OPTS:=}
: ${SOUKO_FZF_TMUX_OPTS:=center,80%}

.souko_msg() {
    local msg="$*"

    if [[ -n "${WIDGET}" ]]; then
        zle -M -- "${msg}"
    else
        print -u2 -- "${msg}"
    fi
}

.souko_resolve_selector() {
    builtin emulate -L zsh ${=${options[xtrace]:#off}:+-o xtrace}
    builtin setopt extended_glob warn_create_global typeset_silent no_short_loops rc_quotes no_auto_pushd

    case "${SOUKO_SELECTOR}" in
        sk|fzf)
            command -v "${SOUKO_SELECTOR}" >/dev/null 2>&1 || return 1
            print -r -- "${SOUKO_SELECTOR}"
            ;;
        auto)
            if command -v sk >/dev/null 2>&1; then
                print -r -- "sk"
            elif command -v fzf >/dev/null 2>&1; then
                print -r -- "fzf"
            else
                return 1
            fi
            ;;
        *)
            return 1
            ;;
    esac
}

.souko_select_line() {
    builtin emulate -L zsh ${=${options[xtrace]:#off}:+-o xtrace}
    builtin setopt pipefail no_aliases extended_glob warn_create_global typeset_silent no_short_loops rc_quotes no_auto_pushd

    local selector="$1" selected
    local -a sk_opts fzf_opts

    if [[ -n "${SOUKO_SK_OPTS}" ]]; then
        sk_opts=("${(@z)SOUKO_SK_OPTS}")
    fi
    if [[ -n "${SOUKO_FZF_OPTS}" ]]; then
        fzf_opts=("${(@z)SOUKO_FZF_OPTS}")
    fi

    case "${selector}" in
        sk)
            if [[ -n "${TMUX:-}" ]]; then
                selected="$("${selector}" --delimiter $'\t' --with-nth 2.. --nth 1.. --tmux="${SOUKO_SK_TMUX_OPTS}" "${sk_opts[@]}")"
            else
                selected="$("${selector}" --delimiter $'\t' --with-nth 2.. --nth 1.. "${sk_opts[@]}")"
            fi
            ;;
        fzf)
            if [[ -n "${TMUX:-}" ]]; then
                selected="$("${selector}" --delimiter $'\t' --with-nth 2.. --nth 1.. --tmux="${SOUKO_FZF_TMUX_OPTS}" "${fzf_opts[@]}")"
            else
                selected="$("${selector}" --delimiter $'\t' --with-nth 2.. --nth 1.. "${fzf_opts[@]}")"
            fi
            ;;
        *)
            return 1
            ;;
    esac

    [[ -n "${selected}" ]] || return 1
    print -r -- "${selected}"
}

.souko_pick_repo_line() {
    builtin emulate -L zsh ${=${options[xtrace]:#off}:+-o xtrace}
    builtin setopt pipefail no_aliases extended_glob warn_create_global typeset_silent no_short_loops rc_quotes no_auto_pushd

    command -v "${SOUKO_COMMAND}" >/dev/null 2>&1 || {
        .souko_msg "${SOUKO_COMMAND}: command not found"
        return 1
    }

    local selector selected
    selector="$(.souko_resolve_selector)" || {
        .souko_msg "${SOUKO_COMMAND}: neither sk nor fzf is available (or invalid SOUKO_SELECTOR)"
        return 1
    }

    selected="$("${SOUKO_COMMAND}" list --template "${SOUKO_LIST_TEMPLATE}" | .souko_select_line "${selector}")" || return 1
    print -r -- "${selected}"
}

# default contract: template outputs "path<TAB>label"
.souko_extract_path_from_line() {
    builtin emulate -L zsh ${=${options[xtrace]:#off}:+-o xtrace}
    builtin setopt extended_glob warn_create_global typeset_silent no_short_loops rc_quotes no_auto_pushd

    local line="$1"
    if [[ "${line}" == *$'\t'* ]]; then
        print -r -- "${line%%$'\t'*}"
    else
        print -r -- "${line}"
    fi
}

souko-cd-widget() {
    builtin emulate -L zsh ${=${options[xtrace]:#off}:+-o xtrace}
    builtin setopt extended_glob warn_create_global typeset_silent no_short_loops rc_quotes no_auto_pushd no_aliases

    local MATCH REPLY
    integer MBEGIN MEND
    local -a match mbegin mend reply
    local line repo_path
    line="$(.souko_pick_repo_line)" || {
        return 0
    }

    repo_path="$(.souko_extract_path_from_line "${line}")"
    [[ -n "${repo_path}" ]] || {
        return 0
    }

    builtin cd -- "${repo_path}" || {
        .souko_msg "${SOUKO_COMMAND}: failed to cd: ${repo_path}"
        return 0
    }

    zle reset-prompt
    return 0
}

if [[ -o interactive ]]; then
    zle -N souko-cd-widget

    if [[ -n "${SOUKO_KEY_CD_REPO}" ]]; then
        bindkey "${SOUKO_KEY_CD_REPO}" souko-cd-widget
    fi
fi
