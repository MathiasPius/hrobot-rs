use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

use crate::{APIResult, Error, Robot};

#[derive(Debug, Deserialize)]
pub struct ReverseDNS {
    pub ip: Ipv4Addr,
    pub ptr: String,
}

#[derive(Debug, Deserialize)]
pub struct ReverseDNSResponse {
    pub rdns: ReverseDNS,
}

#[derive(Serialize)]
struct PtrRecord<'a> {
    pub ptr: &'a str,
}

pub trait ReverseDNSRobot {
    fn list_rdns(&self) -> Result<Vec<ReverseDNS>, Error>;
    fn get_rdns(&self, ip: Ipv4Addr) -> Result<Option<ReverseDNS>, Error>;
    fn create_rdns(&self, ip: Ipv4Addr, ptr: &str) -> Result<Option<ReverseDNS>, Error>;
    fn update_rdns(&self, ip: Ipv4Addr, ptr: &str) -> Result<Option<ReverseDNS>, Error>;
    fn delete_rdns(&self, ip: Ipv4Addr) -> Result<Option<ReverseDNS>, Error>;
}

impl ReverseDNSRobot for Robot {
    fn list_rdns(&self) -> Result<Vec<ReverseDNS>, Error> {
        let result: APIResult<Vec<ReverseDNSResponse>> = self.get("/rdns")?;

        match result {
            APIResult::Ok(s) => Ok(s.into_iter().map(|s| s.rdns).collect()),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn get_rdns(&self, ip: Ipv4Addr) -> Result<Option<ReverseDNS>, Error> {
        let result: APIResult<ReverseDNSResponse> = self.get(&format!("/rdns/{}", ip))?;

        match result {
            APIResult::Ok(s) => Ok(Some(s.rdns)),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn create_rdns(&self, ip: Ipv4Addr, ptr: &str) -> Result<Option<ReverseDNS>, Error> {
        let result: APIResult<ReverseDNSResponse> =
            self.put(&format!("/rdns/{}", ip), PtrRecord { ptr })?;

        match result {
            APIResult::Ok(s) => Ok(Some(s.rdns)),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn update_rdns(&self, ip: Ipv4Addr, ptr: &str) -> Result<Option<ReverseDNS>, Error> {
        let result: APIResult<ReverseDNSResponse> =
            self.post(&format!("/rdns/{}", ip), PtrRecord { ptr })?;

        match result {
            APIResult::Ok(s) => Ok(Some(s.rdns)),
            APIResult::Error(e) => Err(e.into()),
        }
    }

    fn delete_rdns(&self, ip: Ipv4Addr) -> Result<Option<ReverseDNS>, Error> {
        let result: APIResult<ReverseDNSResponse> = self.delete(&format!("/rdns/{}", ip))?;

        match result {
            APIResult::Ok(s) => Ok(Some(s.rdns)),
            APIResult::Error(e) => Err(e.into()),
        }
    }
}
