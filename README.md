# Comline

## Development WOOS
This is a very temporary alias for development (which might just easily break), since we need
to use the package manager sometimes, lets do a alias:

### Linux
From this directory(`comline-rs/`, be sure you are here) in console, make an alias to the executable:
  - Fish: `alias comline-dev="\"$(pwd)/target/debug/comline-package-manager\""`
  - Bash: `alias comline-dev="\"$PWD/target/debug/comline-package-manager\""` (might be wrong, didn't test)

Now the command `comline-dev` is available anywhere
