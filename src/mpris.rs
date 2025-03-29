use std::sync::OnceLock;

use log::info;
use mpris_server::Player;
use tokio::sync::mpsc::{self, UnboundedReceiver};

static TX: OnceLock<mpsc::UnboundedSender<Command>> = OnceLock::new();

pub enum Command {
    SpawnPlayer,
}

pub async fn mpris_local_task(mut rx: UnboundedReceiver<Command>) {
    info!("Spawning bg async task");
    while let Some(m) = rx.recv().await {
        match m {
            Command::SpawnPlayer => {
                info!("Spawning player");
                let player = Player::builder("Test.Application")
                    .can_play(true)
                    .can_pause(true)
                    .can_go_previous(true)
                    .can_go_next(true)
                    .build()
                    .await
                    .unwrap();

                // Handle `PlayPause` method call
                player.connect_play_pause(|_player| {
                    info!("PlayPause");
                });

                // Handle `Previous` method call
                player.connect_previous(|_player| {
                    info!("Previous");
                });

                // Handle `Next` method call
                player.connect_next(|_player| {
                    info!("Next");
                });
                player.run().await;
            }
        }
    }
}

fn get_tx() -> &'static mpsc::UnboundedSender<Command> {
    TX.get_or_init(|| {
        let (tx, rx) = mpsc::unbounded_channel();
        let rt = tokio::runtime::Builder::new_current_thread()
            .enable_io()
            .build()
            .unwrap();
        std::thread::spawn(move || rt.block_on(crate::mpris::mpris_local_task(rx)));
        tx
    })
}

pub fn spawn_player() {
    get_tx().send(Command::SpawnPlayer).unwrap();
}
