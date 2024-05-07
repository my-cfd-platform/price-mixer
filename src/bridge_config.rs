use my_tcp_sockets::TcpClientSocketSettings;

pub struct BridgeConfig {
    pub host_port: String,
    pub name: String,
}

impl BridgeConfig {
    pub fn new(name: String, host_port: String) -> Self {
        Self { name, host_port }
    }
}

impl From<&str> for BridgeConfig {
    fn from(src: &str) -> Self {
        let splitted = src.split("@").collect::<Vec<&str>>();
        Self {
            name: splitted[0].to_string(),
            host_port: splitted[1].to_string(),
        }
    }
}

#[async_trait::async_trait]
impl TcpClientSocketSettings for BridgeConfig {
    async fn get_host_port(&self) -> Option<String> {
        Some(self.host_port.clone())
    }
}
