# from within wash

```
ctl start actor /workspaces/macca/target/wasm32-unknown-unknown/debug/macca_s.wasm
```

start provider 

```
ctl start provider wasmcloud.azurecr.io/httpserver:0.12.1
```

get module key

To get your (ACTOR_MODULE_KEY), a 56-character string beginning with the letter M, you can check them from inside the REPL by inspecting the local signed wasm:

```
 claims inspect --insecure /workspaces/macca/target/wasm32-unknown-unknown/debug/macca_s.wasm
```

link the actor to capability provider

```
ctl link (ACTOR_MODULE_KEY) VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M wasmcloud:httpserver PORT=8080
```

 
 ctl link MDYZK4K3WOHXAS6PUAFSLNEDAUW7KHDM6WRB3BBTSWADGJYJEEJ2SQ4R   VAG3QITQQ2ODAOWB5TTQSDJ53XK3SHBEIFNK4AYJ5RKAX2UNSCAPHA5M wasmcloud:httpserver PORT=8080
 MCWY6ZOWW3ERTGE2BCBUB57P4G2ZCEVP5ZTJDQRTA5TV2EQDQPJMQBR4 