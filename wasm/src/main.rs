use anyhow::Result;
use wasmtime::*;
use wasmtime_wasi::sync::WasiCtxBuilder;
use wasmtime_wasi::WasiCtx;

wai_bindgen_wasmtime::import!("api.witx");

struct State {
    wasi: WasiCtx,
    api: api::ApiData,
}

fn main() -> Result<()> {
    use api::*;

    // Define the WASI functions globally on the `Config`.
    let engine = Engine::default();
    let mut linker = Linker::new(&engine);
    wasmtime_wasi::add_to_linker(&mut linker, |s: &mut State| &mut s.wasi)?;

    // Create a WASI context and put it in a Store; all instances in the store
    // share this context. `WasiCtxBuilder` provides a number of ways to
    // configure what the target program will have access to.
    let wasi = WasiCtxBuilder::new()
        .inherit_stdio()
        .inherit_args()?
        .build();
    let state = State {
        wasi,
        api: Default::default(),
    };
    let mut store = Store::new(&engine, state);

    // Instantiate our module with the imports we've created, and run it.
    let module = Module::from_file(&engine, "../target/wasm32-wasi/debug/api.wasm")?;

    let (api, _instance) =
        Api::instantiate(&mut store, &module, &mut linker, |s: &mut State| &mut s.api)?;
    let handle = api
        .greeter_with_config(
            &mut store,
            &[
                Config::Lang(Lang::En),
                Config::Datetime(Datetime {
                    date: (9, 11, 2021),
                    time: (0, 12, 33),
                }),
                Config::Mode(Mode::FRIENDLY.union(Mode::AGITATED)),
            ],
        )
        .unwrap()
        .unwrap();
    println!(
        "sync greet: {}",
        api.greeter_greet(&mut store, &handle).unwrap().unwrap()
    );

    Ok(())
}
