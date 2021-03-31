use log::debug;

use rumqttc::{
    Client,
    Publish,
    // Incoming,
    // Packet,
    QoS,
};

#[derive(Debug)]
pub enum Event {
    MQTT { message: Publish },
    Tick,
}

pub struct FSM<'a> {
    state: &'a dyn FSMState,
    client: Client,
}

impl<'a> FSM<'a> {
    const INITIALIZATION: Initialization = Initialization;
    const DAYTIME: Daytime = Daytime;

    pub fn new(client: Client) -> FSM<'a> {
        let mut fsm = FSM {
            state: &Self::INITIALIZATION,
            client,
        };
        fsm.init();
        fsm
    }

    fn init(&mut self) {
        debug!("FSM init");
        self.client
            .subscribe("shellies/+/+/+/announce", QoS::AtLeastOnce)
            .unwrap();
        self.client
            .publish("shellies/command", QoS::AtLeastOnce, false, "announce")
            .unwrap();
    }

    pub fn handle(&mut self, event: Event) {
        match self.state.handle(&mut self.client, event) {
            Some(new_state) => {
                self.state.exit();
                self.state = new_state;
                self.state.enter();
            }
            None => debug!("Staying in the same state."),
        }
    }
}

pub trait FSMState {
    fn enter(&self);
    fn exit(&self);
    fn handle<'a>(&self, client: &mut Client, event: Event) -> Option<&'a dyn FSMState>;
}

// #[derive(Debug)]
// struct Always;

#[derive(Debug)]
pub struct Initialization;

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

    fn handle<'a>(&self, _client: &mut Client, event: Event) -> Option<&'a dyn FSMState> {
        debug!("Initialization handler");
        match event {
            _ => Some(&FSM::DAYTIME),
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

    fn handle<'a>(&self, client: &mut Client, event: Event) -> Option<&'a dyn FSMState> {
        debug!("Daytime handle");
        match event {
            Event::MQTT { message: m } => {
                debug!("{}", m.topic)
            }
            Event::Tick => {
                debug!("Tick!");
                client
                    .publish("test/tick", QoS::AtLeastOnce, false, "Fneee")
                    .unwrap();
            }
        }
        None
    }
}
