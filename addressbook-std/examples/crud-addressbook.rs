use std::io::stderr;

use addressbook::{carddav::Client, tcp, Addressbook};
use addressbook_std::Connector;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

fn main() {
    tracing_subscriber::registry()
        .with(fmt::layer().with_writer(stderr))
        .with(EnvFilter::from_default_env())
        .init();

    let client = Client::new_from_envs();

    let mut addressbook = Addressbook::default();
    addressbook.name = "Test".into();
    addressbook.desc = Some("Testing addressbook".into());

    let mut tcp = Connector::connect(&client.config).unwrap();
    let mut flow = client.create_addressbook(addressbook);
    while let Some(io) = flow.next() {
        match io {
            tcp::Io::Read => {
                tcp.read(&mut flow).unwrap();
            }
            tcp::Io::Write => {
                tcp.write(&mut flow).unwrap();
            }
        }
    }

    let mut addressbook = flow.output().unwrap();
    println!();
    println!("created addressbook: {addressbook:#?}");

    addressbook.name = "Test updated".into();
    addressbook.desc = Some("".into());
    addressbook.color = Some("#abcdef".into());

    let mut tcp = Connector::connect(&client.config).unwrap();
    let mut flow = client.update_addressbook(addressbook);
    while let Some(io) = flow.next() {
        match io {
            tcp::Io::Read => {
                tcp.read(&mut flow).unwrap();
            }
            tcp::Io::Write => {
                tcp.write(&mut flow).unwrap();
            }
        }
    }

    let addressbook = flow.output().unwrap();
    println!();
    println!("updated addressbook: {addressbook:#?}");

    let mut tcp = Connector::connect(&client.config).unwrap();
    let mut flow = client.delete_addressbook(&addressbook.id);
    while let Some(io) = flow.next() {
        match io {
            tcp::Io::Read => {
                tcp.read(&mut flow).unwrap();
            }
            tcp::Io::Write => {
                tcp.write(&mut flow).unwrap();
            }
        }
    }

    let success = flow.output().unwrap();
    println!();
    println!("addressbook {} deleted: {success}", addressbook.id);
}
