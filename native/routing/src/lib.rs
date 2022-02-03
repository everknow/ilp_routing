

#[rustler::nif(schedule = "DirtyCpu")]
fn decode<'a>(env: Env<'a>,_bin: Binary) -> NifResult<Term<'a>> {
    Ok(ok().encode(env))
}
rustler::init!("Elixir.Routing", [decode]);

