wai_bindgen_rust::import!("api.witx");

fn main() {
    use api::*;
    let handle = Greeter::with_config(&[
        Config::Lang(Lang::En),
        Config::Datetime(Datetime {
            date: (9, 11, 2021),
            time: (0, 12, 33),
        }),
        Config::Mode(MODE_FRIENDLY | MODE_AGITATED),
    ])
    .unwrap();
    println!("sync greet: {}", handle.greet().unwrap());
    //println!("async greet: {}", async_global_executor::block_on(handle.async_greet()));
}
