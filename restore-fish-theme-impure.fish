#! /usr/bin/env nix-shell
#! nix-shell -i fish
#! nix-shell -p fish

# necessarily impure, due to shell.nix and flake.nix both resetting the theme for some reason
if test -n "$argv" && test "$argv" = "help"
    set -l script_dir_rel (status --current-filename)
    set -l script_dir_abs (realpath (status --current-filename))
    printf "\nUsage (relative path):\n$script_dir_rel [help|default|(nothing or somethign else)]\n"
    printf "\nUsage (absolute path):\n$script_dir_abs [help|default|(nothing or somethign else)]\n"
    printf "\nONLY USE IF YOU USE THE FISH SHELL!!\n"
    return 0
else if test -n "$argv" && test "$argv" = "default"
    set -U fish_color_normal normal
    set -U fish_color_command normal
    set -U fish_color_keyword normal
    set -U fish_color_quote yellow
    set -U fish_color_redirection cyan --bold
    set -U fish_color_end green
    set -U fish_color_error brred
    set -U fish_color_param cyan
    set -U fish_color_comment red
    set -U fish_color_selection white --bold --background=brblack
    set -U fish_color_search_match white --bold --background=brblack
    set -U fish_color_history_current --bold
    set -U fish_color_operator brcyan
    set -U fish_color_escape brcyan
    set -U fish_color_cwd green
    set -U fish_color_cwd_root red
    set -U fish_color_option cyan
    set -U fish_color_valid_path --underline=single
    set -U fish_color_autosuggestion brblack
    set -U fish_color_user brgreen
    set -U fish_color_host normal
    set -U fish_color_host_remote yellow
    set -U fish_color_history_current --bold
    set -U fish_color_status red
    set -U fish_color_cancel --reverse
    set -U fish_pager_color_prefix normal --bold --underline=single
    set -U fish_pager_color_progress brwhite --bold --background=cyan
    set -U fish_pager_color_completion normal
    set -U fish_pager_color_description yellow --italics
    set -U fish_pager_color_selected_background --reverse
    set -U fish_pager_color_secondary_background
    set -U fish_pager_color_selected_completion
    set -U fish_pager_color_selected_description
    set -U fish_pager_color_secondary_prefix
    set -U fish_pager_color_selected_prefix
    set -U fish_pager_color_background
    set -U fish_pager_color_secondary_completion
    set -U fish_pager_color_secondary_description
else
    # opens the web interface
    fish_config
end
