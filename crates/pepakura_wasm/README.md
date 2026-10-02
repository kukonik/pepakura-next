# Pepakura WASM

WebAssembly bindings for Pepakura Next core.

## Overview

This crate provides WebAssembly (WASM) bindings for the Pepakura Next core library, enabling 3D mesh unfolding, PDO parsing, and SVG export directly in the browser.

## Features

- **Mesh unfolding** via MDS (Multidimensional Scaling) and LSCM (Least Squares Conformal Maps)
- **Sparse MDS** for large meshes (k‑nearest neighbors)
- **PDO file parsing** (Pepakura Designer format)
- **SVG export** of unfolded meshes
- **Zero‑copy serialization** between Rust and JavaScript using `serde‑wasm‑bindgen`

## Usage

### Building

Install `wasm‑pack`:

```bash
cargo install wasm-pack
```

Build for the web target:

```bash
wasm-pack build --target web
```

The output will be in the `pkg/` directory.

### JavaScript Example

```javascript
import init, { unfold_mds_wasm, version } from './pkg/pepakura_wasm.js';

async function run() {
    await init();
    console.log(`Pepakura WASM version: ${version()}`);

    const mesh = {
        vertices: [[0,0,0], [1,0,0], [1,1,0], [0,1,0]],
        faces: [[0,1,2], [0,2,3]],
        name: "Square"
    };

    const result = await unfold_mds_wasm(mesh, 100, 1e-6);
    console.log('Unfolded vertices:', result.vertices_2d);
}
```

### API Reference

#### `unfold_mds_wasm(mesh, maxIterations?, tolerance?)`

Unfolds a mesh using parallel MDS.

- `mesh`: `JsMesh` object with `vertices`, `faces`, `name`
- `maxIterations`: optional number (default 100)
- `tolerance`: optional number (default 1e‑6)
- Returns: `JsUnfoldResult`

#### `unfold_lscm_wasm(mesh)`

Unfolds a mesh using LSCM (angle‑preserving).

#### `unfold_mds_sparse_wasm(mesh, k?, maxIterations?, tolerance?)`

Unfolds large meshes using sparse MDS with k‑nearest neighbors.

#### `parse_pdo_wasm(uint8Array)`

Parses a PDO file (binary data) and returns a `ParsePdoResult`.

#### `export_svg_wasm(unfoldedResult, config?)`

Exports an unfolded mesh to an SVG string (basic implementation).

#### `version()`

Returns the crate version as a string.

#### `test_add(a, b)`

Simple test function that returns `a + b`.

## Data Structures

### `JsMesh`

```typescript
interface JsMesh {
    vertices: Array<[number, number, number]>;
    faces: Array<[number, number, number]>;
    name: string;
}
```

### `JsUnfoldResult`

```typescript
interface JsUnfoldResult {
    vertices_2d: Array<[number, number]>;
    faces: Array<[number, number, number]>;
    algorithm: string;
    unfold_time_ms: number;
}
```

## Development

### Testing

Run unit tests:

```bash
cargo test
```

Run WASM‑specific tests (requires `wasm‑bindgen‑test`):

```bash
wasm-pack test --node
```

### Example Page

An interactive example is provided in `examples/index.html`. After building with `wasm‑pack`, serve the directory with any HTTP server and open the HTML file.

## License

MIT