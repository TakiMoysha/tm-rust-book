## Big Picture

- Read config from toml file.
- Prioritize addresses.
- Fault tolerance.
- Minimum latency.

**Round-Robin** - distributes requests evenly among all available servers.
**Pick-First** - always selects the first available address, but does not switch to higher priority addresses if they become available after the initial connection.

In config file addresses order by priority, (0, 1, 2, ...).
Always selected the first available server with highest priority.
Automation switching to higher priority addresses if they become available after the initial connection.

## References

1. [Балансировщик нагрузки с приоритизацией / habr.com](https://habr.com/ru/companies/vk/articles/858290/)
2. []()
