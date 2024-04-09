
bindings were generated with 
```
riddle --in ~/Downloads/Windows.winmd --out bindings.rs --filter Windows.Media.ISystemMediaTransportControls --filter Windows.Media.SystemMediaTransportControls --config implement
```

Since the ones that currently with windows-rs crate do not include few things that are used for implementing them automagically. (maybe because they are not meant to be implemented?)
