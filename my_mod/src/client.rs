use ambient_api::prelude::*;
use packages::this::messages::Teleport;

#[main]
pub fn main() {
  player_teleport.el().spawn_interactive();
}

fn player_teleport(_hooks: &mut Hooks) -> Element {
  Button::new("Teleport", |_| Teleport.send_server_reliable()).el()
}