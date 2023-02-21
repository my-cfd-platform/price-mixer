use my_tcp_sockets::TcpClient;

pub struct BridgeConnection {
    pub host_port: String,
    pub tcp_client: TcpClient,
}
