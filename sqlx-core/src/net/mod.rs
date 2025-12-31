mod socket;
pub mod tls;

pub use socket::{
    BufferedSocket, Socket, SocketIntoBox, WithSocket, WriteBuffer, connect_tcp, connect_uds,
};
