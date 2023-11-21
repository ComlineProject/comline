package = "comline-runtime"
version = "0.1.0"

source = {
    url = "git+https://git.ablecorp.us/DOOME1M8Cover/comline/src/branch/main/runtime/luau",
    tag = "0.1.0",
}

description = {
    summary = "A agnostic and abstract RPC/IPC network schema library for",
    detailed = [[
        An agnostic and abstract RPC/IPC network Schema(IDL) library
        Allows to specify your own transport methods, message formats and call system.
        As the likes of CapNProto, gRPC, FuchsiaIDL, but a bit more refined

        Entirely written in Rust, with some per language runtime adaptations.
    ]],
    homepage = "https://git.ablecorp.us/DOOME1M8Cover/comline",
    license = "None"
}

dependencies = {
    "lua >= 5.1",
    "luarocks-build-rust-mlua",
}

build = {
    type = "rust-mlua",
    modules = {
        "comline-runtime"
    },
}

