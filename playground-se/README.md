## TODO

XML (.sbc)
Чтение XML использует стандартный .NET XmlSerializer SpaceEngineersAPI.cs:198-203 . Это можно портировать, но вам понадобятся схемы классов MyObjectBuilder_* из SpaceEngineers.ObjectBuilders.dll.

Protobuf (.sbcPB)
Использует MyObjectBuilderSerializer.DeserializePB из VRage SpaceEngineersAPI.cs:78 . Формат Protobuf, но схема (.proto файлы) не включена в репозиторий — она компилирована в DLL.

.mwm файлы
Использует MyModel.LoadModelData() из VRage.Import . Это проприетарный бинарный формат Keen Software House без публичной спецификации.

### TUI (sbc-inspector-cli)

Небольшоая TUI для работы с sbc-файлами.

- продолжить на layout [layouts tutorial / ratatui.rs](https://ratatui.rs/tutorials/json-editor/ui/)

- [ ] упаковка при выборке блока:
  - [x] перевод dds в webp (или png) - через imagemagic (`magick source.dds output.webp`)
  - [ ] поиск и конвертация fbx/FBX файла в bltf(blg)
  - [ ] экспорт с спекой и артифактом

