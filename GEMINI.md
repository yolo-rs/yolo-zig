# GEMINI.md — yolo-zig

## 0. Directives Fondatrices
- **NHITL (No Human In The Loop)** : autonomie bare-metal sans interruption.
- **Zero Emoji** : conformite absolue au protocole global YOLO.
- **C-ABI Zero-Copie** : noyaux natifs interoperables sans overhead entre Rust, Bun (`bun:ffi`), Python (`ctypes`) et Go.

## 1. Role et Architecture
`yolo-zig` fournit les couches d'acceleration et les moteurs bas-niveau en Zig :
- **SIMD Accelerators** : AVX-512, AVX2, NEON pour dot product, distance cosinus RAG et normalisation RMSNorm.
- **Electrobun Bare-Metal Forks** : composants systemes legers pour le remplacement d'infrastructures lourdes.
- **Vercel-FX** : moteur d'effets et calculs graphiques haute performance en C-ABI.

## 2. Commandes Canoniques
- `zig build test` : Executer la suite de tests unitaires Zig.
- `zig build -Doptimize=ReleaseFast` : Compiler les bibliotheques partagées C-ABI (`.dll`, `.so`, `.dylib`).
