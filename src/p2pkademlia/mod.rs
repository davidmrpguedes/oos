
use std::arch::is_aarch64_feature_detected;

use std::result;
use rand::random;
use h160;
use hex_literal::hex;

use protobuf::well_known_types::Value_oneof_kind::bool_value;
use protobuf::well_known_types::{BoolValue, Struct};

use sha1::{Sha1,Digest};
use queues::*;


use crate::{hello, p2pkademlia};
use crate::grpc::snd_request;
use crate::hello::k_request::RequestTypeEnum;
use crate::hello::KRequest;


// Node

const TAM_ROUTING_TABLE: usize = 8;
type Kvalue = String;
type Kkey = h160::H160;  // type for nodeid
type Knodelist = Vec<p2pkademlia::Knode>;

type Kbuckets  = Vec<Kbucketrow>;


pub struct Knode {
    ipaddress: String,
    port_no: String,
    nodeid: Kkey,
    routing_table: Kbuckets,
    nodevalue: Kvalue,
    request_queue: queues::Queue<String>, //TODO
    is_bootstrap: bool,
    is_knowallnode: bool
}

#[derive(Debug)]
pub enum KnodeError {
        KeyDoNotExist,
        InvalidBootstrap,
        ValueNotStored,
        ValueNotFound,
        NodeNotResponding,
        NotABootstrapeNode
    }

pub struct Kbucketrow {
    bucket_nodeid: Kkey,
    }





impl Knode {

    pub fn new (ip: String, port: String) -> Self {
        // construtor

        let mut hasher = Sha1::new();

        hasher.update(b"hello world");
        let result = hasher.finalize();
        let nodeid_hex=hex!("2aae6c35c94fcfb415dbe95f408b9ce91ee846ed"); // Ttest


        Knode {
            ipaddress: ip,
            port_no: port,
            nodeid: h160::H160(nodeid_hex),
            routing_table: Vec::new(), // TODO
            nodevalue: "".to_string(),
            request_queue: Queue::new(),
            is_bootstrap: false,
            is_knowallnode: false,
        }
    }

    pub unsafe fn bootstrap (mynode_ori: &Knode, ip_dest: String, port_dest: String) -> Result<bool, KnodeError> {
        // faz o node mynode contactar o node_bootstrap
        // para entrar na rede P2P

        if (mynode_ori.is_bootstrap == false) {
            return Err(KnodeError::NotABootstrapeNode);
        }
        let req = KRequest {
            reqdata: "".to_string(),
            request_id: random(),
            request_knode_id: 0  , // TODO: tem de ser mynode_ori.nodeid, .... como converter h160 para um tipo suportado por protobuf
            request_type: i32::from(RequestTypeEnum::RequestPing),
            request_data: String::from("Empty data request.")
        };
        snd_request(ip_dest, port_dest, req);
        Ok(true)

    }

    pub fn ping(mynode_ori: Knode, nodeid: Kkey) -> Result<bool, KnodeError> {
    // Probes a node to be if it's online

         if (1==0) {
            return Err(KnodeError::NodeNotResponding);
        }
        Ok(true)

    }

    pub fn store(mynode_ori: Knode, key: Kkey, value_to_store: Kvalue) -> Result<bool, KnodeError> {
        // instructs a node to store a key-value pair
         if (1==0) {
            return Err(KnodeError::ValueNotStored);
        }
        Ok(true)
    }

    pub fn find_node(mynode_ori: Knode, key: Kkey) -> Result<Knodelist, KnodeError> {
        // returns information about the k nodes closest to the target id
        if (1==0) {
            return Err(KnodeError::KeyDoNotExist);
        }
        Ok(Vec::new())

    }

    pub fn find_value(mynode_ori: Knode, value: Kkey) -> Result<p2pkademlia::Kvalue,KnodeError> {
        // similar to the find_node RPC, but if the recipient has
        // received a STORE for the given key, it justt returns the stored value.
           if (1==0) {
            return Err(KnodeError::ValueNotFound);
        }
        Ok(Kvalue::new())

    }

}
