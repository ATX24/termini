# termini

Tile every open Terminal window into a grid on screen; maximize back.

```
termini out   # grid-tile every Terminal window (2 windows = split, 4 = 2x2, ...)
termini in    # maximize the current window to fill the screen
```

Works on OS-level Terminal windows. If you have 5 tabs in one window, that's
one window and gets one tile — drag tabs out of the tab bar first if you want
them tiled individually.

## Install

```
cargo install --path .
```

Requires Accessibility permission — System Settings -> Privacy & Security ->
Accessibility -> enable your terminal.
