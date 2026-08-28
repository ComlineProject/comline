# Comline — moved

This was the original Comline monorepo. It has been split into individual
repositories under **<https://github.com/ComlineProject>** and is now archived,
kept only for history.

| Component | Repository | Was in this repo |
|---|---|---|
| Core library & compiler | [ComlineProject/core](https://github.com/ComlineProject/core) — publishes the [`comline-core`](https://crates.io/crates/comline-core) crate | `core/`, `core_stdlib/` |
| CLI (`comline`) | [ComlineProject/cli](https://github.com/ComlineProject/cli) | the former `comline/` directory |
| Code generators | [ComlineProject/generation](https://github.com/ComlineProject/generation) | `codelib-gen/` |
| Runtime | [ComlineProject/runtime](https://github.com/ComlineProject/runtime) | `runtime/`, `runtime-langs/` |
| Package manager | [ComlineProject/package-manager](https://github.com/ComlineProject/package-manager) | `package-manager/` |
| Package registry | [ComlineProject/package-registry](https://github.com/ComlineProject/package-registry) | `package-server/` |
| Language server | [ComlineProject/language-server](https://github.com/ComlineProject/language-server) | — |
| VS Code extension | [ComlineProject/comline-vscode](https://github.com/ComlineProject/comline-vscode) | — |

The [`comline`](https://crates.io/crates/comline) crate name on crates.io still
holds a 2023 placeholder (`0.1.0`) published from this repo; the real CLI lives
in [ComlineProject/cli](https://github.com/ComlineProject/cli).
