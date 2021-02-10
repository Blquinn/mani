# Mani is an HTTP manifold

What's that mean?

**Why?**

Graphql has become very popular, for its ability to speed up web requests
by taking a single request with information about multiple requests and

If this works and you don't need graphql types, then this is way simpler,
and doesn't require extra code to be written.


**Why rust?**

This needs to be fast, it will sit in front of all your other services.

I don't want people complaining about garbage collection.

#### TODO

- [ ] Everything.
- [ ] Accept multiple request formats.
- [ ] Configure allowed origins.
- [ ] Non base64 request body to make it human readable.
- [ ] Cookies support.

Mani example:

**Request**

POST http://localhost:9999/mani?format=json

```json
{
    "requests": [
        {
            "url": "https://jsonplaceholder.typicode.com/comments/1",
            "method": "GET",
            "headers": [
                ["Foo", "Bar"],
                ["Foo", "Baz"],
                ["Baz", "Quux"]
            ],
            "body": "base64 string | null"
        },
        {
            "url": "https://jsonplaceholder.typicode.com/comments/2",
            "method": "GET",
            "headers": [
                ["Foo", "Bar"],
                ["Foo", "Baz"],
                ["Baz", "Quux"]
            ],
            "body": "base64 string | null"
        }
    ]
}
```

**Response**

The response objects come in the same order that they were sent in the request.

```json
{
    "responses": [
        {
            "response": {
                "status": "200 OK",
                "headers": [
                    ["Foo", "Bar"]
                ],
                "body": "{ some json from that site } base64 encoded."
            }
        },
        {
            "errors": [
                {
                    "description": "Maybe a DNS resolution error or something?"
                }
            ]
        }
    ]
}
```
