# Result

```
docker exec -it 69e redis-cli 

```


```

$ docker exec -it 69e redis-cli

127.0.0.1:6379> GET my_key
"Hello world!"

```


```
--> my_key : Hello world!

-->> my_stream len 2

->> xrevrange stream entity: 1705224230523-0
 ->> title: title 01
 ->> name: name-01
->> xrevrange stream entity: 1705223355459-0
 ->> name: name-01
 ->> title: title 01

->> the end

```
