#

1. Subscribe to a real-time websocket data feed;
2. Stream the data to Kafka / Redis;
3. Save the data into a SQL database.

```
┌─────────────────────┐      ─────────────────      ┌───────────────┐
│ Websocket Data Feed │ --->   Redis / Kafka   ---> │ SQL Data Sink │
└─────────────────────┘      ─────────────────      └───────────────┘
```

# References

- Building a Redis / Kafka Data Sink - https://www.sea-ql.org/blog/2024-05-05-redis-kafka-data-sink/
