use rustler::types::atom::{error, ok};
use rustler::types::binary::{Binary};
use rustler::{Encoder, Env, Term, NifResult, MapIterator};
use interledger_packet::{Packet, Address, PrepareBuilder};
use bytes::{BytesMut};
use std::convert::TryFrom;
use once_cell::sync::Lazy;
use std::str::FromStr;
use std::collections::HashMap;
use std::time::{Duration, SystemTime};

pub static CCP_CONTROL_DESTINATION: Lazy<Address> =
    Lazy::new(|| Address::from_str("peer.route.control").unwrap());
pub static CCP_UPDATE_DESTINATION: Lazy<Address> =
    Lazy::new(|| Address::from_str("peer.route.update").unwrap());

#[rustler::nif(schedule = "DirtyCpu")]
fn decode<'a>(env: Env<'a>, bin: Binary) -> NifResult<Term<'a>> {
    match Packet::try_from(BytesMut::from(bin.as_slice())) {
        Ok(Packet::Prepare(p)) => {
            
            let destination = p.destination();
            if destination == *CCP_CONTROL_DESTINATION {
                Ok(custom_atoms::control().encode(env)) 
            } else if destination == *CCP_UPDATE_DESTINATION {
                Ok(custom_atoms::update().encode(env))
            } else {
                Ok(error().encode(env)) 
            }


            // match p.destination() {
            //     Packet::CCP_UPDATE_DESTINATION => {
            //         Ok(custom_atoms::update().encode(env)) 
            //     } 
            //     Packet::CCP_CONTROL_DESTINATION => {
            //         Ok(custom_atoms::control().encode(env)) 

            //     }
                
            //     _ => {
            //         Ok(error().encode(env)) 
            //     }
            // }
        }
        _ => {    
            Ok(error().encode(env))
        }
    }
    
    
}

#[rustler::nif(schedule = "DirtyCpu")]
fn encode<'a>(env: Env<'a>, arg: Term) -> NifResult<Term<'a>> {
    
    match MapIterator::new(arg) {
        Some(kvs) => {
            let mut m = HashMap::new();
            for (k,v) in kvs {
                if k.is_binary() {
                    m.insert(k.into_binary().unwrap(), v);
                } else {
                    return Ok((error(), "a key is not binary").encode(env))
                }
            }

            match m.get(b"type".as_ref()) {
                Some(t) if t.is_binary() => {
                    if &t.into_binary().unwrap().as_slice() == &b"prepare".as_slice() {
                        match m.get(b"data".as_ref()) {
                            Some(d) if d.is_binary() => {
                                // let p = PrepareBuilder {
                                //     destination: CCP_CONTROL_DESTINATION.clone(),
                                //     amount: 0,
                                //     expires_at: SystemTime::now() + Duration::from_secs(30),
                                //     data: &[],
                                //     execution_condition: &PEER_PROTOCOL_CONDITION,
                                // }
                                // .build();

                                Ok(ok().encode(env))
                            }
                            Some(_) => {
                                Ok((error(), "data val is not binary").encode(env)) 
                            }
                            None => {
                                Ok((error(), "data is not present").encode(env))  
                            }
                        }
                    } else {
                        Ok((error(), "type value not recognised").encode(env))
                    }                    
                }
                Some(_) => {
                    Ok((error(), "type val is not binary").encode(env))
                }
                None => {
                    Ok((error(), "type is not present").encode(env))  
                }

            } 
            
        }
        None => {
            Ok((error(), "argument is not a map").encode(env))
        }
    }

}



mod custom_atoms {
    rustler::atoms! {
        update,
        control
    }
}

rustler::init!("Elixir.IlpRouting", [decode, encode]);

