# Concurrent editing manager (CEM) [:hammer: WORK IN PROGRESS :hammer:]

[![Actions Status](https://github.com/Leinvedan/cem/workflows/Rust/badge.svg)](https://github.com/Leinvedan/cem/actions)

## Table of contents

1. [About](#about)
1. [API Interface](#api-interface)
2. [Webpage example](#webpage-example)
3. [License](#license)
4. [Contribution](#contribution)

## About

Simple lightweight concurrent state editing API using [Tokio]() and websockets. There's a webpage example using react inside the `webpage` directory, see more in [Webpage example](#running-webpage-example).

You can start the API by running `cargo run` in your terminal!

## API Interface

The API currently uses the [json-patch](http://jsonpatch.com/) format, provided by the [json-patch rust crate](https://github.com/idubrov/json-patch). Currently the API only supports merging but will support `patch` operations in the future :smiley: .

CEM will try to parse the payload into a `Block` which consists of a json in the following format:

```js
{
  "id": "mylovellyid123",
  "content": {
    "time": 2, // => this is optional
    ...
  }
}
```

| Field                       | Description                                                               |
|-----------------------------|---------------------------------------------------------------------------|
| id                          | unique to this block, will be used as key to update future blocks         |
| content                     | object containing data which will be updated                              |
| time (unsigned 64 bits int) | if a block's time is less than the stored one, the block will be ignored  |


After processing the message, CEM will broadcast to all connected users(except who updated) the updated block in the same `Block` format.

## Webpage example

### setup

1. run `cargo install --path .` to install the API dependencies
2. access the webpage directory and run `npm install` to install the webpage dependencies

### running

1. Start the API by running `cargo run` in the root of the project
2. in another terminal window, access the webpage directory and run `npm start`

## License

This project is licensed under the [MIT license](https://github.com/Leinvedan/cem/blob/master/LICENSE).

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in CEM by you, shall be licensed as MIT, without any additional terms or conditions.