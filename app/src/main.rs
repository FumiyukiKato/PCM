// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License..

extern crate sgx_types;
extern crate sgx_urts;
extern crate serde;
extern crate serde_json;

use sgx_types::*;
use sgx_urts::SgxEnclave;
use std::fs::File;
use std::io::BufReader;

mod querydata;
use querydata::*;

mod ecalls;
use ecalls::{upload_query_data, init_enclave};

static ENCLAVE_FILE: &'static str = "bin/enclave.signed.so";

fn main() {
    let filename = "data/query.json";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let query_data: QueryData = serde_json::from_reader(reader).unwrap();
    if query_data.client_size != query_data.data.len() {
        println!("[Error] Invalid data format from {}!", filename);
        return;
    }
    let mut retval = sgx_status_t::SGX_SUCCESS;
    
    let enclave = match init_enclave() {
        Ok(r) => {
            println!("[+] Init Enclave Successful {}!", r.geteid());
            r
        },
        Err(x) => {
            println!("[-] Init Enclave Failed {}!", x.as_str());
            return;
        },
    };

    let result = unsafe {
        upload_query_data(
            enclave.geteid(),
            &mut retval,
            query_data.total_data_to_u8().as_ptr() as * const u8,
            query_data.total_size(),
            query_data.size_list().as_ptr() as * const usize,
            query_data.client_size,
            query_data.query_id_list().as_ptr() as * const u64
        )
    };
    
    match result {
        sgx_status_t::SGX_SUCCESS => {
            println!("[+] uploading data is succes!");
        },
        _ => {
            println!("[-] ECALL Enclave Failed {}!", result.as_str());
            return;
        }
    }

    enclave.destroy();
    println!("All process is successful!!");
}
