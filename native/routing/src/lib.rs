use rustler::types::atom::{error};
use rustler::types::binary::{Binary};
use rustler::{Encoder, Env, Term, NifResult, Error};
// use interledger::packet::{Packet};
use interledger::ccp::{RouteControlRequest, Mode, ROUTING_TABLE_ID_LEN};//, RouteUpdateRequest};
// use bytes::{BytesMut};
use std::convert::TryFrom;
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

// #[macro_export]
macro_rules! err {
    ( $( $x:expr ),* ) => {
        {
            $(
                Err(Error::Term(Box::new($x)))
            )*   
        }
    };
}
macro_rules! error {
    ( $( $x:expr ),* ) => {
        {
            $(
                Error::Term(Box::new($x))
            )*   
        }
    };
}

#[rustler::nif(schedule = "DirtyCpu")]
fn encode<'a>(env: Env<'a>, arg: Term) -> NifResult<Term<'a>> {
    let m = arg.decode::<HashMap<String, Term>>().or(err!("arg ******"))?;
    let t = m.get("type").ok_or(error!("type missing"))?;
    match t.decode::<&str>().or(err!("type not binary"))? {
        "control_request" => {
            let f = m.get("features").ok_or(error!("control_request > features missing"))?;
            let lke = m.get("last_known_epoch").ok_or(error!("control_request > last_known_epoch missing"))?;
            let lkrti = m.get("last_known_routing_table_id").ok_or(error!("control_request > last_known_routing_table_id missing"))?;
            let md = m.get("mode").ok_or(error!("control_request > mode missing"))?;

            let lkrtis = lkrti.decode::<Vec<u8>>().or(err!("could not decode last_known_routing_table_id"))?;
            let u8mode = md.decode::<u8>().or(err!("mode not u8"))?;

            let p = RouteControlRequest {
                features: f.decode::<Vec<String>>().or(err!("could not decode features list"))?,
                last_known_epoch: lke.decode::<u32>().or(err!("last_known_epoch not u32"))?,
                last_known_routing_table_id: <[u8; ROUTING_TABLE_ID_LEN]>::try_from(lkrtis).or(err!("could not **************"))?,
                mode: Mode::try_from(u8mode).or(err!("u8mode not valid"))?
            };
            Ok(p.to_prepare().as_ref().encode(env))
        }
        "update_request" => {
            err!("not implemented")
        }
        _ => {
            err!("type not recognised")
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

