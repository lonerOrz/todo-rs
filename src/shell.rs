#[allow(clippy::print_literal)]
pub fn init_shell(shell: &str) {
    match shell {
        "fish" => {
            println!(
                "{}",
                r#"
function td_prompt
    set -l s (td prompt-today)
    if test "$s" != ""
        echo -n "$s "
    end
end

function show_todo_list
    td list
end

bind \co show_todo_list

if not functions -q __td_is_prompt_installed
    function __td_is_prompt_installed; end

    if functions -q fish_prompt
        functions -c fish_prompt __td_old_fish_prompt
        function fish_prompt
            td_prompt
            __td_old_fish_prompt
        end
    else
        function fish_prompt
            td_prompt
            echo -n (prompt_pwd) '> '
        end
    end
end
"#
            );
        }

        "zsh" => {
            println!(
                "{}",
                r#"
td_prompt() {
  local s=$(td prompt-today)
  if [[ -n "$s" ]]; then
    echo -n "$s "
  fi
}
show_todo_list() {
  td list
}
bindkey '^O' show_todo_list

if [[ -z "$__TD_PROMPT_INSTALLED" ]]; then
  export __TD_PROMPT_INSTALLED=1
  setopt PROMPT_SUBST
  PROMPT='$(td_prompt)'$PROMPT
fi
"#
            );
        }

        "bash" => {
            println!(
                "{}",
                r#"
td_prompt_bash() {
  local s
  s=$(td prompt-today)
  if [[ -n "$s" ]]; then
    export export TD_PROMPT_STRING="\\[\\e[32m\\]$s\\[\\e[0m\\] "
  else
    export TD_PROMPT_STRING=""
  fi
}

show_todo_list() {
  local old_line=${READLINE_LINE}
  local old_point=${READLINE_POINT}
  local list_output

  # Capture the output of the command
  list_output=$(td list)

  # Clear the current line, print the command output, and then a newline
  printf '\r\033[K%s\n' "$list_output"

  # Redraw the prompt and restore the user's command line
  eval "$PROMPT_COMMAND"
  printf "%s" "$PS1"
  READLINE_LINE=$old_line
  READLINE_POINT=$old_point
}

if [[ -z "$__TD_BASH_PROMPT_INSTALLED" ]]; then

  export __TD_BASH_PROMPT_INSTALLED=1
  if [[ ! "$PROMPT_COMMAND" =~ td_prompt_bash ]]; then
    PROMPT_COMMAND="td_prompt_bash;${PROMPT_COMMAND}"
  fi

  if [[ ! "$PS1" =~ TD_PROMPT_STRING ]]; then
    PS1="$TD_PROMPT_STRING"$PS1
  fi
fi


bind -x '"\C-o": "show_todo_list"'
"#
            );
        }

        _ => eprintln!("Unsupported shell: {}", shell),
    }
}
