POST http://127.0.0.1:8787
Accept: application/dns-message
Content-Type: application/dns-message
# Binary Data
base64,q80BAAABAAAAAAAAA3d3dwdleGFtcGxlA2NvbQAAAQAB;

HTTP 200
[Asserts]
header "Content-Type" contains "application/dns-message"


GET http://127.0.0.1:8787?name=example.com&type=AAAA
Accept: application/dns-json

HTTP 200
[Asserts]
header "Content-Type" contains "application/dns-json"


GET http://127.0.0.1:8787?dns=q80BAAABAAAAAAAAA3d3dwdleGFtcGxlA2NvbQAAAQAB
Accept: application/dns-message

HTTP 200
[Asserts]
header "Content-Type" contains "application/dns-message"

