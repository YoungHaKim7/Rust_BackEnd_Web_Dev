- https://github.com/gotuzzoj23/redis_streams 
<hr>

# redis

-  log 보기 ```MONITOR```

```
127.0.0.1:6379> MONITOR
OK
```

https://stackoverflow.com/questions/16337107/how-to-access-redis-log-file

# Install & run (Getting started)

## redis/redis-stack-server

To start Redis Stack server using the redis-stack-server image, run the following command in your terminal:

```
docker run -d --name redis-stack-server -p 6379:6379 redis/redis-stack-server:latest
```

## redis/redis-stack


- To start a Redis Stack container using the redis-stack image, run the following command in your terminal:

```bash
docker run -d --name redis-stack -p 6379:6379 -p 8001:8001 redis/redis-stack:latest
```

The docker run command above also exposes RedisInsight on port 8001. You can use RedisInsight by pointing your browser to localhost:8001.


```
docker exec -it redis-stack redis-cli
```

<hr>

# Redis

- https://redis.io/

- https://redis.io/docs/get-started/document-database/

- https://redis.io/docs/install/install-redis/install-redis-on-linux/

- Rust to Redis with Async/Await [Intermediate] | Jeremy Chone
  - https://youtu.be/uD5hBVHwyDM?si=-O6ETgppuprjOvvz


- 도커로 redis 띄우기 https://velog.io/@coastby/redis-docker%EB%A1%9C-redis-%EB%9D%84%EC%9A%B0%EA%B8%B0
