# SFROM Format (SNES Nintendo Switch Online)

Format information for the `.sfrom` files used on the SNES Nintendo Switch Online app.
All values are little endian unless stated otherwise.

## Footer

Found at the end of the ROM file:

| Start | Size | Description                    | Notes                                                             |
|-------|------|--------------------------------|-------------------------------------------------------------------|
| 0x00  | n    | Game Info Data                 | This is explained later.                                          |
| n     | 0x4  | Game Info Data Size            |                                                                   |
| n+0x4 | 0x4  | Magic Number "Can1"            |                                                                   |

### Game Tag Data
| Hex  | Tag  | Size | Value | Description | Notes |
|------|------|------|-------|-------------|-------|
| 0x41 | A    | 0x3? | 00 00 00 | Armet Threshold? | |
| 0x44 | D    | 0x4  | XX XX XX <Data> | S-DD1 Data | XX XX XX = Size of S-DD1 Data (Little Endian) |
| 0x47 | G    | 0x5  | 02 00 00 PP PP | Preset ID | PP PP = Preset ID (Little Endian)
| 0x50 | P    | 0x7  | 04 00 00 1E 00 00 00 | Flags? | Seen in Joe & Mac 2 |
| 0x53 | S    | 0x3? | 00 00 00 | ??? | |
| 0x55 | U    | 0x5? | 02 00 00 XX XX | SuperFX Clock Speed? | |
| 0x61 | a    | 0x1  | XX | Armet Version? | |
| 0x63 | c    | 0x1  | XX | SNES Header Location? | |
| 0x64 | d    | 0x1  | XX | ??? | (Added in 2.4.0) |
| 0x65 | e    | 0x1  | XX | Enhancement Chip | See Enhancement Chip list values below. |
| 0x68 | h    | 0x1  | XX | Resolution Ratio | 0 = 256x224 (1:1), 1 = 512x224 (2:1), 2 = 256x448 (1:2), 3 = 512x448 (2:2) |
| 0x6A | j    | 0x1  | XX | ??? | Seen in Super Mario Kart & Brawl Brothers with value 0x38.
| 0x6D | m    | 0x1  | XX | Mouse Flag | Enables mouse if not 0x00 |
| 0x70 | p    | 0x1  | XX | Max amount of players | Seen with values 1, 2, and 4 (Super Puyo Puyo 2) |
| 0x72 | r    | 0x1  | XX | Visible Height | Amount of visible vertical pixels, it stretches vertically. (0x00 = 224 by default, 0xEF = 239 for overscan) |
| 0x74 | t    | 0x1  | XX | ??? | Only seen with value 0x06 and 0x07 (Super Tennis) |
| 0x76 | v    | 0x1  | XX | Volume? | Always different between games |


Format documentation from DarkAkuma and other SFROM SNES Classic research.

# SFROM Format (SNES Mini / Classic)

Format information for the `.sfrom` files used on the SNES Mini / SNES Classic.

## Header
| Start | Size | Description                    | Notes                                                             |
|-------|------|--------------------------------|-------------------------------------------------------------------|
| 0x00  | 0x4  | 0x00010000                     |                                                                   |
| 0x04  | 0x4  | File size                      | Includes the header and footer                                    |
| 0x08  | 0x4  | Location of ROM                | Always 0x00000030 in official files                               |
| 0x0C  | 0x4  | Location of PCM samples        | Comes after end of ROM in official files                          |
| 0x10  | 0x4  | Location of PCM footer         | Equals file size if PCM data missing                              |
| 0x14  | 0x4  | Location of footer (below)     | Same as [New 3DS's SNES VC header](https://3dbrew.org/wiki/3DS_Virtual_Console#data.bin_structure) starting from 0x30 |
| 0x18  | 0x4  | Location of S-DD1 data offset  | Footer location + 0x2C                                            |
| 0x1C  | 0x4  | 0x00000000                     |                                                                   |
| 0x20  | 0x4  | Unknown                        | Footer location + 0x1B. Some kind of a flag?                      |
| 0x24  | 0x8  | Wii U Virtual Console Game ID  | WUP-J**X, X — title region (E = USA, J = Japan, P = EUR)          |
| 0x2C  | 0x4  | 0x00000000                     |                                                                   |

## Footer
| Start | Size | Description            | Notes                                                                     |
|-------|------|------------------------|---------------------------------------------------------------------------|
| 0x00  | 0x1  | Emulation speed in FPS | 0x3C == 60 FPS, 0x32 == 50 FPS                                                                |
| 0x01  | 0x4  | ROM Size               |                                                                           |
| 0x05  | 0x4  | Size of PCM samples    | 0 if missing                                                              |
| 0x09  | 0x4  | Size of PCM footer     | 0 if missing                                                              |
| 0x0D  | 0x2  | Game preset ID         | Varies between games                                                      |
| 0x0F  | 0x1  | Amount of players      | Usually 0x02. Setting to 0x0 disables the second controller               |
| 0x10  | 0x1  | Sound Volume           |                                                                           |
| 0x11  | 0x1  | ROM Type               | 0x14 (20) for LoROM and 0x15 (21) for HiROM                               |
| 0x12  | 0x1  | Enhancement Chip       | See Enhancement Chip list values below.                                   |
| 0x1B  | 0x4  | Unknown                | Usually 0x00000001, sometimes 0x00000000                                  |
| 0x1F  | 0x4  | Unknown                | Always 0x00000001                                                         |


Taken from [this gist](https://gist.github.com/anpage/c1085055db0242ea3c7558dab56712a5), which was taken from this [pastebin](https://pastebin.com/XD02kr3Z) from [@Cluster_M](https://twitter.com/Cluster_M/status/914543801858961410), cleaned up with [anpage's](https://gist.github.com/anpage) own findings added in. The pastebin was originally written by pcm720 on the [GBAtemp forums](https://gbatemp.net/threads/hakchi2-nes-mini-very-simple-pimp-tool.456256/page-577#post-7608411).

Some corrected information from DarkAkuma's documentation [here](http://darkakuma.z-net.us/2019/05/snes-classic-whats-left-whats-next.html)

# Common Info

You can find a list of Game Preset IDs used in this spreadsheet: [https://docs.google.com/spreadsheets/d/1PbIPVA4NpFEXs1zk249aR3FSuBTY3r-ajpTq3dP3GnQ](https://docs.google.com/spreadsheets/d/1PbIPVA4NpFEXs1zk249aR3FSuBTY3r-ajpTq3dP3GnQ)

## Enhancement Chips Value List

Found by Bosco82.

| Value | Usage |
|-------|-------|
| 0x00 | Normal |
| 0x02 | DSP-1 |
| 0x03 | S-DD1 |
| 0x04 | Cx4 |
| 0x05 | Mega Man X (Copy protection fix?) |
| 0x06 | SA-1 |
| 0x07 | SA-1 |
| 0x08 | SA-1 |
| 0x09 | SA-1 |
| 0x0A | SA-1 |
| 0x0B | SA-1 |
| 0x0C | SuperFX / GSU |
