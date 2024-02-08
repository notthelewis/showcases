# Overview

This is a strongly-typed binary message parser written in TypeScript. I wrote this a couple of years ago as part of a 
[medium article](https://medium.com/@lewfar99/using-nodejs-streams-to-build-a-parser-377c04db5181).

It was a culmination of some of my learnings from working at Senti. The main purpose is to take in binary data from 
a source (such as TCP, HTTP or file IO) and parse it into usable data structures (JS Objects). 

# Usage 

To run the server in a production ready fashion, run `npm compile` in the current directory, then in the `dist` directory,
run: `node index.js`. This starts the server. Alternatively, run: `npm start` which will start nodemon in a dev server
environment.

In a separate window, run: `npm test:integration` which runs the test client. This connects to the TCP server and
sends a `door` message to the localhost on port 9999. Logs can be checked.

# Skills shown

- Performant NodeJS code with _minimal_ external dependencies.
- Documentation
- TCP understanding 
- Advanced TypeScript
- Pipe and filter design pattern
- NodeJS project architecture
- NodeJS streams experience
- Binary protocol development
- Object pooling
- Automated unit testing using Jest and integration testing
