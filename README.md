# Physics Cube Game - Bevy ECS Demo

A comprehensive physics-based cube game built with Bevy 0.11, featuring realistic movement mechanics and extensive ECS query pattern examples.

## 🎮 Game Features

- **WASD Movement**: Control a blue cube with keyboard input
- **Realistic Physics**: Acceleration-based movement with friction damping
- **World Boundaries**: Elastic collision with screen edges
- **Smooth Controls**: Normalized diagonal movement for consistent speed

## 🛠️ Technical Features

This project serves as both a playable game and a comprehensive reference for Bevy ECS patterns:

### Core Game Systems
- **Component-based Physics**: Separate LinearVelocity, Acceleration, and Friction components
- **Fixed Timestep Physics**: Frame-rate independent simulation using FixedUpdate
- **Input Handling**: Decoupled WASD input system with configurable force
- **Boundary Collision**: Perfect elastic collisions with world bounds

### ECS Query Examples
- **Basic Intersections**: Tuple-based component combinations
- **With/Without Filters**: Complex entity filtering patterns
- **Entity Relationships**: Parent/child hierarchies and entity references
- **Performance Optimization**: Iterator patterns and archetype analysis
- **Dynamic Queries**: Runtime query construction concepts
- **Change Detection**: Added/Changed component tracking

## 🚀 Quick Start

### Prerequisites
- [Rust](https://rustup.rs/) (latest stable)
- Cargo (included with Rust)

### Running the Game

```bash
# Clone the repository
git clone <repository-url>
cd test-rust-game

# Run the native version
cargo run

# Build for WASM (web deployment)
cargo build --target wasm32-unknown-unknown --profile wasm-release
```

### Controls
- **W/A/S/D** or **Arrow Keys**: Move the cube
- **ESC**: Close the game

### Testing

```bash
# Run all tests
cargo test

# Run with output
cargo test -- --nocapture

# Run specific test
cargo test test_basic_intersection_queries
```

## 📁 Project Structure

```
src/
├── main.rs               # Application entry point and game setup
├── lib.rs                # Module declarations and exports
├── components.rs         # Game components (Player, LinearVelocity, etc.)
├── resources.rs          # Global resources (WorldBounds)
├── input.rs              # WASD input handling system
├── systems.rs            # Physics integration and collision systems
├── query_examples.rs     # Basic to advanced query examples
├── query_utils.rs        # Query utilities and type aliases
├── entity_relations.rs   # Entity relationship patterns
└── dynamic_queries.rs    # Runtime query construction examples

tests/
└── query_integration_tests.rs  # Comprehensive test suite
```

## 🎯 Learning Objectives

This project demonstrates:

### ECS Architecture Patterns
- **Separation of Concerns**: Components, systems, and resources cleanly separated
- **Component Composition**: Physics behavior built from composable components
- **System Scheduling**: Input on Update, physics on FixedUpdate

### Query Techniques
- **Component Intersections**: Finding entities with multiple components
- **Filtering**: With/Without patterns for precise entity selection
- **Performance**: Optimized iteration and archetype-aware design
- **Relationships**: Parent/child and reference patterns
- **Validation**: Entity reference integrity checking

### Best Practices
- **Type Safety**: Leveraging Rust's type system for ECS queries
- **Performance**: Understanding archetype fragmentation and query costs
- **Testing**: Comprehensive test coverage for all query patterns
- **Documentation**: Extensive examples and explanations

## 🔧 Configuration

### Physics Constants
- **Input Force**: 500.0 units (configurable in `input.rs`)
- **Default Friction**: 0.95 (95% velocity retention per frame)
- **World Bounds**: 1280x720 with 50px margin
- **Cube Size**: 50x50 pixels

### Build Profiles
- **Development**: Optimized dependencies, basic optimization for main crate
- **WASM Release**: Size-optimized build with LTO for web deployment

## 🧪 Testing

The project includes comprehensive tests covering:

- **Basic Query Patterns**: Component intersections and filtering
- **Advanced Queries**: Optional components and complex filters
- **Entity Access**: Individual and batch entity operations
- **Mutable Operations**: Safe concurrent access patterns
- **Performance**: Regression testing for query performance
- **Archetype Consistency**: ECS structural validation

## 📚 Educational Value

### For Bevy Beginners
- Clean project structure following Bevy conventions
- Well-commented examples of common ECS patterns
- Progressive complexity from basic to advanced queries

### For ECS Learners
- Practical examples of component composition
- Performance considerations and optimization techniques
- Entity relationship modeling patterns

### For Game Developers
- Physics integration with proper timesteps
- Input handling best practices
- Boundary collision implementation

## 🌐 WASM Deployment

The project is configured for web deployment:

1. **Build for WASM**:
   ```bash
   cargo build --target wasm32-unknown-unknown --profile wasm-release
   ```

2. **Generate WASM bindings** (requires wasm-bindgen):
   ```bash
   wasm-bindgen --out-dir pkg --web target/wasm32-unknown-unknown/wasm-release/bevy_wasm_game.wasm
   ```

3. **Serve with HTTP server** (required for WASM):
   ```bash
   # Example with Python
   python -m http.server 8000
   ```

## 📖 Architecture Deep Dive

### Component Design Philosophy
- **Single Responsibility**: Each component has one clear purpose
- **Composability**: Components combine to create complex behaviors
- **Data-Oriented**: Components are pure data with minimal logic

### System Design Philosophy
- **Functional**: Systems are pure functions that operate on queries
- **Predictable**: Clear input/output through ECS queries
- **Testable**: Systems can be tested in isolation with mock worlds

### Query Design Philosophy
- **Type Safety**: Compile-time guarantees about component access
- **Performance Aware**: Examples show both fast and slow patterns
- **Flexible**: From simple filters to complex entity relationships

## 🤝 Contributing

This project welcomes contributions! Areas for improvement:

- Additional physics features (gravity, collision shapes)
- More complex entity relationship examples
- Performance benchmarking suite
- Additional input methods (gamepad, touch)
- Visual effects and animation systems

## 📄 License

[Add your preferred license here]

## 🙏 Acknowledgments

- [Bevy Engine](https://bevyengine.org/) - Refreshingly simple game engine
- [Rust Community](https://www.rust-lang.org/) - Amazing ecosystem and support
