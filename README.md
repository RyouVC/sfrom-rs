# sfrom

`sfrom` is a crate for parsing the SFROM format, a file format used by Nintendo's
official SNES emulator. It provides a simple way to read and write SFROM files.

Since NERD's SNES emulator is not as accurate as other community emulators,
SFROM files contain not just the ROM data, but also additional emulator
configuration data to enable certain hacks that improve compatibility with
certain games.
