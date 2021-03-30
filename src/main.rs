use log::trace;
use pretty_env_logger;

use rumqttc::{Client, Connection, Event, Incoming, MqttOptions};
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

mod fsm;
use fsm::FSM;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;

fn main() {
    pretty_env_logger::init();

    let mut mqttoptions = MqttOptions::new("rumqtt-sync", "192.168.174.3", 1883);
    mqttoptions.set_keep_alive(5);

    let (client, connection) = Client::new(mqttoptions, 10);

    let mut my_fsm = FSM::new(client);
    let (tx, rx) = mpsc::channel();

    let ticker = start_ticker(tx.clone());
    let notification_receiver = start_notification_receiver(connection, tx);

    for received in rx {
        my_fsm.handle(received);
    }

    // The unwrap will cause the joins to panic if the joined tread itself panicked
    ticker.join().unwrap();
    notification_receiver.join().unwrap();
}

fn start_ticker(tx: Sender<fsm::Event>) -> JoinHandle<()> {
    thread::spawn(move || {
        loop {
            tx.send(fsm::Event::Tick).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    })
}

fn start_notification_receiver(
    mut connection: Connection,
    tx: Sender<fsm::Event>,
) -> JoinHandle<()> {
    thread::spawn(move || {
        for notification in connection.iter() {
            match notification {
                Ok(Event::Incoming(Incoming::Publish(p))) => {
                    tx.send(fsm::Event::MQTT { message: p }).unwrap();
                }
                Ok(Event::Incoming(i)) => {
                    trace!("Incoming: {:?}", i);
                }
                Ok(Event::Outgoing(o)) => {
                    trace!("Outgoing: {:?}", o);
                }
                Err(_) => {
                    panic!("I panicked.");
                }
            }
        }
    })
}
