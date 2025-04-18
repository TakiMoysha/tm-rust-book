# Organization Flat Medium-Sized Web Service

## Structure

Вместо стандартных пакетов с `src` используется плоская структура.

> если нужно указать `build.rs`, то нужно использовать стандартную структуру

## Network Layer

Преобразование HTTP запросов в структуры для использовния в cлое сервисов и обратно.
Сетевой API в rust (http, webscokets, ...) нестабилен и такая прокладка упрощает поддержку кода.

## Services Layer

Отвечает за всю бизнес-логику: авторизация, валидация, workflow и т.д.

## Repositories

Единственный смысл репозиториев - инкапсуляция логики доступа к данным, в частности доступ к БД и выполнение запросов

## Background Jobs (queue)

https://kerkour.com/rust-job-queue-with-postgresql

> При работе с Cron jobs это будет сложнее, необходимо выбрать сервер-лидера, который установин джобы, что бы не было дубликатов от каждого инстанса.

## Caching

Уменьшение кол-ва запросов в БД, особенно если они дорогие. При этом кеширование происходит на уровне сервисов, репозитории должы оставаться "глупыми". Кроме того, кеширование часто завязанно на бизнес-правилах, по этому разумно будет управлять этим в сервисах.

> Для большинства случаев достаточно крейта общего назначения вроде `moka`.

## Logging

Через `tracing` можно настроить все необходимое.

## Serving SPA and static assets

Проще управлять статикой на том же сервере, что и API (вместо CDN).

# Bibliography

- [Architecting and building medium-sized web services in Rust / kerkour.com](https://kerkour.com/rust-web-services-axum-sqlx-postgresql)
