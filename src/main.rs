use pretty_env_logger;
#[macro_use] extern crate log;

use rumqttc::{MqttOptions, Client, Event, Incoming};
use std::thread;
use std::sync::mpsc;
use std::time::Duration;

mod fsm;

//fn main() -> Result<(), Box<dyn Error>> {
fn main() {

    pretty_env_logger::init();

    let mut mqttoptions = MqttOptions::new("rumqtt-sync", "192.168.174.3", 1883);
    mqttoptions.set_keep_alive(5);

    let (client, mut connection) = Client::new(mqttoptions, 10);

    let mut my_fsm = fsm::new(client);
    let (tx, rx) = mpsc::channel();

    let ticker_tx = tx.clone();
    thread::spawn(move || {
        loop {
            ticker_tx.send(fsm::Event::Tick).unwrap();
            thread::sleep(Duration::from_secs(1));
        };
    });

    thread::spawn(move || {
        for notification in connection.iter() {
            match notification {
                Ok(Event::Incoming(Incoming::Publish(p))) => {
                    tx.send(fsm::Event::MQTT {message: p}).unwrap();
                },
                Ok(Event::Incoming(i)) => { trace!("Incoming: {:?}", i); },
                Ok(Event::Outgoing(o)) => { trace!("Outgoing: {:?}",o); },
                Err(_) => { panic!("I panicked."); },
            }
        }
    });

    for received in rx {
        my_fsm.handle(received);
    }
}
