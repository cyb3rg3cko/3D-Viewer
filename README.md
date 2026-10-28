# Update

This Repo is going to be stale, but i will leave it up in case someone wants to use the code. I struggled getting the Tauri to associate with file extensions and open the file when one was double clicked. I changed to wrapping the three.js script in Electron and Capacitor -> https://github.com/cyb3rg3cko/3D-Viewer-Electron-Capacitor

***

# 3D Viewer

Simple offline 3D model viewer for GLB, FBX, VRM, and STL files.

Built with Three.js + Tauri.

## Features
- View GLB / glTF, FBX, VRM, and STL models
- Materials and textures preserved where supported
- Drag & drop or File → Open
- Offline / self-contained
- Native app feel via Tauri (Windows, Linux, Android)

## App Icon
Place a square PNG (ideally 1024×1024) named `app-icon.png` in the project root.  
The workflow automatically generates all required icon sizes with:
```bash
npx tauri icon app-icon.png
```

## File Associations
The app registers itself for:
- `.glb` / `.gltf`
- `.fbx`
- `.vrm`
- `.stl`

After installing, double-clicking these files should open them in 3D Viewer on Windows, Linux, and Android.

## Development

### Prerequisites
- [Node.js](https://nodejs.org/) (LTS)
- [Rust](https://rustup.rs/)
- For Android: Android Studio + SDK + NDK

### Run locally
```bash
npm install
npm run tauri dev
```

### Build
```bash
# Desktop (current OS)
npm run tauri build

# Android
npm run tauri android init   # first time only
npm run tauri android build
```

## GitHub Actions
Pushes and tags automatically build Windows, Linux, and Android releases.
See `.github/workflows/build.yml`.

## License
MIT
