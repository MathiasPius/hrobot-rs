use serde::{Deserialize, Serialize};
use std::net::Ipv4Addr;

use crate::{Error, SyncRobot};

#[derive(Debug, Deserialize)]
pub struct ReverseDNS {
    pub ipv4: Ipv4Addr,
    pub ptr: String,
}

#[derive(Debug, Deserialize)]
struct ReverseDNSResponse {
    pub rdns: ReverseDNS,
}

impl From<ReverseDNSResponse> for ReverseDNS {
    fn from(r: ReverseDNSResponse) -> Self {
        r.rdns
    }
}

#[derive(Serialize)]
struct PtrRecord<'a> {
    pub ptr: &'a str,
}

pub trait ReverseDNSRobot {
    fn list_rdns(&self) -> Result<Vec<ReverseDNS>, Error>;
    fn get_rdns(&self, ip: Ipv4Addr) -> Result<ReverseDNS, Error>;
    fn create_rdns(&self, ip: Ipv4Addr, ptr: &str) -> Result<ReverseDNS, Error>;
    fn update_rdns(&self, ip: Ipv4Addr, ptr: &str) -> Result<ReverseDNS, Error>;
    fn delete_rdns(&self, ip: Ipv4Addr) -> Result<ReverseDNS, Error>;
}

impl<T> ReverseDNSRobot for T
where
    T: SyncRobot,
{
    fn list_rdns(&self) -> Result<Vec<ReverseDNS>, Error> {
        self.get::<Vec<ReverseDNSResponse>>("/rdns")
            .map(|r| r.into_iter().map(ReverseDNS::from).collect())
    }

    fn get_rdns(&self, ip: Ipv4Addr) -> Result<ReverseDNS, Error> {
        self.get::<ReverseDNSResponse>(&format!("/rdns/{}", ip))
            .map(ReverseDNS::from)
    }

    fn create_rdns(&self, ip: Ipv4Addr, ptr: &str) -> Result<ReverseDNS, Error> {
        self.put::<ReverseDNSResponse, PtrRecord>(&format!("/rdns/{}", ip), PtrRecord { ptr })
            .map(ReverseDNS::from)
    }

    fn update_rdns(&self, ip: Ipv4Addr, ptr: &str) -> Result<ReverseDNS, Error> {
        self.post::<ReverseDNSResponse, PtrRecord>(&format!("/rdns/{}", ip), PtrRecord { ptr })
            .map(ReverseDNS::from)
    }

    fn delete_rdns(&self, ip: Ipv4Addr) -> Result<ReverseDNS, Error> {
        self.delete::<ReverseDNSResponse, ()>(&format!("/rdns/{}", ip), ())
            .map(ReverseDNS::from)
    }
}
