# Client Search Engine

## Design

Поисковой движок для wasm, ориентированный для работы в браузере.

```rust
pub trait SearchEngine {
  // load a database
  pub async fn load_database(&mut self, database: Database) -> Result<(), Error>;
  // execute a plan, actually a query
  async fn _execute(&mut self, plan: Plan) -> Result<Vec<Document>, Error>;
  // build a plan
  pub async fn execute(&mut self, query: Query) -> Result<Vec<Document>, Error>;
}
```

**Responsibility**:

- data persistence
- data encryption
- indexing and retrieval
- [?] synchronization - sharing the computed indexes across devices (Sync Scans)

### Considering

Есть объект БД, сейчас он может хранить только одну таблицу данных.
Это Column-Oriented Storage в IndexedDB, со Schema-on-Read + немного гибрида.

Schema-on-Read - то есть схема определяется пользователем.

Поддерживает примитивные типы: bool, number, text, null, tags(?).

> tags - это поле с несколькими text-значениями, может не стоит пока делать;
> number далее я разделю на числовые типы, timestamp, int, float-point, fixed-point;

На текущем этапе не вижу смысла в JOIN-ах. Если будет расширение ф-ционала с поиском связанных сущностей, то пригодятся (но лучше использовать нормальный инструмент).

#### Storing

За хранение будет отвечать JS, не wasm, это не требует производительности и прощe.

Рассматриваются варианты:

- _LocalStorage_: простое хранилище, широко используется и простое. Есть ограничение на размер хранение и поддерживает только строки, синхронный. Неподходит.
- _IndexedDB_: поддерживает большие объемы данных. Использует `ArrayBuffer`, `Blob` - бинарные. Сложнее: транзакции, объектные хранилища.
- _FileSystem API_: новый метод работы с файлами, может не поддерживаться некоторыми браузерами. Хранить как бинарные файлы. Сложнее: работа с файлами, разрешения.

Пишем под IndexedDB. IndexedDB поддерживает транзакции, индексы, оптимизирован под браузер. На поисковой движок остается только под processor+indexing.

Рассматривается вариант с "Lazy Loading" + "Index-Only", а сами документы только для отображения. То есть, когда пользователь вводит query, то мы загружаем только то, что необходимо, получаем значения и выбираем их из IndexedDB по ID.

JS загружает данные из IndexedDB -> передаем Uint8Array -> WASM преобразует и строит индексы.

Поддержка шардинга в целом не имеет смысла. Если база выростит до значений, что это потребуется, то нужен другой инструмент.

#### Encryption

Шифрование данных, будет как опциональная возможность в дальнейшем.
Браузеры поддерживают AES-GCM через Web Crypto API, можно использовать его. Как альтернатива Deterministic Encryption, AES-SIV.
Криптография будет на целый файл, по этому ее тоже можно оставить на JS.

#### Compression

Реализация компрессии данных.

#### Indexing

Реализация по индексов по каждому `term -> attribute -> document`, где `term` - единица теста (нормализованная lower+stremming), по которой строится индекс.
Индексы можно сделать на `BTreeMap`.

Text Index - поддержка stremming (нахождения основы слова для заданного исходного слова).

#### Query

Язык определения запроса. По опыту - самая сложная, неоднозначная и непонятная штука. В начале буду работать сразу с планов исполнения и helper-ами что бы строить планы.

> [!tldr] Ориентировочно - никакого своего DSL
> Синтаксик json-like: `{ "filters": { "and": [{ "field": "<field_name>", "<expression_key>": "<expression_value}] }, "sort": { "field": "<field_name" }, "limit": <limit> }`.

Компоненты для работы плана: `Filter, Condition, Expression`.

_Filters_: Какие фильтры реализовать по приоритетам

- [1] точный
- [2] prefix
- [3] fuzzy (`levenshtein`, limit 2)
- [?] bm25 (тяжелый, еще подумаем, как альтернатива TF-IDF)
- [?] фасеточный - категоризовать результаты и видеть статистику по ним

### Design of Crate

Как references используются Tantivy и Column-Storage (C-Store).

### Implementation

- Нужен ли Bidirectional Mappings для поиска?

- RPN -> стековый исполнитель: для построения дерева исполнения запросов. Будет только стек, нет рекурсий и деревьев. Прост для сериализации и кэширования.

```rust
struct RpnExecutor {
  stack: Vec<DocIdSet>
}

impl RpnExecutor {
  pub fn execute(&mut self, tokens: &[RpnToken]) -> Result<DocIdSet, Error> {
    for token in tokens {
      match token {
        RpnToken::Term(term) => ...,
        RpnToken::And => ...,
        RpnToken::Or => ...,
        RpnToken::Not => ...
      }
    }
  }
}
```

- `SlotMap` для хранения документов, для работы с обновленями доков.

> [!REVIEW] А может и не нужно
> Если нету облако с синхрой, то не нужно

```rust
struct DocumentStore {
  docs: SlotMap<DocId, Document>
}
```

- `SyncScan` для синхронизации индексов на разных устройствах.

> [!REVIEW] А может и не нужно

#### Optimizations

- `Box<str>` вместо `String`;
- `smallvec` для маленьких векторов, в индексах, `SmallVec<[u32; 8]>` для хранения элементов на стеке: для `term -> doc_ids`, `tags`, `prefix matches`, `fuxxy suggestions`;
- `u32` вместо `usize` для `doc_id`, для экономии в индексах;
- для общения между JS и WASM используется `Uint8Array`, по этому храним данные в серилаизованном бинарном виде (`postcard`);
- wasm не использует настоящие потоки, по этому либа использует `&` и `Cow` для упрощенияж
- [?] совместимость с `C` (`#[repr(C)]`) для упрощения с `WebAssembly.Memory` (но тогда соблюдать выравнивание);
- `hashbrown`: более быстрая реализация хеш-таблицы, вместо `std::collections::HashMap`;
- `blake3` вместо `sha256`;

# Bibliography

- Mailisearch (LMDB, hannoy), Elasticsearch
- https://wasm-bindgen.github.io/wasm-bindgen/examples/add.html
- https://github.com/quickwit-oss/tantivy
- https://jdrouet.github.io/posts/202503191700-search-engine-part-2/
- https://jdrouet.github.io/posts/202503311500-search-engine-part-4/
- CMU Database Group Lectures

**Bidirectional Mappings between indexes and document id** - двойная связь между документами и индексами. Нужно для корректного обновления документа и удаления.

**Reverse Polish Notation (RPN)** - обратная польская запись, запись выражений без скобок, где операторы идут после операндов (калькулятор).
