use std::{io::BufReader, net::TcpStream};

pub struct TcpCnx {
    pub cnx: BufReader<TcpStream>,
    pub buf: bytes::BytesMut,
    // TODO: TIMEOUT
    // time_connected: Instant,
    // read_timeout: time::Duration,
    // write_timeout: time::Duration,
}

impl TcpCnx {
    pub fn new(cnx: TcpStream) -> Self {
        TcpCnx {
            cnx: BufReader::new(cnx),
            buf: bytes::BytesMut::with_capacity(1024), // time_connected: Instant::now(),
                                                       // read_timeout: time::Duration::new(2, 0),
                                                       // write_timeout: time::Duration::new(2, 0)
        }
    }
}
