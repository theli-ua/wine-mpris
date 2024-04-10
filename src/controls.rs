use log::{info, warn};
use windows::{core::implement, Foundation, Win32::Foundation::HWND};

use crate::bindings::Media::*;
#[implement(SystemMediaTransportControls)]
pub struct MediaControls(pub HWND);

impl ISystemMediaTransportControls_Impl for MediaControls {
    fn PlaybackStatus(&self) -> windows_core::Result<windows::Media::MediaPlaybackStatus> {
        todo!()
    }

    fn SetPlaybackStatus(
        &self,
        value: windows::Media::MediaPlaybackStatus,
    ) -> windows_core::Result<()> {
        todo!()
    }

    fn DisplayUpdater(
        &self,
    ) -> windows_core::Result<windows::Media::SystemMediaTransportControlsDisplayUpdater> {
        todo!()
    }

    fn SoundLevel(&self) -> windows_core::Result<windows::Media::SoundLevel> {
        todo!()
    }

    fn IsEnabled(&self) -> windows_core::Result<bool> {
        todo!()
    }

    fn SetIsEnabled(&self, value: bool) -> windows_core::Result<()> {
        todo!()
    }

    fn IsPlayEnabled(&self) -> windows_core::Result<bool> {
        todo!()
    }

    fn SetIsPlayEnabled(&self, value: bool) -> windows_core::Result<()> {
        todo!()
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
        todo!()
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
        todo!()
    }

    fn IsPreviousEnabled(&self) -> windows_core::Result<bool> {
        todo!()
    }

    fn SetIsPreviousEnabled(&self, value: bool) -> windows_core::Result<()> {
        todo!()
    }

    fn IsNextEnabled(&self) -> windows_core::Result<bool> {
        todo!()
    }

    fn SetIsNextEnabled(&self, value: bool) -> windows_core::Result<()> {
        todo!()
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
        todo!()
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
