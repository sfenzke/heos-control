use serde_json::Value;
use telnet::{Telnet, Event};
use regex::Regex;
use ssdp_client::URN;
use std::{time::Duration};
use lazy_static::lazy_static;
use futures::{prelude::*, executor::block_on};

use crate::Device;
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

    fn query_device(&mut self, query: &str) -> Result<String, HeosError> {
        let query = format!("heos://{}\r\n", query);

        self.connection.write(query.as_bytes())?;
        if let Event::Data(buffer) =  self.connection.read()? {
            Ok(String::from_utf8_lossy(&(*buffer)).into_owned())
        }
        else {
            Err(HeosError::QuerryError(format!("Error while running querry {}", &query)))
        }
    } 

    pub fn get_devices(&mut self) -> Result<Vec<Device>, HeosError> {
        let response = self.query_device("player/get_players")?;
        let json: Value = serde_json::from_str(&response)?;
        let mut result:Vec<Device> = Vec::new();

        if let Some(payload) = json["payload"].as_array() {
            for player in payload {
                let gid = if player["gid"] == serde_json::Value::Null { None } 
                                         else { Some(player["gid"].as_i64()).unwrap() };

                result.push(Device::new(player["name"].as_str().unwrap().to_owned(), 
                                        player["pid"].as_i64().unwrap(), 
                                        gid));
            }
            return Ok(result);
        }
        else {
            Err(HeosError::NoDevices())
        }

    }
}