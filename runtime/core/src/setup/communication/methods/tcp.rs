// Standard Uses

// Crate Uses

// External Uses


pub mod provider {
    // Standard Uses
    use std::net::SocketAddr;

    // Crate Uses
    use crate::setup::communication::provider::CommunicationProvider;
    use crate::setup::call_system::CallSystem;
    use crate::setup::message_format::MessageFormat;

    // External Uses
    use eyre::Result;
    use tokio::net::{TcpListener, TcpStream};
    use async_trait::async_trait;
    use tokio::io::AsyncReadExt;

    pub struct TcpProvider {
        listener: TcpListener,
        pub connection_count: usize,
        // data_received_callback: fn(&[u8]),
    }

    impl TcpProvider {
        // const INCOMING_DATA_MIN_LEN: usize = 1;

        pub async fn with_address_and_callback(
            address: &str,
            // data_received_callback: fn(&[u8])
        ) -> Result<Self> {
            Ok(Self {
                listener: TcpListener::bind(address).await?,
                connection_count: 0
                // data_received_callback
            })
        }

        pub async fn listen_connections(
            &mut self, call_system: &mut dyn CallSystem, message_format: &dyn MessageFormat
        ) {
            loop {
                self.listen_incoming_connection(call_system, message_format).await
            }
        }

        pub async fn listen_incoming_connection(
            &mut self, call_system: &mut dyn CallSystem, message_format: &dyn MessageFormat
        ) {
            let (stream, address) = self.listener.accept().await.unwrap();
            stream.set_nodelay(true).unwrap();

            self.connection_count += 1;
            self.listen_stream(stream, address, call_system, message_format).await;
            self.connection_count -= 1;
        }

        #[allow(unused)]
        pub async fn listen_stream(
            &self, mut stream: TcpStream, address: SocketAddr,
            call_system: &mut dyn CallSystem, message_format: &dyn MessageFormat
        ) {
            let mut buf = [0; 1024];

            loop {
                // let mut buf = vec![];
                let length = stream.read(&mut buf).await;

                match length {
                    Ok(n) if n == 0 => return, // Stream closed
                    Ok(n) => n,
                    Err(e) => {
                        panic!("Couldn't read on stream: {e}");
                    }
                };

                /*
                if length < Self::INCOMING_DATA_MIN_LEN {
                    panic!(
                        "Incoming packet is not lengthy enough, got {} but expected at least {}",
                        length, Self::INCOMING_DATA_MIN_LEN
                    )
                }
                */

                println!("Incoming data: {:?}", buf);
                // let serialized = message_format.deserialize(&buf).unwrap();
                let casted = call_system.on_receive_data(Box::new(buf));

                // (self.data_received_callback)(&buf)
            }
        }
    }

    #[async_trait]
    impl CommunicationProvider for TcpProvider {
        async fn listen_for_connections(
            &mut self, call_system: &mut dyn CallSystem, message_format: &dyn MessageFormat
        ) {
            self.listen_connections(call_system, message_format).await;
        }
    }
}

pub mod consumer {
    // Standard Uses
    use std::net::TcpStream;

    // Crate Uses
    use crate::setup::communication::consumer::CommunicationConsumer;

    // External Uses
    use eyre::Result;
    use async_trait::async_trait;


    #[allow(unused)]
    pub struct TcpConsumer {
        stream: TcpStream,
        // data_received_callback: fn(&[u8]),
    }

    impl TcpConsumer {
        pub fn with_address_and_callback(
            address: &str, // data_received_callback: fn(&[u8])
        ) -> Result<Self> {
            Ok(Self {
                stream: TcpStream::connect(address)?,
                // data_received_callback
            })
        }
    }

    #[async_trait]
    impl CommunicationConsumer for TcpConsumer {
        async fn connect_to_provider(&self) {
            todo!()
        }
    }
}