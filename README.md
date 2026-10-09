# IDEALEMX's Dotfile Manager
A *simple* nushell utility for managing your system's configuration *quickly*
and *predictably*.

## Features
### Commands
- idm sync
Will sync all needed system links from dotfiles folder.

## Configuration
Stored on: ~/.config/idm/idm.json

Sample configuration:
``` json
{
    "dotfile-path" : "~/dotfiles",
    "paths" : {
        "~/.config/" : {
            "nvim" : {},
            "waybar" : {}
        },

        "~/" : {
            ".bashrc" : { "alias" : "bash" }
        }
    }
}
```
