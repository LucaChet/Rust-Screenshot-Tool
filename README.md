# PDS Project 2023 - Screen Grabbing Utility

## Overview

This project implements a screen grabbing utility written in Rust. The utility is capable of capturing what is currently shown on a display, processing it, and making it available in various formats.

## Implemented Features

- **Platform Support:** Compatible with multiple desktop operating systems, including Windows, macOS, and Linux.
- **User Interface (UI):** Intuitive and user-friendly interface for easy navigation.
- **Selection Options:** Allows users to restrict the grabbed image to a custom area selected with a click and drag motion. The selected area may be further adjusted with subsequent interactions.
- **Hotkey Support:** Customizable hotkeys for quick screen grabbing. Users can set up their preferred shortcut keys.
- **Output Format:** Supports multiple output formats including `.png`, `.jpg`, `.gif`. Also supports copying the screen grab to the clipboard.
- **Annotation Tools:** Built-in annotation tools like shapes, arrows, text, and a color picker for highlighting or redacting parts of the screen grab.
- **Delay Timer:** Supports a delay timer function, allowing users to set up a screen grab after a specified delay.
- **Save Options:** Allows users to specify the default save location for screen grabs. Also supports automatic saving with predefined naming conventions.
- **Multi-monitor Support:** Recognizes and handles multiple monitors independently, allowing users to grab screens from any connected display.

## Usage

To use the utility, follow these steps:

1. Download and install the application on your system.
2. Launch the utility.
3. Use the UI to select desired options such as output format, annotation tools, etc.
4. Use hotkeys or UI buttons to initiate screen grabbing.
5. Customize and annotate the captured screen as needed.
6. Save or copy the screen grab to the desired location or clipboard.

## Development

If you're interested in contributing to the project or modifying the code, follow these steps to set up your development environment:

1. Clone the repository
2. Install Rust and Cargo if not already installed.
3. Navigate to the project directory
4. Build the project: `cargo build`
5. Make your changes and test them.
6. Submit a pull request with your changes.
