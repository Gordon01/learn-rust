use std::net::SocketAddrV4;

struct Server {
    tcp: SocketAddrV4,
    name: String,
    max_connections: u32,
}

impl Server {
    pub fn builder() -> ServerBuilder<NoSocket> {
        ServerBuilder::default()
    }
}

#[derive(Default)]
struct ServerBuilder<Socket> {
    tcp: Socket,
    name: String,
    max_connections: u32,
}

#[derive(Default)]
struct NoSocket;

impl ServerBuilder<NoSocket> {
    pub fn tcp(self, tcp: SocketAddrV4) -> ServerBuilder<SocketAddrV4> {
        ServerBuilder {
            tcp,
            name: self.name,
            max_connections: self.max_connections,
        }
    }
}

impl ServerBuilder<SocketAddrV4> {
    pub fn build(self) -> Server {
        Server {
            tcp: self.tcp,
            name: self.name,
            max_connections: self.max_connections,
        }
    }
}

impl<Socket> ServerBuilder<Socket> {
    pub fn name(mut self, name: impl ToString) -> Self {
        self.name = name.to_string();
        self
    }

    pub fn max_connections(mut self, max: u32) -> Self {
        self.max_connections = max;
        self
    }
}

fn main() {
    let s = Server::builder()
        .name("my_srv")
        .max_connections(32)
        // .build()
        .tcp(SocketAddrV4::new([127, 0, 0, 1].into(), 6969))
        .name("my_2")
        .max_connections(324)
        .build();
}
