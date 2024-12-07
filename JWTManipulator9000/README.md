# JWTManipulator9000

A WASM filter designed to manipulate JWT token headers and payloads. It supports modifying string parameters and works best when integrated with the Meshery Project.

DISCLAIMER:
This filter does not regenerate the signature of the modified JWT and provides no protections. Use with caution!
Features
- Add or remove headers and payload entries.
- Transfer values between headers and payload.
- Modify JWT tokens in a flexible and configurable manner.

### Sample Configuration

The configuration should be passed in JSON format. Below is an example configuration:
```json
 {
  "add_header": [
    ["header1", "value1"],
    ["header2", "value2"]
  ],
  "del_header": [
    "header1"
  ],
  "add_payload": [
    ["payload1", "value1"],
    ["payload2", "value2"]
  ],
  "del_payload": [
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
Configuration Parameters

  - add_header: A list of key-value pairs to add to the JWT headers.
  - del_header: A list of header keys to be removed.
  - add_payload: A list of key-value pairs to add to the JWT payload.
  - del_payload: A list of payload keys to be removed.
  - payload_to_header: A list of payload keys whose values will be moved to headers.
  - header_to_payload: A list of header keys whose values will be moved to payloads.


