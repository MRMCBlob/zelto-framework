# Zelto

> Native Windows apps in TSX — no Chromium, no Electron, no bloat.

Zelto is an open-source framework for building Windows desktop applications using a React/TypeScript-inspired syntax. Apps compile to a single native `.exe` using Win32 + Direct2D — no browser runtime required.

## Why Zelto?

| | Electron | Tauri | **Zelto** |
|---|---|---|---|
| Runtime | Chromium (~120 MB) | WebView2 (~10 MB) | **Win32 + D2D (~1 MB)** |
| Language | JS/TS | JS/TS | **TSX (compiled)** |
| Rendering | Web | Web | **Native + D2D** |
| Windows-only | No | No | **Yes (by design)** |

## Status

**Phase 1 — In Progress**: Rust API + Win32 runtime + layout engine.

| Phase | Status | Description |
|---|---|---|
| 1 | 🟡 In Progress | Rust builder API, Win32 runtime, Flexbox layout |
| 2 | ⬜ Planned | TSX compiler (`swc_core`), `zelto build` |
| 3 | ⬜ Planned | Hot reload, Direct2D canvas, `zelto dev` |
| 4 | ⬜ Planned | Extended std-lib, theming |
| 5 | ⬜ Planned | Docs, VS Code extension, community |

## Quick Start (Phase 1 — Rust API)

```bash
# Requires Rust + MSVC toolchain on Windows
git clone https://github.com/zelto-framework/zelto
cd zelto
cargo run -p counter
```

## Phase 2 Preview (TSX syntax — coming soon)

```tsx
// src/main.zelto
import { useState } from "zelto";

function Counter() {
  const [count, setCount] = useState(0);

  return (
    <Window title="My App" width={400} height={300}>
      <View style={{ flexDirection: "column", padding: 20, gap: 12 }}>
        <Text style={{ fontSize: 32 }}>{count}</Text>
        <Button onClick={() => setCount(count + 1)}>+1</Button>
        <Button onClick={() => setCount(0)}>Reset</Button>
      </View>
    </Window>
  );
}
```

```bash
zelto build    # → my-app.exe  (~1-3 MB)
```

## Architecture

```
.zelto (TSX) ──► Zelto Compiler ──► Zelto IR ──► Runtime (dev/hot-reload)
                                         └──────► Rust Codegen ──► rustc ──► .exe
```

**Crates:**
- `zelto-ir` — Intermediate Representation (ZeltoNode tree)
- `zelto-win32` — Safe Win32 wrappers (built on `windows-rs`)
- `zelto-runtime` — Event loop, reconciler, hook system
- `zelto-d2d` — Direct2D custom canvas renderer
- `zelto-compiler` — TSX parser + Rust codegen (Phase 2)
- `zelto-std` — Standard component library
- `zelto-cli` — `zelto new/dev/build` CLI

## Contributing

PRs and issues welcome! See [CONTRIBUTING.md](CONTRIBUTING.md).

Licensed under [MIT](LICENSE).
