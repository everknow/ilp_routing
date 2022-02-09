use rustler::types::atom::{error};
use rustler::types::binary::{Binary};
use rustler::{Encoder, Env, Term, NifResult, Error};
use interledger::packet::{Address};
use interledger::ccp::{RouteControlRequest, Mode, ROUTING_TABLE_ID_LEN, RouteUpdateRequest};
// use bytes::{BytesMut};
use std::convert::TryFrom;
// use once_cell::sync::Lazy;
use std::str::FromStr;
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
    //         //     }Address
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
    let m = arg.decode::<HashMap<String, Term>>().or(err!("could not decode arg to map<String,Term>"))?;
    let t = m.get("type").ok_or(error!("type missing"))?;
    
    match t.decode::<&str>().or(err!("type not binary"))? {
        
        "control_request" => {
            // get fields
            let f = m.get("features").ok_or(error!("control_request > features missing"))?;
            let lke = m.get("last_known_epoch").ok_or(error!("control_request > last_known_epoch missing"))?;
            let lkrti = m.get("last_known_routing_table_id").ok_or(error!("control_request > last_known_routing_table_id missing"))?;
            let md = m.get("mode").ok_or(error!("control_request > mode missing"))?;

            // transform
            let lkrtis = lkrti.decode::<Vec<u8>>().or(err!("could not decode last_known_routing_table_id"))?;
            let u8mode = md.decode::<u8>().or(err!("mode not u8"))?;
            let features = f.decode::<Vec<String>>().or(err!("could not decode features list"))?;
            let last_known_epoch = lke.decode::<u32>().or(err!("last_known_epoch not u32"))?;
            let last_known_routing_table_id =  <[u8; ROUTING_TABLE_ID_LEN]>::try_from(lkrtis).or(err!(
                "could not convert last_known_routing_table_id to list of bytes of size ROUTING_TABLE_ID_LEN"))?;
            let mode = Mode::try_from(u8mode).or(err!("u8mode not valid"))?;

            let p = RouteControlRequest {
                features,
                last_known_epoch,
                last_known_routing_table_id,
                mode,
            };
            Ok(p.to_prepare().as_ref().encode(env))
        }
        
        "update_request" => {
            // get fields
            let rti = m.get("routing_table_id").ok_or(error!("update_request > routing_table_id missing"))?; // routing_table_id
            let cei = m.get("current_epoch_index").ok_or(error!("update_request > the current epoch index is missing"))?; // current_epoch_index
            let fei = m.get("lfrom_epoch_index").ok_or(error!("update_request > the index from the epoch is missing"))?; // from_epoch_index
            let tei = m.get("to_epoch_index").ok_or(error!("update_request > the index to the epoch is missing"))?; // to_epoch_index
            let hdt = m.get("hold_down_time").ok_or(error!("update_request > the hold_down_time is missing"))?; // hold_down_time
            let s = m.get("speaker").ok_or(error!("update_request > speaker is missing"))?; // speaker
            let nr = m.get("new_routes").ok_or(error!("update_request > new routes is missing"))?; // new_routes
            let wr = m.get("mode").ok_or(error!("update_request > the withdrawn routes is missing"))?; // withdrawn_routes

            // transform

            let rtis = rti.decode::<Vec< u8 >>().or(err!("could not decode the routing_table_id"))?;// routing_table_id
            let speakerstr = s.decode::<&str>().or(err!("could not decode speaker"))?;
            
            let routing_table_id = <[u8; ROUTING_TABLE_ID_LEN]>::try_from(rtis).or(err!("could not decode the routing table id "))?;
            let current_epoch_index = cei.decode::<u32>().or(err!("could not decode the routing_table_id"))?;
            let from_epoch_index = fei.decode::<u32>().or(err!("could not decode the index from the epoch"))?;
            let to_epoch_index = tei.decode::<u32>().or(err!("could not decode the index to the epoch"))?;
            let hold_down_time = hdt.decode::<u32>().or(err!("could not decode the hold_down_time"))?;
            let speaker = Address::from_str(speakerstr).or(err!("could not convert speaker into address"))?;
            let new_routes = nr.decode::<String>().or(err!("could not decode new_routes"))?;
            let withdrawn_routes = wr.decode::<Vec<String>>().or(err!("could not decode withdrawn_routes"))?;

            let p = RouteUpdateRequest {
                routing_table_id,
                current_epoch_index,
                from_epoch_index,
                to_epoch_index,
                hold_down_time,
                speaker,
                new_routes,
                withdrawn_routes,
            };
            Ok(p.to_prepare().as_ref().encode(env))
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

