
use rumqttc::{
    Publish,
    Client,
    // Incoming,
    // Packet,
    QoS
};


#[derive(Debug)]
pub enum Event {
    MQTT {
        message: Publish
    },
    Tick,
}

pub struct FSM {
    state: Box<dyn FSMState>,
    client: Client,
}

impl FSM {

    fn init(&mut self) {
        debug!("FSM init");
        self.client.subscribe("shellies/+/+/+/announce", QoS::AtLeastOnce).unwrap();
        self.client.publish("shellies/command", QoS::AtLeastOnce, false, "announce").unwrap();

    }

    pub fn handle(&mut self, event: Event) {
        match self.state.handle(&mut self.client, event) {
            Some(new_state) => {
                self.state.exit();
                self.state = new_state;
                self.state.enter();
            },
            None => debug!("Staying in the same state.")
        }
    }
}

trait FSMState {
    fn enter(&self);
    fn exit(&self);
    fn handle(&self, client: &mut Client, event: Event) -> Option<Box<dyn FSMState>>;
}

// #[derive(Debug)]
// struct Always;

#[derive(Debug)]
struct Initialization;

#[derive(Debug)]
struct Daytime;

#[derive(Debug)]
struct Nighttime;

impl FSMState for Initialization {
    fn enter(&self) {
        debug!("Entering initialization.");
    }

    fn exit(&self) {
        debug!("Exiting Initialization");
    }

    fn handle(&self, _client: &mut Client, event: Event) -> Option<Box<dyn FSMState>> {
        debug!("Initialization handler");
        match event {
            _ => Some(Box::new(Daytime))
        }
    }
}

impl FSMState for Daytime {
    fn enter(&self) {
        debug!("Entering Daytime");
    }

    fn exit(&self) {
        debug!("Exiting Daytime");
    }

    fn handle(&self, client: &mut Client, event: Event) -> Option<Box<dyn FSMState>> {
        debug!("Daytime handle");
        match event {
            Event::MQTT { message: m } => { debug!("{}", m.topic) },
            Event::Tick => {
                debug!("Tick!");
                client.publish("test/tick", QoS::AtLeastOnce, false, "Fneee").unwrap();
            },
        }
        None
    }
}

pub fn new(client: Client) -> FSM {
    let mut fsm = FSM { state: Box::new(Initialization), client };
    fsm.init();
    fsm
}
