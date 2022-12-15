global_hotkey lets you register Global HotKeys for Desktop Applications.

## Example

```rs
use global_hotkey::{GlobalHotKeyManager, hotkey::{HotKey, Modifiers, Code}};

// initialize the hotkeys manager
let manager = GlobalHotKeyManager::new().unwrap();

// construct the hotkey
let hotkey = HotKey::new(Some(Modifiers::SHIFT), Code::KeyD);

// register it
manager.register(hotkey);
```


## Processing global hotkey events

You can use `global_hotkey_event_receiver` to get a reference to the `GlobalHotKeyEventReceiver`
which you can use to listen to the hotkey pressed events.
```rs
use global_hotkey::global_hotkey_event_receiver;

if let Ok(event) = global_hotkey_event_receiver().try_recv() {
    println!("{:?}", event);
}
```

## Platforms-supported:

- Windows
- macOS
- Linux (X11 Only)

## License

Apache-2.0/MIT
