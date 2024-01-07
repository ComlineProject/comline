// Relative Modules
mod generated;
mod server;
mod client;


#[tokio::test]
async fn send_name_from_client_and_receive_hello_from_server() {
    // This entry point is just an example of simulation, you would do differently
    // if not simulating both parts

    tokio::task::LocalSet::new().run_until(async move {
        // Lets spawn a handle for the server, pretending its a different process
        let server_thread = tokio::task::spawn_local(server::main());

        // And for the client we just run in our existing thread
        let client_thread = tokio::task::spawn(client::main());

        server_thread.await.unwrap();
        client_thread.await.unwrap();
    }).await;
}

