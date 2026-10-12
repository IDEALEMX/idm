# IDEALEMX's Dotfile Manager
A *simple* nushell utility for managing your system's configuration *quickly*
and *predictably*.

## Features
### Symlink management
idm allows for the automatic creation and managing of symlinks from a dotfiles
directory, to their corresponding places.

As an example, if you need to manage your waybar configuration, you can configure idm as follows:
``` json
{
    "dotfile-path" : "~/dotfiles",
    "paths" : {
        "~/.config/" : {
            "waybar" : {}
        }
    }
}
```
And after running 
``` sh
idm sync
```
idm will automatically generate a symlink that goes from ~/dotfiles/waybar/ to ~/.config/waybar
allowing you to place all system configurations in one single repository

### Git commands
#### idm also doubles as a small git wrapper for convenience, with the following special commands:

``` sh
idm gpush
```
idm will automatically add, commit and push the entire repository into the configured Git remote,
note the commit message is auto generated from a UUID.

``` sh
idm gpull
```
idm will automatically pull from the set configured Git remote.

``` sh
idm grestore ...modules
```
idm will git restore any configuration modules inside the dotfile path.

> [!NOTE]
> Implementation is literally git restore ./path

idm grestore can also interpret aliases, so for example, in the case of having to
keep track of a .bashrc as well, your configuration could look as follows:

``` json
{
    "dotfile-path" : "~/dotfiles",
    "paths" : {
        "~/.config/" : {
            "waybar" : {}
        },

        "~/" : {
            ".bashrc" : { "alias" : "bash" }
        }
    }
}
```
And now instead of writing
``` sh
idm grestore .bashrc
```
You can also write
``` sh
idm grestore bash
```

### Other git commands
For any other git commands, idm simply behaves a wrapper for
``` sh
git -C path/to/dotfiles
```
Allowing you to easily manage the repo from anywhere

## Flags
Currently idm supports only one flag
```sh
idm -c "config-path"
```

Which allows you to run idm while loading some .json config file other that
~/.config/idm/idm.json

## Installation
### Arch linux
```sh
sudo pacman -S nushell
git clone https://www.github.com/IDEALEMX/idm
cd idm/
sudo link ./idm /bin/idm
```

Now you can just git pull on this directory for any updates!

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

## Module Object Parameters
Every module found within a path, (say, ".bashrc" inside of "~/" or "nvim" in
"~/.config") points to a object, for example:

``` json
{
    "idm" : {}
}
```

This object can contain a few "key" "value" pairs to modify how idm will a few
"key" "value" pairs to modify how idm will treat it.

### The following include all implemented so far:

----
#### Alias

``` json
{
    ".bash" : { "alias" : "shell" }
}
```

Alias allows for the definition of a more convenient name for the module,
similar to shell aliases. \
In this case this case this would allow for the following syntax

``` sh
idm grestore shell
# instead of
idm grestore .bashrc
```

#### as
As allows to change the name of the generated symlink, as well as any
subdirectory it should be put on.

``` json
{ 
    "~/.config" : {
        "some-dir/idm" : {}
    }
}
```
In the previous example the symlink generated will be: \
~/.config/some-dir/idm\
But in this case we don't actually want to place it inside "some-dir\" and
would prefer to save the symlink as:
~/.config/idm\
In this case, we can use "as", to change the name this new symlink will take: 

``` json
{ 
    "~/.config" : {
        "some-dir/idm" : { "as" : "idm" }
    }
}
```
and so, the resulting symlink is "~/.config/idm"

## TODO
[ ] Implement proper --help/ -h message
