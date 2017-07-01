extern crate discord;
extern crate rand;

use discord::model::{Event, ServerId, Presence};
use std::thread;
use std::sync::{mpsc, Arc};
use {Context, event_handler};

pub struct ServerHandler {
    id: ServerId,
    recever: mpsc::Receiver<Event>,
}

impl ServerHandler {
    pub fn create(id: ServerId, recever: mpsc::Receiver<Event>, context: Arc<Context>) {
        let mut t = ServerHandler { id, recever };
        thread::spawn(move || t.main(context));
    }

    fn main(&mut self, context: Arc<Context>) {
        loop {
            let event = match self.recever.recv() {
                Err(err) => {
                    println!("{}", err);
                    continue;
                }
                Ok(event) => event, 
            };
            match event {
                Event::MessageCreate(message) => {
                    event_handler::handle_message_create(message, &context.state.lock().unwrap());
                }
                Event::ServerDelete(..) => {
                    println!("[ServerDelete] Server delete sent.  Closing thread {}",
                             self.id);
                    break;
                }
                Event::PresenceUpdate {
                    presence: Presence {
                        game: Some(game),
                        user_id,
                        ..
                    },
                    server_id: Some(server_id),
                    roles: _,
                } => {
                    println!("[PresenceUpdate] matched game start.");
                    event_handler::handle_presence_update_start_game(&context.discord,
                                                                     game,
                                                                     user_id,
                                                                     server_id)
                }
                Event::Unknown(name, data) => {
                    // log unknown event types for later study
                    println!("[Unknown Event] {}: {:?}", name, data);
                }
                x => {
                    println!("[Debug] uncaught event  = {:?}", x);
                } // discard other known events
            }
        }
    }
}