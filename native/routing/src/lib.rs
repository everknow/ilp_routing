use rustler::types::atom::{ok};
use rustler::types::binary::{Binary};
use rustler::{Encoder, Env, Term, NifResult};

#[rustler::nif(schedule = "DirtyCpu")]
fn decode<'a>(env: Env<'a>,_bin: Binary) -> NifResult<Term<'a>> {
    Ok(ok().encode(env))
}
rustler::init!("Elixir.IlpRouting", [decode]);

