# Audio FFT Web Application

This project is a web application that visualizes the frequency spectrum of an audio file in real-time. It uses Bun as the JavaScript runtime, Vite as the build tool, Vue 3 with Composition API for the frontend framework, and Rust with WebAssembly for the FFT computation.

## Folder Structure

```plaintext
audio-fft-web/
├── .vscode/
├── public/
│   └── vite.svg
├── src/
│   ├── assets/
│   │   └── vue.svg
│   ├── components/
│   │   └── AudioVisualizer.vue
│   ├── rust_wasm/
│   │   ├── src/
│   │   │   └── lib.rs
│   │   └── Cargo.toml
│   ├── App.vue
│   ├── main.js
│   ├── style.css
├── .gitignore
├── biome.json
├── bun.lockb
├── index.html
├── package.json
├── README.md
└── vite.config.js
```

## Prerequisites
- Bun
- Rust
- wasm-pack

## Getting Started
1. Clone the repository
```sh
git clone https://github.com/yourusername/audio-fft-web.git
cd audio-fft-web
```
2. Install dependencies
```sh
bun install
```
3. Build the Rust and WebAssembly module
Navigate to the rust_wasm directory and build the Rust code with wasm-pack.
```sh
cd src/rust_wasm
wasm-pack build --target web
```

4. Run the development server
Navigate back to the project root and start the development server.
```sh
cd ../../
bun run dev
```

5. Build for production
To create a production build, use the following command:
```sh
bun run build
This will generate the production files in the dist directory.
```

## Project Details
# Bun
Bun is used as the JavaScript runtime for this project. It is faster and more efficient than Node.js in many scenarios.

# Vite
Vite is used as the build tool. It provides fast development server start times and optimized production builds.

# Vue 3
Vue 3 with the Composition API is used for building the user interface. The AudioVisualizer.vue component handles the audio visualization logic.

# Rust and WebAssembly
Rust is used for the performance-critical parts of the application, specifically the FFT computation. The Rust code is compiled to WebAssembly using wasm-pack, and the resulting WebAssembly module is used in the Vue component.

# Contributing
Feel free to submit issues and pull requests. Contributions are always welcome.

# License
This project is licensed under the MIT License.
