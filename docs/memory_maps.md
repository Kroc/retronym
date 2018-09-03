Memory Maps
================================================================================

SEGA Master System
--------------------------------------------------------------------------------

    $0000 +------------+- - - - ROM:
          |  ROM$0000  |  1 KB
    $0400 +------------+
          |            |
          |   SLOT 0   |  15 KB
          |            |
    $4000 +------------+
          |            |
          |   SLOT 1   |  16 KB
          |            |
    $8000 +------------+
          |            |
          |   SLOT 2   |  16 KB
          |            |
    $C000 +------------+- - - - RAM:
          |    RAM     |  8 KB
    $E000 |------------|
          | RAM MIRROR |  8 KB - 16 B
    $FFF0 +------------+
          |   MAPPER   |  16 B
    $FFFF +------------+

GameBoy / GameBoy Color
--------------------------------------------------------------------------------

    $0000 +------------+- - - - ROM:
          |            |            The first 16 KB is always mapped
          |  ROM$0000  |  16 KB     to the first 16 KB of ROM
          |            |
    $4000 +------------+
          |            |            This slot can be mapped to
          |    SLOT    |  16 KB     ROM banks 1 up to 127
          |            |
    $8000 +------------+- - - - VRAM:
          |  CHAR RAM  |  6 KB
    $9800 |------------|            This 8 KB section consists of graphic tiles
          |  BG MAP 1  |  1 KB      and the layout of background layers 1 & 2.
    $9C00 |------------|            On the Gameboy Colour this has 2 banks
          |  BG MAP 2  |  1 KB
    $A000 +------------+- - - - RAM:
          |  CART RAM  |  8 KB      On cart RAM, if present
    $C000 +------------+
          | WORK RAM 1 |  4 KB      Main RAM. On GameBoy this is 8 KB total.
    $D000 |------------|            On GameBoy Color the last 4 KB can be
          | WORK RAM 2 |  4 KB      mapped to RAM banks 1-7
    $E000 +------------+
          |//RESERVED//|  7'680 B   A mirror of RAM (sans last 512 B)
    $FE00 |------------|
          |  OAM  RAM  |  160 B     Sprite layout RAM
    $FEA0 |------------|
          |///UNUSED///|  96 B
    $FF00 +------------+
          |   HW  IO   |  128 B
    $FF80 +------------+
          |  HIGH RAM  |  128 B     Fast RAM inside the CPU
    $FFFF +------------+

Information derived from: <http://gameboy.mongenel.com/dmg/asmmemmap.html>

Amstrad CPC 464
--------------------------------------------------------------------------------

    $0000 +-----------+ RAM/ROM:
          |           |
          | RAM / ROM |     16 KB
          |           |
    $4000 +-----------+ RAM:
          |           |
          |    RAM    |     16 KB
          |           |
    $8000 +-----------+
          |           |
          |    RAM    |     16 KB
          |           |
    $C000 +-----------+ RAM/ROM:
          |           |
          | RAM / ROM |     16 KB
          |           |
    $FFFF +-----------+

Information derived from: <http://www.irespa.eu/daan/lib/howtoCPCemu.htm>

Amstrad PCW 8256 / 8512 / 9256 / 9512
--------------------------------------------------------------------------------

    $0000 +------------+ RAM:
          |            |
          |   SLOT 0   |    16 KB
          |            |
    $4000 +------------+
          |            |
          |   SLOT 1   |    16 KB
          |            |
    $8000 +------------+
          |            |
          |   SLOT 2   |    16 KB
          |            |
    $C000 +------------+
          |            |
          |   SLOT 3   |    16 KB
          |            |
    $FFFF +------------+

Information derived from: <http://www.systemed.net/pcw/hardware.html>