// Standard Uses

// Crate Uses

// External Uses


pub mod provider {
    // Standard Uses
    use std::net::SocketAddr;
    use std::sync::{Arc, RwLock};

    // Crate Uses
    use crate::setup::communication::provider::CommunicationProvider;
    use crate::setup::call_system::Callback;

    // External Uses
    use eyre::Result;
    use tokio::net::{TcpListener, TcpStream};
    use tokio::io::AsyncReadExt;
    use async_trait::async_trait;


    pub struct TcpProvider {
        listener: TcpListener,
        pub connection_count: usize,
        data_received_callback: Vec<Arc<RwLock<dyn Callback>>>,
    }

    impl TcpProvider {
        // const INCOMING_DATA_MIN_LEN: usize = 1;

        pub async fn with_address(
            address: &str,
            // data_received_callback: fn(&[u8])
        ) -> Result<Self> {
            Ok(Self {
                listener: TcpListener::bind(address).await?,
                connection_count: 0,
                data_received_callback: vec![],
            })
        }

        pub fn and_callback(mut self, callback: Arc<RwLock<dyn Callback>>) -> Self
        {
            self.data_received_callback.push(callback);
            self
        }

        pub fn into_threaded(self) -> Arc<RwLock<Self>> { Arc::new(RwLock::new(self)) }

        pub async fn listen_connections(&mut self, /*call_system: &mut dyn CallSystem*/) {
            loop { self.listen_incoming_connection(/*call_system*/).await }
        }

        pub async fn listen_incoming_connection(
            &self,
            // call_system: &mut dyn CallSystem
        ) {
            let (stream, address) = self.listener.accept().await.unwrap();
            stream.set_nodelay(true).unwrap();

            // TODO: Introduce interior mutability for count, and/or atomics
            //self.connection_count += 1;
            self.listen_stream(stream, address, /*call_system*/).await;
            //self.connection_count -= 1;
        }

        pub async fn listen_stream(
            &self, mut stream: TcpStream, address: SocketAddr,
            //call_system: &mut dyn CallSystem
        ) {
            let mut buf = [0; 1024];

            loop {
                let length = match stream.read(&mut buf).await {
                    Ok(0) => return, // Stream closed
                    Ok(n) => n,
                    Err(e) => {
                        // TODO: Shouldn't be a panic and needs to do proper error setup
                        panic!("Couldn't read on stream: {e}");
                    }
                };

                /*
                if length < Self::INCOMING_DATA_MIN_LEN {
                    panic!(
                        "Incoming packet is not big enough, got {} but expected at least {} bytes",
                        length, Self::INCOMING_DATA_MIN_LEN
                    )
                }
                */
                let data = &buf[..length];

                println!(
                    "[Provider] {} - Incoming data ({} bytes, first 10 bytes: {:?}",
                    address, length, &data
                );

                for callback in &self.data_received_callback {
                    callback.write().unwrap().on_received_data(&data)
                }
            }
        }
    }

    #[async_trait]
    impl CommunicationProvider for TcpProvider {
        fn add_received_data_callback(&mut self, callback: Arc<RwLock<dyn Callback>>) {
            self.data_received_callback.push(callback)
        }

        async fn listen_for_connections(&mut self, /*call_system: &mut dyn CallSystem*/) {
            self.listen_connections(/*call_system*/).await;
        }
    }
}

pub mod consumer {
    // Standard Uses
    use std::io::Write;
    use std::net::TcpStream;
    use std::sync::{Arc, RwLock};

    // Crate Uses
    use crate::setup::communication::consumer::CommunicationConsumer;

    // External Uses
    use eyre::Result;
    use async_trait::async_trait;


    pub struct TcpConsumer {
        stream: TcpStream,
        data_received_callback: Vec<fn(&[u8])>,
    }
    impl TcpConsumer {
        pub fn into_threaded(self) -> Arc<RwLock<Self>> { Arc::new(RwLock::new(self)) }
    }

    impl TcpConsumer {
        pub fn with_address(address: &str) -> Result<Self> {
            Ok(Self {
                stream: TcpStream::connect(address)?,
                data_received_callback: vec![]
            })
        }
        pub fn and_callback(mut self, callback: fn(&[u8])) -> Self {
            self.data_received_callback.push(callback);
            self
        }
    }

    #[async_trait]
    impl CommunicationConsumer for TcpConsumer {
        async fn connect_to_provider(&self) {
            todo!()
        }

        #[allow(unused)]
        fn send_data(&mut self, data: &[u8]) -> Result<()> {
            println!(
                "Sending data ({} bytes, first 10: {:?}): {:?}",
                data.len(), &data[..10], String::from_utf8_lossy(&data[..10])
            );

            self.stream.write_all(data)?;
            self.stream.flush();

            Ok(())
        }

        #[allow(unused)]
        async fn send_data_async(&mut self, data: &[u8]) -> Result<()> {
            todo!()
        }
    }
}
