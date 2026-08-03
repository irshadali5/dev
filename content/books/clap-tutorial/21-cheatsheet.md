---
title: "Cheatsheet"
description: ""
---

cheatsheet">

<div class="section-eyebrow">

Reference** >

## Cheatsheet

### \#[command(...)] attributes

| Attribute                    | Effect                       |
|------------------------------|------------------------------|
| `name = "myapp"`             | Binary name in usage         |
| `about = "..."`              | Short description (-h)       |
| `long_about = "..."`         | Long description (--help)    |
| `version = "1.0.0"`          | Version for -V / --version   |
| `author = "..."`             | Author in --help             |
| `after_help = "..."`         | Text after options in help   |
| `subcommand_required = true` | Error if no subcommand given |
| `propagate_version = true`   | Pass -V to subcommands       |

### \#[arg(...)] attributes

| Attribute                   | Effect                                   |
|-----------------------------|------------------------------------------|
| `short`                     | Enable -x short flag (auto first letter) |
| `long`                      | Enable --field-name long flag            |
| `short = 'X'`               | Custom short flag letter                 |
| `long = "alias"`            | Custom long flag name                    |
| `default_value = "str"`     | Default for String types                 |
| `default_value_t = val`     | Default for non-string types             |
| `required = true`           | Force required even if Option<T>       |
| `env = "VAR"`               | Read from env var as fallback            |
| `value_enum`                | Use ValueEnum for validation             |
| `num_args = 1..`            | Accept 1+ values                         |
| `value_delimiter = ','`     | Split on delimiter: a,b,c                |
| `conflicts_with = "other"`  | Mutually exclusive with other arg        |
| `requires = "other"`        | This arg requires other arg              |
| `global = true`             | Inherited by all subcommands             |
| `hide = true`               | Hidden from help output                  |
| `action = ArgAction::Count` | Count occurrences (-vvv → 3)             |
| `alias = "other-name"`      | Accept alternate flag name               |