use super::EventBlockKind;
use crate::{_exports::JsonBlocks, aux::parse_block::ParseJsonBlock};

#[test]
fn test_unit() {
    let val = serde_json::json!( {
        "mH2CP36wcQ`]/+}0#tf/": {
            "opcode":"event_whenflagclicked",
            "next":"L[[LH(CNG`srmf6|f}fo",
            "parent":null,
            "inputs":{},
            "fields":{},
            "shadow":false,
            "topLevel":true,
            "x":79,
            "y":113
        }
    });
    let json = JsonBlocks::new(&val);
    let parsed = EventBlockKind::ctx_parse_json_block(json, "mH2CP36wcQ`]/+}0#tf/");
    if let Err(err) = parsed {
        panic!("{err}\n{err:#?}");
    }
}
#[test]
fn test_one_attr() {
    let json = serde_json::json!({
        "id" : {
            "opcode":"event_whenbroadcastreceived",
            "next":"6/`dK3Ep|=~TIc0?5KpZ",
            "parent":null,
            "inputs":{},
            "fields":{
                    "BROADCAST_OPTION":[
                        "Nachricht1",
                        "5u_E}Qk{6HKd}T[$`Z#i"
                    ]
            },
            "shadow":false,
            "topLevel":true,
            "x":98,
            "y":3515
        }
    });
    let val = JsonBlocks::new(&json);
    let parsed = EventBlockKind::ctx_parse_json_block(val, "id");
    if let Err(err) = parsed {
        panic!("{err}\n{err:#?}");
    }
}

#[test]
fn test_greater() {
    let json = serde_json::json!({
        "+L7mzXPq%r%nVzvm8#B-": {
            "opcode": "event_whengreaterthan",
            "next": null,
            "parent": null,
            "inputs": {
                "VALUE": [
                    1,
                    [4,"10"]
                ]
            },
            "fields":{
                "WHENGREATERTHANMENU": ["LOUDNESS",null]
            },
            "shadow":false,
            "topLevel":true,
            "x":101,
            "y":3370
        }
    });
    let val = JsonBlocks::new(&json);
    let parsed = EventBlockKind::ctx_parse_json_block(val, "+L7mzXPq%r%nVzvm8#B-");
    if let Err(err) = parsed {
        panic!("{err}\n{err:#?}");
    }
}
