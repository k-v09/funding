# Fundamentals - 3D Audio Visualization

## Overview
Fundamentals is a real-time 3D audio visualization app built using [Bevy](https://bevyengine.org/), a Rust game engine. The goal of the project is to create an immersive environment for visualizing spatialized audio, where different types of emitters and receivers can interact dynamically. This serves as a foundation for experimenting with real-time frequency analysis, object transformations, and simulated acoustics in 3D space.

## Setup
### Prerequisites
- Ensure you have Rust installed.
- Ensure you're prepared to have your mind absolutely blown
- Please be sure you're using bevy 0.13+ I learned that the hard way

### Installation & Running the App
```sh
git clone https://github.com/k-v09/funding.git
cd fundamentals
cargo run
```

## How It Works
### Core Mechanics
- **Scene Setup**: The simulation starts with a 3D scene containing a ground plane, a movable camera, a light source, and multiple `AudioEmitter` objects.
- **Audio Emitters**: Each emitter represents an audio source with a specific frequency and amplitude.
- **Wave Simulation**: The size of each emitter oscillates based on a sine wave, determined by its frequency and phase.
- **Camera Controls**: The user can zoom in and out, as well as rotate around the scene to explore the visualization dynamically.

## Controls
| Action         | Key/Mouse Input  |
|---------------|----------------|
| Orbit Camera | Hold **Right Click** + Drag Mouse |
| Zoom In | **Arrow Up** |
| Zoom Out | **Arrow Down** |

## Technical Details
### System Breakdown
#### 1. App Initialization
The Bevy `App` is configured with:
- **Default Plugins**: Enables core Bevy functionalities like rendering, windowing, and input handling.
- **Resources**:
  - `SimulationTime`: Tracks elapsed time for wave calculations.
  - `CameraController`: Manages camera movement and zoom sensitivity.
- **Systems**:
  - `setup`: Creates the initial scene and objects.
  - `update_sim`: Updates emitter scales based on waveforms.
  - `camera_controller`: Handles user camera movement.

#### 2. Audio Visualization
Each `AudioEmitter` has:
- **Frequency** (in Hz) determining wave oscillations.
- **Amplitude**, affecting the intensity of scaling.
- **Phase**, introducing offsets between emitters.

The simulation updates emitter sizes using:
```rust
let wave = ((emitter.frequency * sim_time.elapsed * std::f32::consts::TAU) + emitter.phase).sin();
let scale = 1.0 + wave * emitter.amplitude;
transform.scale = Vec3::splat(scale);
```
This creates a pulsing effect, simulating an audio wave's impact on a visual object.

#### 3. Camera Controls
The camera orbits the scene using:
- **Mouse Motion** for rotation.
- **Arrow Keys** for zooming in and out.
- **Clamping** to prevent extreme angles or excessive zoom distances.

## Roadmap
### Upcoming Features
- [x] **Basic 3D Scene** (Implemented)  
- [ ] **Audio Processing Integration** - Real-time frequency analysis using FFT (may or may not happen depending on what direction the project takes). 
- [ ] **Dynamic Object Transformations** - Emitters could deform or change color in response to audio.  
- [ ] **Multiple Audio Sources** - Support for diverse inputs beyond predefined frequencies.  
- [ ] **User Interaction Improvements** - Customizable controls and UI enhancements with time controls and "world building" tools

## Contributing
Contributions are welcome! Feel free to submit issues or pull requests.

## License
This project is licensed under the MIT License.


