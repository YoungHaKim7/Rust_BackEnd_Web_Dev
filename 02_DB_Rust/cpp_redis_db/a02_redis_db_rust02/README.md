# Result


```
What is returned for key: "First the Key" - Result: "next the World!!"
The length of stream "The first stream" is 1
--> xrevrange stream entity: 1705226493128-0
 --> name:name-01 --> title:title-01--> xread block: The first stream
 --> StreamID: StreamId { id: "1705226493230-0", map: {"name": string-data('"name-02"'), "title": string-data('"title-02"')} }

--> xread block: The first stream
 --> StreamID: StreamId { id: "1705226493333-0", map: {"title": string-data('"title-03"'), "name": string-data('"name-03"')} }

 -->> the end


```
