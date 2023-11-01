# Comline

## Relevant resources
https://doc.rust-lang.org/cargo/reference/registries.html



## Development Temporary Requirements
### Configuration Env Variable
This is temporary, the environment variables can be set manually but optimally should be done
on installation of the package manager

### Linux
From this directory(`comline-rs/`, be sure you are here) in console, make an alias to the executable:
  - Fish: `set COMLINE_HOME=~/.config/comline"`
  - Bash: `COMLINE_HOME=~/.config/comline` (might be wrong, didn't test)

You can check if its working by doing `echo $COMLINE_HOME`



### Package Manager Alias
This is a very temporary alias for development (which might just easily break), since we need
to use the package manager sometimes, lets do a alias:

### Linux
From this directory(`comline-rs/`, be sure you are here) in console, make an alias to the executable:
  - Fish: `alias comline-dev="\"$(pwd)/target/debug/comline-package-manager\""`
  - Bash: `alias comline-dev="\"$PWD/target/debug/comline-package-manager\""` (might be wrong, didn't test)

Now the command `comline-dev` is available


