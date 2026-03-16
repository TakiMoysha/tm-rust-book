# Game UI

Тестовое Bevy приложение для изучения работы с UI и переключением между сценами.

## TODO

- [x] States: LoadingScreen, MainMenu, Options, Playing
- [x] MainMenu UI (Play, Options, Quit)
- [x] Playing сцена (простой объект)
- [ ] InGameMenu (ESC: Resume, Options, Exit to Menu)
- [x] Options UI (общий для MainMenu и Playing)
  - [ ] Две спеки: MainMenu (все настройки) / Playing (только runtime)
- [ ] LoadingScreen

# Cargo

**Features** - фичи, настраиваемые пакеты. По дефолту настроен "dev_native", который поддягивает дополнительные крейты.

```bash
cargo run --features "bevy/file_watcher" # запустить но вместо features по умолчанию переданныe
cargo build --release --features "dev" # собрать релизную сборку, но с инструментами из dev
cargo build --release --no-default-features # собрать релизную сборку
```

**Детализированные Features** - настраиваются конкретно под бинарники, что бы не тянуть их во все пакеты.

```toml
[[bin]]
name = "test_shaders"
path = "src/bin/test_shaders.rs"
required-features = ["dev", "bevy/bevy_shader"]
```

## Examples/Bin

- `native_ui`: working with native ui elements;
- `native_shader`: working with native shaders, added simple shader;
- `native_material`: working with materials, try added ocean shader to sphere;

- `states_simple`: working with bevy states;

- `camera_movement`: control camera movement (WASD + mouse motion);
- `camera_rtslike`: RTS-like camera movement (WASD + mouse motion, scrolling, top-down view);
- `camera-blender`: camera movement like in blender (editors);
