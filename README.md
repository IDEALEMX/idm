# IDEALEMX's Dotfile Manager
A *simple* nushell utility for managing your system's configuration *quickly*
and *predictably* for POSIX UNIX systems.

# Features
## Symlink management
### Introduction
IDM allows for the automatic creation and management of symbolic links from a local git to
the rest of the system.

### Configuration syntax
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

# Roadmap
[ ] Proper --help/ -h message
[ ] Sudo flag for links that may require sudo to be created
[ ] Allow gpush to push into origin in branches other than main properly
[ ] Include support for all run-pre and run-post variants
[ ] Restore option implemented using fzf match selection
