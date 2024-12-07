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

Example Use Case

Imagine you're managing user authentication in a microservices environment, and you need to:

  Add a custom claim to the payload to track user sessions.
  Remove sensitive or irrelevant header information.
  Transfer a specific claim from the payload to a header for an upstream service.

Configuration for this case:
```json
{
  "add_payload": [
    ["session_id", "abc123"]
  ],
  "del_header": [
    "debug_info"
  ],
  "payload_to_header": [
    "user_role"
  ]
}
```
When applied:

  - The filter will add a session_id claim to the payload with a value of abc123.
  - It will remove the debug_info key from the headers.
  - The user_role claim from the payload will be moved to the headers.

Integration with Meshery

To integrate JWTManipulator9000 into your Meshery configuration:

   - Add the filter as part of your WASM filter chain.
   - Provide the desired configuration JSON through Meshery's UI or API.
   - Deploy the filter and observe how it modifies JWT tokens based on your specifications.

For more details about configuring WASM filters with Meshery, visit the official ![Meshery documentation](https://github.com/meshery/.github/blob/master/profile/README.md)
