# JWTManipulator9000
A WASM filter made to manipulate JWT token headers and payloads. Works best with the Meshery Project :)

Sample configuration to be passed:
```json
{
  "add_header": [
    ["test1","test1"],
    ["test2","test2"]
  ],
  "del_header":[
    "test1"
  ],
  "add_payload": [
    ["test3","test3"],
    ["test4","test4"]
  ],
  "del_payload":[
    "test3"
  ],
  "payload_to_header": [
    "test2"
  ],
  "header_to_payload": [
    "test4"
  ]
}
```