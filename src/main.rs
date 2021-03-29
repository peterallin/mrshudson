use pretty_env_logger;
#[macro_use] extern crate log;

use std::error::Error;
use rumqttc::{MqttOptions, AsyncClient, Event, Incoming};
use tokio::time::{sleep, Duration};
use std::sync::{Arc, Mutex};

mod fsm;

#[tokio::main(worker_threads = 1)]
async fn main() -> Result<(), Box<dyn Error>> {

    pretty_env_logger::init();

    let mut mqttoptions = MqttOptions::new("rumqtt-sync", "192.168.174.3", 1883);
    mqttoptions.set_keep_alive(5);

    let (client, mut eventloop) = AsyncClient::new(mqttoptions, 10);

    let my_fsm = Arc::new(Mutex::new(fsm::new(client)));

    let ticker = tokio::task::spawn(async move {
        loop {
            debug!("Tick!");
            // my_fsm.handle(fsm::Event::Tick);
            sleep(Duration::from_secs(1)).await;
        }
    });

    loop {
        match eventloop.poll().await {
            Ok(Event::Incoming(Incoming::Publish(p))) => {
                // debug!("Topic: {}, Payload: {:?}", p.topic, p.payload);
                let mut foo = my_fsm.lock().unwrap();
                foo.handle(fsm::Event::MQTT {message: p});
            }
            Ok(Event::Incoming(i)) => {
                trace!("Incoming: {:?}", i);
            }
            Ok(Event::Outgoing(o)) => {
                trace!("Outgoing: {:?}", o);
            }
            Err(e) => {
                error!("Error: {:?}", e);
                continue;
            }
        };
    };

}
