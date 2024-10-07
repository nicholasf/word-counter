
# Word Counter
Word counter is a programming exercise to compare Rust, Go and Node to solve a basic HTTP services based challenge. 

I tend to work on backend HTTP services and these are the three languages I have chosen to pursue over time. Implementing the challenge in each language lets me contrast and compare them, and also serves as a talking point to potential employers and coding friends.

## The Challenge

Any amount of text files holding human language words can be set into files. Each word in each file will be sent via a word counter *client* to a *server* in a JSON payload resembling this. The client should be processing multiple files in parallel. 

```json
{
    "title": "A Tale of Two Cities.",
    "word": "The",
    "word_number": "1"
}
```

The server receiving the JSON payload will then have two tasks:

1) to store the word, along with the associated data, in a Postgres database that will allow for querying information about words in a title via HTTP GET services.
2) to offer a HTTP service that can return the average length of a word in a given title, at any given moment while processing.

The choice to use text files holding is really just a convenience for finding a dataset to play with. I like literature, so it lets me see interesting data as I build a coding exercise.

Running `make download_texts` will bring down 10 texts into the `/files` directory for you.

## Directory Layout

There are two `clients` and three `servers`. The clients are written in Go and Rust, and the servers are written in Node, Go and Rust. 

## Database

The same database will be used for all three server side implementations.

