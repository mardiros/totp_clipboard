# totp-clipboard

`totp-clipboard` is a system tray application to add totp code in the clipboard in two mouse click.


`totp-clipboard` is a port of [TOTP_Clipboard Gnome 3 Extensions](https://github.com/mardiros/TOTP_Clipboard-guillaume.gauvr.it).

I am currently using it on `i3wm`.


Like [TOTP_Clipboard Gnome 3 Extensions](https://github.com/mardiros/TOTP_Clipboard-guillaume.gauvr.it),
deeds are store in a json file in `~/.config/totp-seeds.json`.

Currently `totp-clipboard` does not support seeds management (CRUD operation).


You can edit the file manually to add seeds by writing this file.

The file permission shoud be `-rw-------`.

The file format is `{"seed label": "BASE32 ENCODED SEED", "seed label2": "ANOTHER BASE32 ENCODED SEED"}`.

## Installation (Archlinux)

```
sudo pacman -S gtk3 adwaita-icon-theme libappindicator-gtk3 clang
cargo install totp_clipboard
```
0