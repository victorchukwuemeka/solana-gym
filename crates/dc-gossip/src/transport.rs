//! UDP transport layer for dc-gossip.
//! Single responsibility: send and receive raw bytes.
//! No protocol knowledge lives here — just the socket.

use anyhow::Result;
use std::net::SocketAddr;
use tokio::net::UdpSocket;

pub struct Transport {
    socket: UdpSocket,
}

impl Transport {
    /// Bind to a local address and return a ready transport
    pub async fn new(bind_addr: &str) -> Result<Self> {
        let socket = UdpSocket::bind(bind_addr).await?;
        tracing::info!("transport bound to {}", bind_addr);
        Ok(Self { socket })
    }

    /// Send raw bytes to a remote address
    pub async fn send(&self, data: &[u8], to: &SocketAddr) -> Result<()> {
        self.socket.send_to(data, to).await?;
        Ok(())
    }

    /// Receive raw bytes and the sender's address
    pub async fn recv(&self) -> Result<(Vec<u8>, SocketAddr)> {
        let mut buffer = vec![0u8; 65535];
        let (len, addr) = self.socket.recv_from(&mut buffer).await?;
        buffer.truncate(len);
        let data_in_bytes = buffer;
        Ok((data_in_bytes, addr))
    }

    /// Get the local address we are bound to
    pub async fn local_addr(&self) -> Result<SocketAddr> {
        Ok(self.socket.local_addr()?)
    }
}
