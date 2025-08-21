# Modern Visual Reaction Test

A sophisticated 3D reaction and aiming test application built with Bevy game engine. Test your reaction time, tracking accuracy, and aim precision with customizable settings and professional analytics.

![Modern Reaction Test](https://img.shields.io/badge/Engine-Bevy-blue)
![Rust](https://img.shields.io/badge/Language-Rust-orange)

## Features

- **Precision Aim Testing**: Measure reaction time, angular error, and tracking accuracy
- **Customizable Settings**: Adjust DPI, sensitivity, FOV, colors, and lighting
- **Professional Analytics**: Detailed performance metrics and rating system
- **Modern UI**: Clean, intuitive interface with color pickers and real-time feedback
- **3D Environment**: Immersive arena with physics-based target movement
- **Fresnel Effects**: Advanced visual effects for target highlighting
- **Config Persistence**: Automatic saving of all settings and preferences

## Installation

### Prerequisites

1. **Install Rust**: [rustup.rs](https://rustup.rs/)
2. **Install Build Tools**:
   - Windows: Visual Studio Build Tools with C++ support
   - Linux: `build-essential`, `libasound2-dev`, `libx11-dev`
   - macOS: Xcode Command Line Tools

### Building from Source

Clone the repository
```bash
git clone https://github.com/papersaccul/mVRT
cd mVRT
```
Build in release mode (recommended)
```
cargo build --release
```
*The executable will be in `target/release/`*


### Pre-built Binaries

Download the latest release from the [Releases page](https://github.com/papersaccul/mVRT/releases) and extract the archive.

## Configuration

The application automatically creates a configuration file at `assets/config.json` with default settings. You can modify this file directly or use the in-game settings menu.

### Key Configuration Options

- **DPI**: Your mouse DPI setting
- **cm/360**: Centimeters per 360-degree rotation
- **FOV**: Field of view in degrees
- **Colors**: Customize crosshair, arena, and target colors
- **Lighting**: Adjust directional and ambient light intensity
- **Keybinds**: Customizable control scheme

## Usage

### Getting Started

1. Launch the application
2. Configure your mouse settings in the Settings menu
3. Press `SPACE` to start the test
4. Track and shoot the moving target
5. Review your results after the 20-second test duration

### Controls
Default keybinds:
| Action | Default Key |
|--------|-------------|
| Start Test | SPACE |
| Restart Test | R |
| Open Settings | ESC |
| Toggle Fullscreen | F12 |
| Shoot | Auto |

### Understanding Results

- **Avg Reaction**: Average reaction time in milliseconds
- **Accuracy**: Hit/miss percentage
- **Angular Error**: Average deviation from target in degrees
- **Rating**: Performance ranking (Bronze to Supreme)

## Development

### Project Structure

```
src/
├── main.rs          # Application entry point
├── config/          # Configuration management
├── kernel/          # Core game systems
├── rendering/       # Graphics and materials
├── state/           # Application state management
└── user_interface/  # UI components and systems
```

### Building for Development

```bash
# Debug build with symbols
cargo build

# Run with debug output
cargo run

# Run with optimizations
cargo run --release
```

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- Built with the [Bevy game engine](https://bevyengine.org/)
- Uses Rust for high performance and safety
- Inspired by professional aim training tools

## Support

For support and questions:
- 📖 Check the documentation above
- 🐛 [Open an issue](https://github.com/papersaccul/mVRT/issues)
- 🗨️ You can contact the author in Discord: @sccl
- 💬 Join [CisA Discord community](https://discord.gg/rWH3BJAcED)
