//use bytes::BytesMut;
use rustler::types::atom::{error};
use rustler::types::binary::{Binary};
use rustler::{Encoder, Env, Term, NifResult, Error};
use interledger::packet::{Address};
use interledger::ccp::{RouteControlRequest, Mode, ROUTING_TABLE_ID_LEN, RouteUpdateRequest, Route, RouteProp, AUTH_LEN};
use bytes::Bytes;
use std::convert::TryFrom;
use std::str::FromStr;
use std::collections::HashMap;
use std::boxed::Box;
//use tracing::debug;

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
fn decode_control<'a>(env: Env<'a>, bin: Binary) -> NifResult<Term<'a>> {
    match RouteControlRequest::try_from_data(bin.as_slice()) {
        Ok(RouteControlRequest {
            mode,
            last_known_routing_table_id,
            last_known_epoch,
            features,
        }) => {
            let mut result: HashMap<String, Term> = HashMap::new(); 
            result.insert("type".to_string(), "control_request".encode(env));
            result.insert("features".to_string(), features.encode(env));
            result.insert("last_known_routing_table_id".to_string(), last_known_routing_table_id.encode(env) );
            result.insert("last_known_epoch".to_string(), last_known_epoch.encode(env) );
            match mode {
                Mode::Idle => result.insert("mode".to_string(), (0 as u8).encode(env) ),
                Mode::Sync => result.insert("mode".to_string(), (1 as u8).encode(env) ),
            };

            Ok(result.encode(env))
         }

        //Err(CcpPacketError::Oer(error_message)) =>
        //    Err(error_message.encode(env))

        _ => Ok(error().encode(env))
    }
}

#[rustler::nif(schedule = "DirtyCpu")]
fn decode_update<'a>(env: Env<'a>, bin: Binary) -> NifResult<Term<'a>> {
    match RouteUpdateRequest::try_from_data(bin.as_slice()) {
        Ok(RouteUpdateRequest {
            routing_table_id,
            current_epoch_index,
            from_epoch_index,
            to_epoch_index,
            hold_down_time,
            speaker,
            new_routes,
            withdrawn_routes,
        }) => { 
            let mut result: HashMap<String, Term> = HashMap::new(); 
            result.insert("type".to_string(), "update_request".encode(env));
            result.insert("routing_table_id".to_string(), routing_table_id.encode(env));
            result.insert("current_epoch_index".to_string(), current_epoch_index.encode(env));
            result.insert("from_epoch_index".to_string(), from_epoch_index.encode(env));
            result.insert("to_epoch_index".to_string(), to_epoch_index.encode(env));
            result.insert("hold_down_time".to_string(), hold_down_time.encode(env));
            result.insert("speaker".to_string(), speaker.encode(env));
            result.insert("new_routes".to_string(), encode_routes(env, new_routes).encode(env));
            result.insert("withdrawn_routes".to_string(), withdrawn_routes.encode(env));

            Ok(result.encode(env))
        }

        _ => Ok(error().encode(env))
    }
}

fn encode_routes<'a>(env: Env<'a>, routes: Vec<Route>) -> Vec<HashMap<String, Term>> {
    let mut result = Vec::new();
    for route in routes {
        let route_result: HashMap<String, Term> = encode_route(env, route);
        result.push(route_result);
    };

    return result;
}

fn encode_route<'a>(env: Env<'a>, route: Route) -> HashMap<String, Term> {
        let mut result: HashMap<String, Term> = HashMap::new();
        
        result.insert("prefix".to_string(), route.prefix.encode(env));
        result.insert("path".to_string(), route.path.encode(env));
        result.insert("auth".to_string(), route.auth.encode(env));
        //result.insert("props".to_string(), route.props);
        
        let mut props: Vec<HashMap<String, Term>> = Vec::new();

        for prop in route.props {
            props.push(encode_route_prop(env, prop));
        };

        result.insert("props".to_string(), props.encode(env));

        return result;
    }

fn encode_route_prop<'a>(env: Env<'a>, route_prop: RouteProp) -> HashMap<String, Term> { 
        let mut result: HashMap<String, Term> = HashMap::new();
        
        result.insert("is_optional".to_string(), route_prop.is_optional.encode(env));
        result.insert("is_transitive".to_string(), route_prop.is_transitive.encode(env));
        result.insert("is_partial".to_string(), route_prop.is_partial.encode(env));
        result.insert("id".to_string(), route_prop.id.encode(env));
        result.insert("is_utf8".to_string(), route_prop.is_utf8.encode(env));
        result.insert("value".to_string(), route_prop.value.encode(env));
        
        return result;
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
            Ok(p.to_data().encode(env))
        }
        
        "update_request" => {
            // get fields
            let rti = m.get("routing_table_id").ok_or(error!("update_request > routing_table_id missing"))?;
            let cei = m.get("current_epoch_index").ok_or(error!("update_request > the current epoch index is missing"))?;
            let fei = m.get("from_epoch_index").ok_or(error!("update_request > the index from the epoch is missing"))?;
            let tei = m.get("to_epoch_index").ok_or(error!("update_request > the index to the epoch is missing"))?;
            let hdt = m.get("hold_down_time").ok_or(error!("update_request > the hold_down_time is missing"))?;
            let s = m.get("speaker").ok_or(error!("update_request > speaker is missing"))?;
            let nr = m.get("new_routes").ok_or(error!("update_request > new routes is missing"))?;
            let wr = m.get("withdrawn_routes").ok_or(error!("update_request > the withdrawn routes is missing"))?;

            // transform

            let rtis = rti.decode::<Vec< u8 >>().or(err!("could not decode the routing_table_id"))?;
            let speakerstr = s.decode::<&str>().or(err!("could not decode speaker"))?;
            let nrms = nr.decode::<Vec<HashMap<String, Term>>>().or(err!("could not decode new routes map"))?; 

            let mut new_routes = Vec::with_capacity(nrms.len());

            for nrm in nrms {

                 // get fields

                let nr_pre = nrm.get("prefix").ok_or(error!("update_request > new_routes > prefix missing"))?;
                let nr_pat  = nrm.get("path").ok_or(error!("update_request > new_routes > path missing"))?;
                let nr_aut = nrm.get("auth").ok_or(error!("update_request > new_routes > auth missing"))?;
                let nr_prs  = nrm.get("props").ok_or(error!("update_request > new_routes > props missing"))?;

                // transform

                let prefix = nr_pre.decode::<String>().or(err!("could not decode new_routes > prefix"))?;
                let path = nr_pat.decode::<Vec<String>>().or(err!("could not decode new_routes > path"))?;
                let auths = nr_aut.decode::<Vec<u8>>().or(err!("could not decode new_routes > auth"))?;
                let auth =  <[u8; AUTH_LEN]>::try_from(auths).or(err!("could not convert auth to list of bytes of size ROUTING_TABLE_ID_LEN"))?; //Here
                let nr_prms = nr_prs.decode::<Vec<HashMap<String, Term>>>().or(err!("could not decode new_routes > props map"))?;
                
                let mut props = Vec::with_capacity(nr_prms.len());
                for nr_prm in nr_prms {

                    // get fields

                    let is_opt = nr_prm.get("is_optional").ok_or(error!("update_request > new_routes > route_props > is_optional missing"))?;
                    let is_par = nr_prm.get("is_partial").ok_or(error!("update_request > new_routes > route_props > is_partial missing"))?;
                    let is_utf8 = nr_prm.get("is_utf8").ok_or(error!("update_request > new_routes > route_props > is_utf8 missing"))?;
                    let is_tran = nr_prm.get("is_transitive").ok_or(error!("update_request > new_routes > route_props > is_transitive missing"))?;
                    let val = nr_prm.get("value").ok_or(error!("update_request > new_routes > route_props > value missing"))?;
                    let an_id = nr_prm.get("id").ok_or(error!("update_request > new_routes > route_props > id missing"))?;

                    // transform

                    let is_optional = is_opt.decode::<bool>().or(err!("could not decode new_routes > route_props >is_optional"))?;
                    let is_partial = is_par.decode::<bool>().or(err!("could not decode new_routes > route_props > is_partial"))?;
                    let is_utf8 = is_utf8.decode::<bool>().or(err!("could not decode new_routes > route_props > is_utf8"))?;
                    let is_transitive = is_tran.decode::<bool>().or(err!("could not decode new_routes > route_props > is_transitive"))?;
                    let valu = val.into_binary().or(err!("expected value to be binary"))?.as_slice(); 
                    let value = Bytes::copy_from_slice(valu);
                    let id = an_id.decode::<u16>().or(err!("could not decode new_routes > route_props > id"))?;

                    props.push(RouteProp {
                        is_optional,
                        is_transitive,
                        is_partial,
                        id,
                        is_utf8,
                        value,
                    })
                }

                new_routes.push(Route {
                    prefix,
                    path,
                    auth,
                    props,
                })
            }
            
            
            let current_epoch_index = cei.decode::<u32>().or(err!("could not decode the routing_table_id"))?;
            let from_epoch_index = fei.decode::<u32>().or(err!("could not decode the index from the epoch"))?;
            let to_epoch_index = tei.decode::<u32>().or(err!("could not decode the index to the epoch"))?;
            let hold_down_time = hdt.decode::<u32>().or(err!("could not decode the hold_down_time"))?;
            let speaker = Address::from_str(speakerstr).or(err!("could not convert speaker into address"))?;
            let withdrawn_routes = wr.decode::<Vec<String>>().or(err!("could not decode withdrawn_routes"))?;

            let routing_table_id = <[u8; ROUTING_TABLE_ID_LEN]>::try_from(rtis).or(err!("could not decode the routing_table_id "))?;

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
            Ok(p.to_data().encode(env))
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

rustler::init!("Elixir.IlpRouting", [decode_control, decode_update, encode]);

