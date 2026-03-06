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

## Архитектура

```
src/
├── main.rs           # AppBuilder
├── states.rs         # AppState enum
├── ui/
│   ├── mod.rs
│   ├── main_menu.rs
│   ├── options.rs
│   └── in_game_menu.rs
└── game/
    ├── mod.rs
    └── scene.rs
```

## Управление

- **MainMenu**: кнопки Play, Options, Quit
- **Playing**: ESC - возврат в MainMenu
- **Options**: кнопка Back
