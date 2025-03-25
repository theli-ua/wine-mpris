
bindings were generated with 
```
riddle --in ~/Downloads/Windows.winmd --out bindings.rs --filter Windows.Media.ISystemMediaTransportControls --filter Windows.Media.SystemMediaTransportControls --config implement
```

Since the ones that currently with windows-rs crate do not include few things that are used for implementing them automagically. (maybe because they are not meant to be implemented?)


### Notes
This will rely on https://gitlab.winehq.org/wine/wine/-/merge_requests/2786 for unix domain sockets support to connect to dbus socket
