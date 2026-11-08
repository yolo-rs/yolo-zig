# yolo-zig — High-Performance Zig C-ABI Accelerators

Noyaux d'acceleration bare-metal, primitives SIMD et interoperabilite zero-copie multi-langages.

## Architecture
- `carbon_ffi` : Noyaux partagés de calcul tensoriel et distance cosinus.
- Primitives SIMD specialisees pour AVX-512, AVX2, ARM NEON.
- C-ABI transparent pour Bun FFI, Rust FFI, Python ctypes et Go cgo.

## Compilation & Tests
```bash
zig build test
zig build -Doptimize=ReleaseFast
```
