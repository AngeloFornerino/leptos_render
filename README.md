# Leptos Render

Leverage the power of web graphics rendering with Leptos. The Leptos Render project provides bindings to integrate popular web rendering packages with the Leptos framework. Each framework binding is available as a distinct feature, allowing developers to mix and match according to their requirements. By default, no feature is enabled, ensuring a lightweight footprint.

## Table of Contents

- [Features](#features)
- [Getting Started](#getting-started)
    - [Prerequisites](#prerequisites)
    - [Installation](#installation)
- [Usage](#usage)
- [Contributing](#contributing)
- [License](#license)
- [Acknowledgments](#acknowledgments)

## Features

- `three`: Bindings for integrating the [Three.js](https://threejs.org/) library with Leptos.

## Getting Started

### Prerequisites

- A project set up with the [Leptos framework](https://leptos.dev/).

### Installation

1. Add the following to your `Cargo.toml` under `[dependencies]`:

   ```toml
   leptos_render = { version = "x.y.z", features = ["three"] }
   ```

   Replace "x.y.z" with the desired version of `leptos_render`.

2. Ensure the following scripts are added to the `index.html` of your Leptos project:

   ```html
   <script async src="https://unpkg.com/es-module-shims@1.8.0/dist/es-module-shims.js"></script>
   <script type="importmap">
     {
       "imports": {
         "three": "https://unpkg.com/three@0.156.1/build/three.module.js",
         "three/addons/": "https://unpkg.com/three@0.156.1/examples/jsm/"
       }
     }
   </script>
   <script type="module">
     import * as THREE from 'three';
     import * as THREE_GLTF from 'three/addons/loaders/GLTFLoader.js';
     window.THREE = THREE;
     window.THREE_GLTF = THREE_GLTF;
   </script>
   ```

## Usage

After setting up the prerequisites and installation steps, you can seamlessly use the Three.js library within your Leptos project. More detailed documentation for each feature will be provided as the project evolves.

## Contributing

Contributions, suggestions, and bug reports are always welcome. As we plan to expand to more rendering frameworks, your input will be invaluable.

## License

_TODO: Specify the license or provide a link to the license file._

## Acknowledgments

- Thanks to the [Three.js](https://threejs.org/) team for their outstanding work in web graphics rendering.
