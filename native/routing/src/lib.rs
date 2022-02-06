use rustler::types::atom::{error};
use rustler::types::binary::{Binary};
use rustler::{Encoder, Env, Term, NifResult, ListIterator, Error};
// use interledger::packet::{Packet};
use interledger::ccp::{RouteControlRequest, Mode};//, RouteUpdateRequest};
// use bytes::{BytesMut};
// use std::convert::TryFrom;
// use once_cell::sync::Lazy;
// use std::str::FromStr;
use std::collections::HashMap;
use std::boxed::Box;
// use std::time::{Duration, SystemTime};

// pub static CCP_CONTROL_DESTINATION: Lazy<Address> =
//     Lazy::new(|| Address::from_str("peer.route.control").unwrap());
// pub static CCP_UPDATE_DESTINATION: Lazy<Address> =
//     Lazy::new(|| Address::from_str("peer.route.update").unwrap());
// pub const PEER_PROTOCOL_CONDITION: [u8; 32] = [
//     102, 104, 122, 173, 248, 98, 189, 119, 108, 143, 193, 139, 142, 159, 142, 32, 8, 151, 20, 133,
//     110, 226, 51, 179, 144, 42, 89, 29, 13, 95, 41, 37,
// ];

#[rustler::nif(schedule = "DirtyCpu")]
fn decode<'a>(env: Env<'a>, _bin: Binary) -> NifResult<Term<'a>> {
    // match Packet::try_from(BytesMut::from(bin.as_slice())) {
    //     Ok(Packet::Prepare(_p)) => {
            
    //         // let destination = p.destination();
    //         // if destination == *CCP_CONTROL_DESTINATION {
    //         //     Ok(custom_atoms::control().encode(env)) 
    //         // } else if destination == *CCP_UPDATE_DESTINATION {
    //         //     Ok(custom_atoms::update().encode(env))
    //         // } else {
    //             Ok(error().encode(env)) 
    //         // }


    //         // match p.destination() {
    //         //     Packet::CCP_UPDATE_DESTINATION => {
    //         //         Ok(custom_atoms::update().encode(env)) 
    //         //     } 
    //         //     Packet::CCP_CONTROL_DESTINATION => {
    //         //         Ok(custom_atoms::control().encode(env)) 

    //         //     }
                
    //         //     _ => {
    //         //         Ok(error().encode(env)) 
    //         //     }
    //         // }
    //     }
    //     _ => {    
            Ok(error().encode(env))
        // }
    // }
    
    
}

#[rustler::nif(schedule = "DirtyCpu")]
fn encode<'a>(env: Env<'a>, arg: Term) -> NifResult<Term<'a>> {
    let m = arg.decode::<HashMap<String, Term>>()?;
    match m.get("type") {
        Some(t) => {
            if "control_request" == t.decode::<String>()? {
                match m.get("features") {
                    Some(f) => {
                        let fs :ListIterator = f.decode()?; 
                        match m.get("last_known_epoch") {
                            Some(lke) => {
                                let p = RouteControlRequest {
                                    features: fs.map(|x| x.decode::<String>()).collect::<NifResult<Vec<String>>>()?,
                                    last_known_epoch: lke.decode()?,
                                    last_known_routing_table_id: [0,1,2,3,4,5,6,7,8,9,1,2,3,4,5,6],
                                    mode: Mode::Sync
                                };
                                Ok(p.to_prepare().as_ref().encode(env))
                            }
                            None => {
                                Err(Error::Term(Box::new("control_request > last_known_epoch missing")))
                            }
                        }                               

                    }
                    None => {
                        Err(Error::Term(Box::new("control_request > features missing")))
                    }
                }
                
               

            } else {
                Err(Error::Term(Box::new("type not recognised")))
            }
            // let fs :ListIterator = ;
        }
        None => {
            Err(Error::Term(Box::new("type missing")))
        }

    }


    

    // let mut m = HashMap::new();
    // for (k,v) in kvs {
    //     if k.is_binary() {
    //         m.insert(k.into_binary().unwrap(), v);
    //     } else {
    //         return Ok((error(), "a key is not binary").encode(env))
    //     }
    // }

    //         match m.get(b"type".as_ref()) {
    //             Some(t) if t.is_binary() => {
    //                 if &t.into_binary().unwrap().as_slice() == &b"prepare".as_slice() {
    //                     match m.get(b"features".as_ref()) {
    //                         Some(f) => {
    //                             let fs :ListIterator = f.decode()?;


    //                             Ok(ok().encode(env))
    //                         }
    //                         // Some(d) if d.is_binary() => {
                                
    //                         //     let data = RouteControlRequest {
    //                         //         features: Vec::new(),
    //                         //         last_known_epoch: 10,
    //                         //         last_known_routing_table_id: [0,1,2,3,4,5,6,7,8,9,1,2,3,4,5,6],
    //                         //         mode: Mode::Sync


    //                         //     };

    //                         //     let p = data.to_prepare();
                                
    //                         //     // let p = PrepareBuilder {
    //                         //     //     destination: CCP_CONTROL_DESTINATION.clone(),
    //                         //     //     amount: 0,
    //                         //     //     expires_at: SystemTime::now() + Duration::from_secs(30),
    //                         //     //     data: &[],
    //                         //     //     execution_condition: &PEER_PROTOCOL_CONDITION,
    //                         //     // }
    //                         //     // .build();

    //                         //     Ok(p.as_ref().encode(env))
    //                         // }
    //                         // Some(_) => {
    //                         //     Ok((error(), "data val is not binary").encode(env)) 
    //                         // }
    //                         None => {
    //                             Ok((error(), "data is not present").encode(env))  
    //                         }
    //                     }
    //                 } else {
    //                     Ok((error(), "type value not recognised").encode(env))
    //                 }                    
    //             }
    //             Some(_) => {
    //                 Ok((error(), "type val is not binary").encode(env))
    //             }
    //             None => {
    //                 Ok((error(), "type is not present").encode(env))  
    //             }

    //         } 
            
    //     }
    //     None => {
    //         Ok((error(), "argument is not a map").encode(env))
    //     }
    // }

}



mod custom_atoms {
    rustler::atoms! {
        update,
        control
    }
}

rustler::init!("Elixir.IlpRouting", [decode, encode]);

