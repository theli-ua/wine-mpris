use std::sync::Arc;

use log::{debug, info};
// use mpris_server::Player;
use windows::{
    core::implement,
    Foundation::{self, EventRegistrationToken},
    Win32::{
        Foundation::HWND,
        UI::WindowsAndMessaging::{
            GetWindowTextLengthA, GetWindowTextLengthW, GetWindowTextW, GetWindowThreadProcessId,
        },
    },
};
use windows_core::HRESULT;

use crate::bindings::Media::*;
#[implement(SystemMediaTransportControls)]
pub struct MediaControls {
    appwindow: HWND,
    display_updater: SystemMediaTransportControlsDisplayUpdater, // rt_handle: tokio::runtime::Handle,
                                                                 // notify: tokio::sync::oneshot::Sender<()>,
                                                                 // player: Arc<Player>,
}

impl MediaControls {
    pub fn new(appwindow: HWND) -> Self {
        // let rt = tokio::runtime::Builder::new_current_thread()
        //     .enable_all()
        //     .build()
        //     .unwrap();
        // let (tx, rx) = tokio::sync::oneshot::channel();
        // let handle = rt.handle().clone();
        // std::thread::spawn(move || rt.block_on(rx));
        // let player = Arc::new(handle
        //     .block_on(Player::builder("com.wine.mpris").build())
        //     .unwrap());
        // let player2 = Arc::clone(&player);
        // handle.spawn(player.run());
        let display_updater = DisplayUpdater {
            music: MusicDisplayPropertiesImpl {}.into(),
        }
        .into();
        Self {
            appwindow,
            display_updater,
            // notify: tx,
            // player,
            // rt_handle: handle,
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
        Ok(())
    }

    fn IsPlayEnabled(&self) -> windows_core::Result<bool> {
        todo!()
    }

    fn SetIsPlayEnabled(&self, value: bool) -> windows_core::Result<()> {
        Ok(())
    }

    fn IsStopEnabled(&self) -> windows_core::Result<bool> {
        todo!()
    }

    fn SetIsStopEnabled(&self, value: bool) -> windows_core::Result<()> {
        todo!()
    }

    fn IsPauseEnabled(&self) -> windows_core::Result<bool> {
        todo!()
    }

    fn SetIsPauseEnabled(&self, value: bool) -> windows_core::Result<()> {
        Ok(())
    }

    fn IsRecordEnabled(&self) -> windows_core::Result<bool> {
        todo!()
    }

    fn SetIsRecordEnabled(&self, value: bool) -> windows_core::Result<()> {
        todo!()
    }

    fn IsFastForwardEnabled(&self) -> windows_core::Result<bool> {
        todo!()
    }

    fn SetIsFastForwardEnabled(&self, value: bool) -> windows_core::Result<()> {
        todo!()
    }

    fn IsRewindEnabled(&self) -> windows_core::Result<bool> {
        todo!()
    }

    fn SetIsRewindEnabled(&self, value: bool) -> windows_core::Result<()> {
        Ok(())
    }

    fn IsPreviousEnabled(&self) -> windows_core::Result<bool> {
        todo!()
    }

    fn SetIsPreviousEnabled(&self, value: bool) -> windows_core::Result<()> {
        Ok(())
    }

    fn IsNextEnabled(&self) -> windows_core::Result<bool> {
        todo!()
    }

    fn SetIsNextEnabled(&self, value: bool) -> windows_core::Result<()> {
        Ok(())
    }

    fn IsChannelUpEnabled(&self) -> windows_core::Result<bool> {
        todo!()
    }

    fn SetIsChannelUpEnabled(&self, value: bool) -> windows_core::Result<()> {
        todo!()
    }

    fn IsChannelDownEnabled(&self) -> windows_core::Result<bool> {
        todo!()
    }

    fn SetIsChannelDownEnabled(&self, value: bool) -> windows_core::Result<()> {
        todo!()
    }

    fn ButtonPressed(
        &self,
        handler: Option<
            &Foundation::TypedEventHandler<
                SystemMediaTransportControls,
                windows::Media::SystemMediaTransportControlsButtonPressedEventArgs,
            >,
        >,
    ) -> windows_core::Result<Foundation::EventRegistrationToken> {
        Ok(EventRegistrationToken { Value: 1 })
    }

    fn RemoveButtonPressed(
        &self,
        token: &Foundation::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        todo!()
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
        todo!()
    }

    fn RemovePropertyChanged(
        &self,
        token: &Foundation::EventRegistrationToken,
    ) -> windows_core::Result<()> {
        todo!()
    }
}

#[implement(MusicDisplayProperties)]
pub struct MusicDisplayPropertiesImpl {}

impl IMusicDisplayProperties_Impl for MusicDisplayPropertiesImpl {
    fn Title(&self) -> windows_core::Result<windows_core::HSTRING> {
        todo!()
    }

    fn SetTitle(&self, value: &windows_core::HSTRING) -> windows_core::Result<()> {
        debug!("Title: {value:?}");
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
        debug!("Artist: {value:?}");
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
