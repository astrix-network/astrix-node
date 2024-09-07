# Astrix WASM SDK

An integration wrapper around [`astrix-wasm`](https://www.npmjs.com/package/astrix-wasm) module that uses [`websocket`](https://www.npmjs.com/package/websocket) W3C adaptor for WebSocket communication.

This is a Node.js module that provides bindings to the Astrix WASM SDK strictly for use in the Node.js environment. The web browser version of the SDK is available as part of official SDK releases at [https://github.com/astrix-network/astrix-node/releases](https://github.com/astrix-network/astrix-node/releases)

## Usage

Astrix NPM module exports include all WASM32 bindings.
```javascript
const astrix = require('astrix');
console.log(astrix.version());
```

## Documentation

Documentation is available at [https://astrix.aspectron.org/docs/](https://astrix.aspectron.org/docs/)


## Building from source & Examples

SDK examples as well as information on building the project from source can be found at [https://github.com/astrix-network/astrix-node/tree/master/wasm](https://github.com/astrix-network/astrix-node/tree/master/wasm)

## Releases

Official releases as well as releases for Web Browsers are available at [https://github.com/astrix-network/astrix-node/releases](https://github.com/astrix-network/astrix-node/releases).

Nightly / developer builds are available at: [https://aspectron.org/en/projects/astrix-wasm.html](https://aspectron.org/en/projects/astrix-wasm.html)

