# UI Framework Analysis for Dadagen GUI

## Overview

Analysis of UI framework options for the dadagen cross-platform GUI application, comparing native Rust frameworks vs. hybrid approaches.

## Requirements Recap

From [requirements.md](requirements.md#requirement-2-cross-platform-gui-application):
- Native look-and-feel on Windows, macOS, and Linux
- File drag-and-drop support
- Real-time preview of generated data
- Excel, JSON, CSV parsing and display
- DSL editor with syntax highlighting
- Export functionality

## Framework Options

### Option 1: Tauri + React/TypeScript (Hybrid - Current Design)

**Pros:**
- ✅ Rich ecosystem of React components (tables, editors, file pickers)
- ✅ Mature syntax highlighting libraries (Monaco, CodeMirror)
- ✅ Excellent developer experience with hot reload
- ✅ Large talent pool for React developers
- ✅ Native system integration via Tauri API
- ✅ Small bundle size (~600KB with tree-shaking)
- ✅ Proven track record (VS Code, Discord, Figma use similar approach)
- ✅ Easy to integrate existing web libraries for CSV/Excel parsing

**Cons:**
- ⚠️ Requires learning React/TypeScript in addition to Rust
- ⚠️ More complex build pipeline (webpack/vite + cargo)
- ⚠️ Potential performance overhead for very large datasets
- ⚠️ Two-language codebase (Rust backend + JS/TS frontend)

**Bundle Size:** ~3-5 MB total (Tauri runtime + React app)

**Development Time:** Medium (familiar tools, good docs)

**Example Projects:** 
- [Tauri Examples](https://github.com/tauri-apps/tauri)
- Many production apps using Tauri v2

---

### Option 2: egui (Native Rust - Immediate Mode)

**Pros:**
- ✅ Pure Rust - no JavaScript/TypeScript required
- ✅ Very fast compile times for UI changes
- ✅ Excellent performance for real-time updates
- ✅ Built-in widgets for common UI patterns
- ✅ Cross-platform (works on web via WASM too)
- ✅ Simple mental model (immediate mode)
- ✅ Active development and community

**Cons:**
- ⚠️ Limited complex widget ecosystem vs React
- ⚠️ No built-in rich text editor with syntax highlighting
- ⚠️ Manual layout management can be tedious
- ⚠️ Less "native" look-and-feel (custom rendering)
- ⚠️ Would need to implement or integrate CSV/Excel table viewer
- ⚠️ Smaller community compared to React

**Bundle Size:** ~2-3 MB (Rust binary with embedded assets)

**Development Time:** Medium-High (need to build custom widgets)

**Example Projects:**
- [egui demo](https://www.egui.rs/)
- [rerun.io](https://www.rerun.io/) - data visualization tool

---

### Option 3: Iced (Native Rust - Elm Architecture)

**Pros:**
- ✅ Pure Rust with type-safe message passing
- ✅ Declarative UI similar to React (easier mental model)
- ✅ Good performance
- ✅ Cross-platform support
- ✅ Built-in theming support
- ✅ Async support built-in

**Cons:**
- ⚠️ Smaller ecosystem than egui or Tauri
- ⚠️ Still developing (0.x versions)
- ⚠️ Limited widget library
- ⚠️ Would need custom implementations for file viewers
- ⚠️ Syntax highlighting not built-in
- ⚠️ Less documentation compared to established frameworks

**Bundle Size:** ~3-4 MB (Rust binary)

**Development Time:** High (newer framework, less documentation)

**Example Projects:**
- [Iced examples](https://github.com/iced-rs/iced)
- Various small tools and games

---

### Option 4: Slint (Native - Declarative UI Language)

**Pros:**
- ✅ Declarative UI with .slint markup language
- ✅ Fast development with live preview
- ✅ Good documentation
- ✅ Native look-and-feel on each platform
- ✅ Built-in design tools
- ✅ Professional backing (SLINT GmbH)

**Cons:**
- ⚠️ Requires learning new .slint markup language
- ⚠️ Limited widget ecosystem
- ⚠️ Smaller community
- ⚠️ Would need custom widgets for complex views
- ⚠️ Commercial licensing for some features

**Bundle Size:** ~3-4 MB

**Development Time:** Medium-High (new language to learn)

**Example Projects:**
- [Slint examples](https://slint.dev/examples)

---

### Option 5: Dioxus (Native Rust - React-like)

**Pros:**
- ✅ React-like API but pure Rust
- ✅ Works across desktop, web, mobile, and terminal
- ✅ Hot reloading support
- ✅ Good performance
- ✅ Growing ecosystem
- ✅ Familiar to React developers

**Cons:**
- ⚠️ Still relatively new (0.x versions)
- ⚠️ Smaller widget ecosystem than React
- ⚠️ Less mature than Tauri or egui
- ⚠️ Documentation still growing
- ⚠️ Limited third-party component libraries

**Bundle Size:** ~2-3 MB

**Development Time:** Medium (React-like but pure Rust)

**Example Projects:**
- [Dioxus examples](https://github.com/DioxusLabs/dioxus)

---

## Feature Comparison Matrix

| Feature | Tauri+React | egui | Iced | Slint | Dioxus |
|---------|-------------|------|------|-------|--------|
| **Native Look** | ⭐⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ |
| **Syntax Highlighting** | ⭐⭐⭐⭐⭐ | ⭐⭐ | ⭐ | ⭐⭐ | ⭐⭐ |
| **Table/Grid Views** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ |
| **File Parsing Libs** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **Dev Experience** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **Performance** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **Bundle Size** | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **Community** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐ |
| **Maturity** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ |
| **Documentation** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ |
| **Pure Rust** | ❌ | ✅ | ✅ | ✅ | ✅ |

## Detailed Analysis

### Critical Requirements

1. **Excel/CSV Table Display**
   - **Tauri+React**: Excellent - use [react-data-grid](https://github.com/adazzle/react-data-grid) or [ag-grid](https://www.ag-grid.com/)
   - **egui**: Good - [egui_extras::TableBuilder](https://docs.rs/egui_extras/latest/egui_extras/struct.TableBuilder.html)
   - **Others**: Would need custom implementation or basic tables

2. **Syntax Highlighting for DSL**
   - **Tauri+React**: Excellent - [Monaco Editor](https://microsoft.github.io/monaco-editor/) (VS Code's editor) or [CodeMirror](https://codemirror.net/)
   - **egui**: Basic - would need custom lexer or integration with [syntect](https://docs.rs/syntect/)
   - **Others**: Would need custom implementation

3. **File Drag-and-Drop**
   - **Tauri**: Built-in API with [tauri::api::dialog](https://tauri.app/v1/api/js/dialog/)
   - **egui**: Requires platform-specific implementation
   - **Others**: Platform-specific or limited support

4. **Real-time Preview Performance**
   - **Tauri**: Good (React rendering + Rust backend)
   - **egui**: Excellent (immediate mode, very fast updates)
   - **Others**: Good to Excellent

### Use Case Analysis

**For Dadagen specifically:**
- Need to display potentially large CSV/Excel tables
- Syntax highlighting for DSL is important for usability
- File parsing is CPU-intensive (Rust backend is a must)
- Export operations should not block UI

## Recommendation

### Primary Recommendation: **Tauri + React/TypeScript**

**Rationale:**
1. **Rich Ecosystem**: Access to battle-tested React components for tables, editors, and file handling
2. **Syntax Highlighting**: Monaco Editor provides VS Code-quality DSL editing out of the box
3. **Development Speed**: Faster to implement complex UI features with existing libraries
4. **Maintainability**: Large community means easier to find help and contributors
5. **Professional Polish**: Can achieve high-quality UX matching commercial applications
6. **Hybrid Benefits**: Rust handles heavy computation (parsing, generation), React handles UI

**Best For:**
- Applications that need rich, complex UI components
- Teams familiar with web technologies
- Projects requiring professional-looking interface quickly

### Alternative: **egui** (If Pure Rust is Priority)

**When to Choose:**
- Team wants 100% Rust codebase
- Performance is absolutely critical (though Tauri is fast enough)
- Simpler UI requirements (don't need Monaco-level editor)
- Want to share code with WASM web version more easily

**Trade-offs:**
- More development time for complex widgets
- Less polished out-of-the-box experience
- Would need to implement custom DSL editor

## Implementation Path Forward

### Recommended: Tauri + React

1. **Phase 1: Setup** (Task 6.1)
   - Initialize Tauri v2 project with React template
   - Configure build pipeline (Vite + Rust)
   - Set up IPC communication between frontend and backend
   
2. **Phase 2: Core Features** (Tasks 6.2-6.3)
   - Implement file upload with [tauri-plugin-dialog](https://github.com/tauri-apps/tauri-plugin-dialog)
   - Use [react-data-grid](https://github.com/adazzle/react-data-grid) for table display
   - Integrate [Monaco Editor](https://github.com/suren-atoyan/monaco-react) for DSL editing
   - Rust backend handles all parsing and generation

3. **Phase 3: Polish** (Task 6.4)
   - Add drag-and-drop
   - Real-time preview updates
   - Export dialogs and file operations

### Alternative Path: Pure Rust (egui)

If team prefers 100% Rust:

1. **Phase 1: Setup**
   - Use [eframe](https://docs.rs/eframe/latest/eframe/) (egui + window management)
   - Set up basic window and layout

2. **Phase 2: Custom Widgets**
   - Build custom table widget for CSV/Excel display
   - Implement basic code editor (or integrate [egui-code-editor](https://github.com/0x00002a/egui-code-editor))
   - File dialog with [rfd](https://docs.rs/rfd/latest/rfd/)

3. **Phase 3: Polish**
   - Custom DSL syntax highlighting
   - Drag-and-drop via [egui-dnd](https://github.com/lucasmerlin/egui_dnd)

## Decision Criteria

**Choose Tauri + React if:**
- ✅ Want fastest time to market
- ✅ Need professional UI polish
- ✅ Team comfortable with React/TypeScript
- ✅ Rich editing features are important

**Choose egui if:**
- ✅ Want pure Rust codebase
- ✅ Simpler UI requirements acceptable
- ✅ Performance is absolutely critical
- ✅ Want to share more code with WASM version

**Choose other native Rust if:**
- ⚠️ Specific requirements (e.g., Slint's design tools)
- ⚠️ Have time for custom widget development
- ⚠️ Want to experiment with newer frameworks

## Conclusion

**Recommendation: Proceed with Tauri + React/TypeScript** as originally designed.

This provides the best balance of:
- Development speed
- UI quality and polish
- Access to existing libraries
- Community support
- Ability to deliver a professional product quickly

The hybrid approach is well-proven (VS Code, Discord, Figma all use similar architecture) and Tauri's architecture keeps the bundle size reasonable while providing native performance for heavy operations.

**Alternative considered**: egui is the strongest pure-Rust option and should be reconsidered if:
- Pure Rust becomes a hard requirement
- We want tighter integration with the WASM web version
- Performance profiling shows Tauri approach has bottlenecks

**Next Step**: Proceed with Task 6.1 using Tauri + React unless there are strong objections to the hybrid approach.
