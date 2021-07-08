# JWTManipulator9000
A WASM filter made to manipulate JWT token headers and payloads (currently only supports string parameters). Works best with the Meshery Project :)

DISCLAIMER: This filter doesn't regenerate the signature of the modified JWT, and provides no protections. Proceed with caution!

Sample configuration to be passed:
```json
{
  "add_header": [
    ["header1","value1"],
    ["header2","value2"]
  ],
  "del_header":[
    "header1"
  ],
  "add_payload": [
    ["payload1","value1"],
    ["payload2","value2"],
  ],
  "del_payload":[
    "payload1"
  ],
  "payload_to_header": [
    "payload2"
  ],
  "header_to_payload": [
    "header2"
  ]
}
```
