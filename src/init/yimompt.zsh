autoload -Uz add-zsh-hook

function yimompt_precmd() {
    # Path to yimompt 
    /home/timmy/Desktop/yimompt/target/release/yimompt
}

add-zsh-hook precmd yimompt_precmd
