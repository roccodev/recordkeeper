# Recordkeeper

![License LIB](https://img.shields.io/badge/license--lib-LGPL--3.0-orange)
![License App](https://img.shields.io/badge/license--webapp-GPL--3.0-orange)
[![Translation status](https://hosted.weblate.org/widget/recordkeeper/webapp/svg-badge.svg)](https://hosted.weblate.org/engage/recordkeeper/)

Recordkeeper is a save editor for Xenoblade Chronicles 3. It aims to support
the latest version of the game, along with the latest version of each DLC.

A web application that implements the library is hosted [here](https://rocco.dev/recordkeeper).

## Progress

Here is a list of the features supported by the save editor.

Legend:

* :white_check_mark:: fully supported
* :heavy_check_mark:: partially supported
* :wrench:: possible through low-level editing (e.g. flags)
* None: not supported/unknown

Base game features:

| Feature | Library support | Webapp support |
| ------- | --------------- | -------------- |
| Characters | :white_check_mark: | :white_check_mark: |
| Ouroboros | :white_check_mark: | :white_check_mark: |
| Inventory | :white_check_mark: | :white_check_mark: |
| Quest progress | :white_check_mark: | :white_check_mark: |
| Unique Monster records | :white_check_mark: | :white_check_mark: |
| Map locations | :white_check_mark: | :white_check_mark: |
| Map visibility (fog of war) | :white_check_mark: | |
| Save file settings | :white_check_mark: | :white_check_mark: |
| Party formations | :white_check_mark: | :white_check_mark: |
| Chronological data (sorting, etc.) | :white_check_mark: | |
| System file (`bf3system00.sav`) | :white_check_mark: | |
| Colony affinity | :wrench: | :wrench: |
| NPC affinity & Collectopedia | :wrench: | :wrench: |

DLC features:

| Feature | Library support | Webapp support |
| ------- | --------------- | -------------- |
| Inoswap | :white_check_mark: | :white_check_mark: |
| Accessory crafting | :white_check_mark: | :white_check_mark: |
| Time Attack records | :white_check_mark: | :white_check_mark: |
| Archsage's Gauntlet records | :white_check_mark: | :white_check_mark: |
| Archsage's Gauntlet save states | :white_check_mark: | :heavy_check_mark: |

Future Redeemed features:

| Feature | Library support | Webapp support |
| ------- | --------------- | -------------- |
| Affinity Growth | :white_check_mark: | :white_check_mark: (missing APs) |
| Collectopedia | | |
| Enemypedia | | |
| Map progress | | |

## Webapp translations

The webapp is translated on [Weblate](). I am currently looking for translations for [all
languages](https://switchbrew.org/wiki/Settings_services#Language) supported by the Nintendo Switch. 

I'd appreciate if you could join the effort if you have some time to spare!

[![Translation banner](https://hosted.weblate.org/widget/recordkeeper/webapp/287x66-black.png)](https://hosted.weblate.org/engage/recordkeeper/)

## License

The `recordkeeper` and `recordkeeper-macros` libraries are licensed under the
GNU Lesser General Public License v3.0. (see [COPYING-LGPL](COPYING-LGPL))

The save editor web app (https://rocco.dev/recordkeeper) is licensed under the
GNU General Public License v3.0. (see [COPYING-GPL](COPYING-GPL))
