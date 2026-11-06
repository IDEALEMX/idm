# IDEALEMX's Dotfile Manager
A *simple* Rust utility for managing your system's configuration *quickly*
and *predictably* for POSIX UNIX systems.

# Features
## Symlink management
### Introduction
IDM allows for the automatic creation and management of symbolic links from a local git to
the rest of the system.

### Configuration syntax
#### Basic
IDM is configured using a idm.json file with the following syntax:
```json
{
    "dotfile-path" : "path/to/dotfile/repository",

    "paths" : {
        "parent/path" : {
            "module_name" : {}
        }
    }
}
```

In the previous example, after syncing the configuration, idm would create the following symlink:
/parent/path/module_name -> /path/to/dotfile/repository/module_name

#### "from" keyword
If the name or path of the module name happens to be different from the target, then the "from" keyword can be 
used to specify the location (relative to the dotfile-path)

```json
{
    "dotfile-path" : "path/to/dotfile/repository",

    "paths" : {
        "parent/path" : {
            "module_name" : {
                "from" : "some/path/to/module"
            }
        }
    }
}
```
Which will create the following symlink
/parent/path/module_name -> /path/to/dotfile/repository/some/path/to/module

#### Hooks
Hooks allow for arbitrary code execution before and after calling different commdands,
so for example, the following configuration:

``` json
{
    "run-pre-gpush" : "~/path/to/script",
    "run-post-gpush" : "echo 'gpush done!'",
    
    "dotfile-path" : "path/to/dotfile/repository",

    "paths" : {
        "parent/path" : {
            "module_name" : {}
        }
    }
}
```

Will make it so that after calling the **idm gpush** command it will:
execute ~/path/to/script before pushing,
echo 'gpush done!' after pushing.

The content inbetween quotes will be executed as a POSIX sh command.

**The following are all the avaliable hooks:**
- run-pre-gpush
- run-post-gpush

- run-pre-gpull
- run-post-gpull

### Commands
#### imd sync
Will generate all symlinks based on configuration file.

#### idm gpush
Will automatically perform the following commands in the dotfiles repo:
``` sh
git add -A
git commit -m "26e9fb14-ef75-490a-b6ea-4139e44474db"
git push origin
```

#### idm gpush
Will pull from origin into the dotfiles repo.

## Other commands
### idm -c path/to/config_file.json
The -c flag allows for the loading a different configuration file from the default.

# Roadmap
- Proper --help/ -h message
- Sudo flag for links that may require sudo to be created
- Allow gpush to push into origin in branches other than main properly
- Restore option implemented using fzf match selection
