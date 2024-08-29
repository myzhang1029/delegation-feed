//! Simple CIDR block representation

// SPDX-License-Identifier: AGPL-3.0-or-later

#![allow(clippy::module_name_repetitions)]

use serde::{Deserialize, Serialize};
use std::fmt;
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};

/// A IPv4 CIDR block
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Cidr4 {
    addr: Ipv4Addr,
    prefix_len: u8,
}

impl fmt::Display for Cidr4 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.addr, self.prefix_len)
    }
}
impl Cidr4 {
    pub fn new(addr: Ipv4Addr, prefix_len: u8) -> Self {
        Self { addr, prefix_len }
    }

    pub fn from_num_hosts(start: Ipv4Addr, num_hosts: u32) -> Self {
        let prefix_len = u8::try_from(32 - num_hosts.ilog2()).expect("Invalid prefix length");
        Self {
            addr: start,
            prefix_len,
        }
    }
}

/// A IPv6 CIDR block
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct Cidr6 {
    addr: Ipv6Addr,
    prefix_len: u8,
}

impl fmt::Display for Cidr6 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.addr, self.prefix_len)
    }
}

impl Cidr6 {
    pub fn new(addr: Ipv6Addr, prefix_len: u8) -> Self {
        Self { addr, prefix_len }
    }
}

/// A CIDR block
#[derive(Copy, Clone, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub enum Cidr {
    V4(Cidr4),
    V6(Cidr6),
}

impl fmt::Display for Cidr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Cidr::V4(cidr) => write!(f, "{cidr}"),
            Cidr::V6(cidr) => write!(f, "{cidr}"),
        }
    }
}

impl Cidr {
    pub fn into_parts(self) -> (IpAddr, u8) {
        match self {
            Cidr::V4(cidr) => (IpAddr::V4(cidr.addr), cidr.prefix_len),
            Cidr::V6(cidr) => (IpAddr::V6(cidr.addr), cidr.prefix_len),
        }
    }
}
