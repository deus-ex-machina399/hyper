use std::io;
use std::net::{IpAddr, SocketAddr, ToSocketAddrs};
use std::thread;
use std::vec;

use ::futures::{Future, Poll};
use ::futures_cpupool::{CpuPool, CpuFuture};

#[derive(Clone)]
pub struct Dns {
    pool: CpuPool,
}

impl Dns {
    pub fn new(threads: usize) -> Dns {
        Dns {
            pool: CpuPool::new(threads)
        }
    }

    pub fn resolve<S: Into<String>>(&self, hostname: S) -> Query {
        let hostname = hostname.into();
        Query(self.pool.spawn_fn(move || work(hostname)))
    }
}

pub struct Query(CpuFuture<IpAddrs, io::Error>);

impl Future for Query {
    type Item = IpAddrs;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.0.poll()
    }
}

pub struct IpAddrs {
    iter: vec::IntoIter<SocketAddr>,
}

impl Iterator for IpAddrs {
    type Item = IpAddr;
    #[inline]
    fn next(&mut self) -> Option<IpAddr> {
        self.iter.next().map(|addr| addr.ip())
    }
}

pub type Answer = io::Result<IpAddrs>;

fn work(hostname: String) -> Answer {
    debug!("resolve {:?}", hostname);
    (&*hostname, 80).to_socket_addrs().map(|i| IpAddrs { iter: i })
}
