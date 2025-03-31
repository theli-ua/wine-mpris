use std::{
    collections::HashMap,
    sync::{atomic::AtomicI64, Arc, Mutex},
};

use crate::{bindings::Media::*, mpris::send_command};
use log::debug;
use windows::{
    core::implement,
    Foundation::{self, EventRegistrationToken},
    Media::SystemMediaTransportControlsButton,
    Win32::Foundation::HWND,
};

pub struct EventHandlers {
    event_token: AtomicI64,
    event_handlers: Mutex<
        HashMap<
            i64,
            Foundation::TypedEventHandler<
                SystemMediaTransportControls,
                SystemMediaTransportControlsButtonPressedEventArgs,
            >,
        >,
    >,
    caller: Option<SystemMediaTransportControls>,
}
unsafe impl Send for EventHandlers {}
unsafe impl Sync for EventHandlers {}

#[implement(SystemMediaTransportControlsButtonPressedEventArgs)]
struct ButtonEventArgs(SystemMediaTransportControlsButton);

impl ISystemMediaTransportControlsButtonPressedEventArgs_Impl for ButtonEventArgs {
    fn Button(&self) -> windows_core::Result<SystemMediaTransportControlsButton> {
        Ok(self.0)
    }
}

impl ButtonEventArgs_Impl {}

impl EventHandlers {
    fn add_handler(
        &self,
        handler: Option<
            &Foundation::TypedEventHandler<
                SystemMediaTransportControls,
                SystemMediaTransportControlsButtonPressedEventArgs,
            >,
        >,
    ) -> windows_core::Result<Foundation::EventRegistrationToken> {
        let Value = self
            .event_token
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        self.event_handlers
            .lock()
            .unwrap()
            .insert(Value, handler.unwrap().clone());

        Ok(EventRegistrationToken { Value })
    }

    fn remove_handler(&self, key: i64) {
        self.event_handlers.lock().unwrap().remove(&key);
    }

    pub fn invoke(&self, button: SystemMediaTransportControlsButton) {
        let system_media_transport_controls_button_pressed_event_args =
            SystemMediaTransportControlsButtonPressedEventArgs::from(ButtonEventArgs(button));
        for handler in self.event_handlers.lock().unwrap().values() {
            handler
                .Invoke(
                    self.caller.as_ref(),
                    &system_media_transport_controls_button_pressed_event_args,
                )
                .unwrap();
        }
    }
}

#[implement(SystemMediaTransportControls)]
pub struct MediaControls {
    appwindow: HWND,
    display_updater: SystemMediaTransportControlsDisplayUpdater,
    event_handlers: Arc<EventHandlers>,
}

impl MediaControls {
    pub fn new(appwindow: HWND) -> Self {
        let event_handlers = Arc::new(EventHandlers {
            event_token: 0.into(),
            event_handlers: Mutex::default(),
            caller: None,
        });
        crate::mpris::spawn_player(appwindow, event_handlers.clone());

        let display_updater = DisplayUpdater {
            music: MusicDisplayPropertiesImpl { hwnd: appwindow }.into(),
        }
        .into();
        Self {
            appwindow,
            display_updater,
            event_handlers,
        }
    }
}

impl ISystemMediaTransportControls_Impl for MediaControls {
    fn PlaybackStatus(&self) -> windows_core::Result<windows::Media::MediaPlaybackStatus> {
        todo!()
    }

    fn SetPlaybackStatus(
        &self,
        value: windows::Media::MediaPlaybackStatus,
    ) -> windows_core::Result<()> {
        send_command(crate::mpris::Command::SetState(self.appwindow, value));
        debug!("{value:?}");
        Ok(())
    }

    fn DisplayUpdater(&self) -> windows_core::Result<SystemMediaTransportControlsDisplayUpdater> {
        Ok(self.display_updater.clone())
    }

    fn SoundLevel(&self) -> windows_core::Result<windows::Media::SoundLevel> {
        todo!()
    }

    fn IsEnabled(&self) -> windows_core::Result<bool> {
        todo!()
    }

    fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()> {
        let _ = value;
        Ok(())
    }

    fn IsPlayEnabled(&self) -> windows_core::Result<bool> {
        todo!()
    }

    fn SetIsPlayEnabled(&self, value: bool) -> windows_core::Result<()> {
        let _ = value;
        Ok(())
    }

    fn IsStopEnabled(&self) -> windows_core::Result<bool> {
        todo!()
    }

    fn SetIsStopEnabled(&self, value: bool) -> windows_core::Result<()> {
        let _ = value;
        todo!()
    }

    fn IsPauseEnabled(&self) -> windows_core::Result<bool> {
        todo!()
    }

    fn SetIsPauseEnabled(&self, value: bool) -> windows_core::Result<()> {
        let _ = value;
        Ok(())
    }

    fn IsRecordEnabled(&self) -> windows_core::Result<bool> {
        todo!()
    }

    fn SetIsRecordEnabled(&self, value: bool) -> windows_core::Result<()> {
        let _ = value;
        todo!()
    }

    fn IsFastForwardEnabled(&self) -> windows_core::Result<bool> {
        todo!()
    }

    fn SetIsFastForwardEnabled(&self, value: bool) -> windows_core::Result<()> {
        let _ = value;
        todo!()
    }

    fn IsRewindEnabled(&self) -> windows_core::Result<bool> {
        todo!()
    }

    fn SetIsRewindEnabled(&self, value: bool) -> windows_core::Result<()> {
        let _ = value;
        Ok(())
    }

    fn IsPreviousEnabled(&self) -> windows_core::Result<bool> {
        todo!()
    }

    fn SetIsPreviousEnabled(&self, value: bool) -> windows_core::Result<()> {
        let _ = value;
        Ok(())
    }

    fn IsNextEnabled(&self) -> windows_core::Result<bool> {
        todo!()
    }

    fn SetIsNextEnabled(&self, value: bool) -> windows_core::Result<()> {
        let _ = value;
        Ok(())
    }

    fn IsChannelUpEnabled(&self) -> windows_core::Result<bool> {
        todo!()
    }

    fn SetIsChannelUpEnabled(&self, value: bool) -> windows_core::Result<()> {
        let _ = value;
        todo!()
    }

    fn IsChannelDownEnabled(&self) -> windows_core::Result<bool> {
        todo!()
    }

    fn SetIsChannelDownEnabled(&self, value: bool) -> windows_core::Result<()> {
        let _ = value;
        todo!()
    }

    fn ButtonPressed(
        &self,
        handler: Option<
            &Foundation::TypedEventHandler<
                SystemMediaTransportControls,
                SystemMediaTransportControlsButtonPressedEventArgs,
            >,
        >,
    ) -> windows_core::Result<Foundation::EventRegistrationToken> {
        self.event_handlers.add_handler(handler)
    }

    fn RemoveButtonPressed(
        &self,
        token: &Foundation::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        self.event_handlers.remove_handler(token.Value);
        Ok(())
    }

    fn PropertyChanged(
        &self,
        handler: Option<
            &Foundation::TypedEventHandler<
                SystemMediaTransportControls,
                windows::Media::SystemMediaTransportControlsPropertyChangedEventArgs,
            >,
        >,
    ) -> windows_core::Result<Foundation::EventRegistrationToken> {
        let _ = handler;
        todo!()
    }

    fn RemovePropertyChanged(
        &self,
        token: &Foundation::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        let _ = token;
        todo!()
    }
}

#[implement(MusicDisplayProperties)]
pub struct MusicDisplayPropertiesImpl {
    hwnd: HWND,
}

impl IMusicDisplayProperties_Impl for MusicDisplayPropertiesImpl {
    fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        todo!()
    }

    fn SetTitle(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        debug!("Title: {value:?}");
        send_command(crate::mpris::Command::SetTitle(
            self.hwnd,
            value.to_string(),
        ));
        Ok(())
    }

    fn AlbumArtist(&self) -> windows_core::Result<windows_core::HSTRING> {
        todo!()
    }

    fn SetAlbumArtist(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        debug!("AlbumArtist: {value:?}");
        Ok(())
    }

    fn Artist(&self) -> windows_core::Result<windows_core::HSTRING> {
        todo!()
    }

    fn SetArtist(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        debug!("Artist: {value:?} {value} {}", value.len());
        send_command(crate::mpris::Command::SetArtist(
            self.hwnd,
            value.to_string(),
        ));
        Ok(())
    }
}

#[implement(SystemMediaTransportControlsDisplayUpdater)]
pub struct DisplayUpdater {
    music: MusicDisplayProperties,
}

impl ISystemMediaTransportControlsDisplayUpdater_Impl for DisplayUpdater {
    fn Type(&self) -> windows_core::Result<windows::Media::MediaPlaybackType> {
        todo!()
    }

    fn SetType(&self, value: windows::Media::MediaPlaybackType) -> windows_core::Result<()> {
        debug!("SetType: {value:?}");
        Ok(())
    }

    fn AppMediaId(&self) -> windows_core::Result<windows_core::HSTRING> {
        todo!()
    }

    fn SetAppMediaId(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        let _ = value;
        todo!()
    }

    fn Thumbnail(
        &self,
    ) -> windows_core::Result<windows::Storage::Streams::RandomAccessStreamReference> {
        todo!()
    }

    fn SetThumbnail(
        &self,
        value: Option<&windows::Storage::Streams::RandomAccessStreamReference>,
    ) -> windows_core::Result<()> {
        let _ = value;
        todo!()
    }

    fn MusicProperties(&self) -> windows_core::Result<MusicDisplayProperties> {
        Ok(self.music.clone())
    }

    fn VideoProperties(&self) -> windows_core::Result<windows::Media::VideoDisplayProperties> {
        todo!()
    }

    fn ImageProperties(&self) -> windows_core::Result<windows::Media::ImageDisplayProperties> {
        todo!()
    }

    fn CopyFromFileAsync(
        &self,
        r#type: windows::Media::MediaPlaybackType,
        source: Option<&windows::Storage::StorageFile>,
    ) -> windows_core::Result<windows::Foundation::IAsyncOperation<bool>> {
        let _ = source;
        let _ = r#type;
        todo!()
    }

    fn ClearAll(&self) -> windows_core::Result<()> {
        todo!()
    }

    fn Update(&self) -> windows_core::Result<()> {
        debug!("Update");
        Ok(())
    }
}
