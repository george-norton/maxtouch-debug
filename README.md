# MaxTouch Debug

This is a work in progress tool for tuning MaxTouch sensors in QMK based trackpads (currently Peacock and Ploopy Pavonis). If you build a firmware with the debug keymap this tool can visualize the raw sensor data, and in the future will enable you to change register values and inspect the results.

## Build Instructions

You must first install the system dependencies for Tauri before building.
Please refer the the [Tauri Prerequisites](https://tauri.app/start/prerequisites/) section of the Tauri quickstart guide.

To build this project run the `npm run tauri build` command.
For development, similarly run the `npm run tauri dev` command.

