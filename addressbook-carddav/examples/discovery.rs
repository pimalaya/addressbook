use std::io::stderr;

use addressbook::carddav::Client;
use addressbook_carddav::Connector;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(stderr))
        .with(EnvFilter::from_default_env())
        .init();

    let client = Client::new_from_envs();

    // Current user principal

    // NOTE: ideally, this should be needed once in order to re-use
    // the connection. It depends on the HTTP version returned by the
    // server.
    let mut tcp = Connector::connect(&client.config.hostname, client.config.port).unwrap();
    let mut flow = client.current_user_principal();
    while let Some(io) = flow.next() {
        tcp.execute(&mut flow, io).unwrap()
    }

    let current_user_principal = flow.output().unwrap();
    println!();
    println!("current user principal: {current_user_principal:?}");

    let current_user_principal = current_user_principal.unwrap_or(String::from("/"));

    // Addressbook home set

    tcp = Connector::connect(&client.config.hostname, client.config.port).unwrap();
    let mut flow = client.addressbook_home_set(current_user_principal);
    while let Some(io) = flow.next() {
        tcp.execute(&mut flow, io).unwrap()
    }

    let addressbook_home_set = flow.output().unwrap();
    println!("addressbook home set: {addressbook_home_set:?}");
}
