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

function fish_prompt
    td_prompt
    echo -n (prompt_pwd) '> '
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

PROMPT='$(td_prompt)%~> '
"#
            );
        }

        _ => eprintln!("Unsupported shell: {}", shell),
    }
}
