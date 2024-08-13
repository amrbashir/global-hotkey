---
"global-hotkey": patch
---

Fix a panic when parsing `HotKey` from a string and return an error instead, if the hotkey string consists of only modifiers and doesn't contain a key.