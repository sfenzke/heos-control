use telnet::Telnet;
use regex::Regex;
use ssdp_client::URN;
use std::{time::Duration};
use lazy_static::lazy_static;
use futures::{prelude::*, executor::block_on};

use crate::HeosError;

pub struct Connection {
    connection: Telnet,
}

impl Connection {
    async fn get_first_heos_ip() -> Result<Option<String>, ssdp_client::Error> {
        lazy_static!(
            static ref IP_EXTRACTOR:Regex = 
                Regex::new(r"http:\/\/(?<ip>[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}).*")
                .expect("invalid regexp");
        );
    
        let timeout = Duration::from_secs(3);
        let search_target = URN::device("schemas-denon-com", "ACT-Denon", 1).into();
        let mut responses = ssdp_client::search(&search_target, timeout, 2).await?;
    
        if let Some(response) = responses.next().await {
            let response = response?;
            
            if let Some(captures) = IP_EXTRACTOR.captures(response.location()) {
                return  Ok(Some(captures["ip"].to_owned()));
            }
        }
        else {
            return Ok(None);
        }

        return Ok(None)
    }

    pub fn connect() -> Result<Self, HeosError> {
        if let Some(heos_ip) =
            block_on(Connection::get_first_heos_ip())? {
                match Telnet::connect((heos_ip, 1255), 1024) {
                    Ok(connection) => {
                        Ok(Self{connection})
                    }
                    Err(error) => {
                        return Err(HeosError::Connect(error))
                    }
                }
        }
        else {
            return Err(HeosError::NoDevices());
        }
    }

}