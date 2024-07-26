## Instructions

cargo run

go to localhost:4000

open browser console

select a very large image or other file, e.g. from https://github.com/samdutton/simpl/tree/gh-pages/bigimage

-> logs: "There was an error TypeError: NetworkError when attempting to fetch resource."

stop server

uncomment ".layer(axum::middleware::from_fn(print_request_body))"

cargo run

go to localhost:4000

open browser console

select a very large file

-> logs: "413" "Payload Too Large" "There was an error Error: Error searching tag: 413"


## What should happen

What I would actually expect is that both versions of the code return the response code 413 Payload Too Large.
It could be that this is just not how HTTP works (what about the newer HTTP versions?),
but this would mean the server always has to buffer the full body, even if it is very large due to large file upload.

"The server might close the connection or return a Retry-After header field." - https://developer.mozilla.org/en-US/docs/Web/HTTP/Status/413
-> it also sounds like "Content-Length: 4012345" should be used to test for payload length, but that still means the whole body has to be buffered
-> I mean, right now (with axum?) we can't say "these first few data packets said it's 4012345 bytes long, close the connection, 413 Payload Too Large"


related: https://github.com/tokio-rs/axum/discussions/2445
