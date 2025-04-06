```sh
npm install
npm run tauri android init
npm run tauri ios init
```

Generate icons

```sh
cargo tauri icon src-tauri/icons/raw.png
```

For Desktop development, run:

```sh
npm run tauri dev
```

For Android development, run:

```sh
npm run tauri android dev
```

For iOS development, run:

```sh
npm run tauri ios dev
```

## TODO

- different pitch detections
  - https://github.com/jkjaer/fastF0Nls
  - https://www.mdpi.com/2076-3417/13/14/8191
  - https://github.com/JorenSix/TarsosDSP/tree/master/core/src/main/java/be/tarsos/dsp/pitch
