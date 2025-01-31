# Fundamentals - 3D Audio Visualization

## Overview
Fundamentals is a real-time 3D audio visualization app built with Bevy. In time, it will be for visualizing immersive audio spaces with different types of emission devices and recievers and spaces and all that fun stuff.

## Setup
1. Install Rust ([Guide](https://www.rust-lang.org/tools/install))
2. Clone and run:
   ```sh
   git clone https://github.com/yourusername/fundamentals.git
   cd fundamentals
   cargo run
   ```

## Core Features
- **Scene Setup:** Includes a 3D camera, lighting, a sphere (audio emitter), and a ground plane.
- **Rotating Camera:** Use the up and down arrow keys to zoom. Hold right click and drag the mouse to move the camera.
- **Audio Visualization (Upcoming):** Plans to react to frequency and amplitude data.

## Code Breakdown
- **App Initialization:** Loads plugins, sets up the scene, and updates visuals.
- **Camera Rotation:** Smoothly orbits around the scene.

## Roadmap
- **Audio Processing:** Real-time frequency analysis.
- **Visual Effects:** Responsive object transformations.
- **Multiple Sources:** Support for diverse audio inputs.

## Contributing & License
Contributions welcome! Licensed under MIT.


