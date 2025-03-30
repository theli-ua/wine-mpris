use std::{mem, sync::OnceLock};

use log::info;
use mpris_server::Player;
use tokio::{
    sync::mpsc::{self, UnboundedReceiver},
    task::LocalSet,
};
use windows::Win32::{Foundation::HWND, UI::WindowsAndMessaging::GetWindowTextW};

static TX: OnceLock<mpsc::UnboundedSender<Command>> = OnceLock::new();

pub enum Command {
    SpawnPlayer(HWND),
}

pub async fn mpris_local_task(mut rx: UnboundedReceiver<Command>) {
    info!("Spawning bg async task");
    info!(
        "rt flavor {:?}",
        tokio::runtime::Handle::current().runtime_flavor()
    );
    while let Some(m) = rx.recv().await {
        match m {
            Command::SpawnPlayer(hwnd) => {
                let window_title = unsafe {
                    let mut v: [u16; 255] = mem::uninitialized();
                    let read_len = GetWindowTextW(hwnd, &mut v[..]);
                    String::from_utf16_lossy(&v[0..read_len as usize])
                };
                let window_title = window_title.replace(' ', ".");
                info!("Spawning player");
                let player = Player::builder(&window_title)
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
                player
                    .set_playback_status(mpris_server::PlaybackStatus::Playing)
                    .await
                    .unwrap();
                // player.run().await;
                tokio::task::spawn_local(async move {
                    player.run().await;
                    info!("DDDDONE");
                });
            }
        }
    }
}

fn get_tx() -> &'static mpsc::UnboundedSender<Command> {
    TX.get_or_init(|| {
        let (tx, rx) = mpsc::unbounded_channel();
        std::thread::spawn(move || {
            let rt = tokio::runtime::Builder::new_current_thread()
                .enable_io()
                .build()
                .unwrap();
            rt.block_on(LocalSet::new().run_until(crate::mpris::mpris_local_task(rx)));
            panic!("TERMINATOR");
        });
        tx
    })
}

pub fn spawn_player(hwnd: HWND) {
    get_tx().send(Command::SpawnPlayer(hwnd)).unwrap();
}
