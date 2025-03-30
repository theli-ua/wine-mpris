### QuickStart
This is a "native" drop-in replacement for wine's implementation of 
Windows.Media.SystemMediaTransportControls component.
Basically this bridges windows media players to be seen as mpris-compatible players in the host system.
To use one needs a wine version with unix domain sockets.
Need to replace `windows/system32/windows.media.mediacontrol.dll` as example on 32 bit prefix and set `windows.media.mediacontrol.dll` DLLOVERRIDE to `native`.
Only been tested/used by me with Amazon Music desktop app.

### Notes

bindings were generated with 
```
riddle --in ~/Downloads/Windows.winmd --out bindings.rs --filter Windows.Media.ISystemMediaTransportControls --filter Windows.Media.SystemMediaTransportControls --config implement
```

Since the ones that currently with windows-rs crate do not include few things that are used for implementing them automagically. (maybe because they are not meant to be implemented?)


This will rely on https://gitlab.winehq.org/wine/wine/-/merge_requests/2786 for unix domain sockets support to connect to dbus socket
