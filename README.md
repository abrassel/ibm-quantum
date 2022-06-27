# My implementation of our "quantum computer"

## Intro

Hey, Juan!

First of all, I want to thank you for handing out this project.  It's been a fun time.  The purpose of this README is to provide some thoughts about how I went about this, as well as to give some instructions for how to run my code.

## Architecture

First steps first - if there's some JSON to decode, `serde` is the way to go.  It plays nicely with the strongly typed nature of Rust, and allowed me to neatly deserialize the json input with very little boiler plate.  

Similarly, I was able to use `clap`'s new integration with `structopt` to get some pretty fast, strongly typed command line parsing off the ground.  I ended up adding arguments for the acme and madrid servers' URLs.  It was necessary to disambiguate their addresses.

After the initial setup, one thing that I encountered in my code which I feel like I wasn't sufficiantly able to resolve was the tension between having a closed set of architectures and operations (necessary to parse string architectures, etc), and the inherent non-closedness of different architectures and operations.  I was using `enum_dispatch` for a while, but I settled on creating an `Architecture` trait, and manually mapping `Architecture`s and `Operation`s.  This is unfortunate boilerplate.

The quantum servers are single threaded by default, so there's little point in bottle-necking them with asynchronous requests.  Instead, I opted for two workers, one per architecture/server, each of which ran a single calculation at a time.  This output is piped to a third thread which prints output.  It owns a lock on stdout, just to make things faster.  Rust is notorious for having slow printing without the lock.

Each implementer of architecture issues interpreter instructions.  The `Program` struct abstracts over the underlying architecture to aggregate the instructions and then serialize them with `serde` once more, prior to delegating once more to `Architecture` to send off the program to the underlying server.

The Rust binary makes no assumptions about the servers being started and assumes that input to the program and output from the servers are always well formed.

## Testing

There are some unit tests to determine that `madrid` and `acme` seem to be issuing correct instruction sets.  The bulk of the testing is e2e and done using an external Python script.

I have a `fuzz.py` script which can be used to check against a specific `json` blob, or to generate variable quantum programs and check for correctness.  This is done using the `pytests/util.py` script, which is a straightforward file to evaluate a high level program.  It invokes the rust binary as a subprocess and gathers its output to ensure consistency.  My e2e tests run on this principle, and invoke the rust binary with a few inputs and check against the outputs.  The e2e tests do not assume that the servers are started.

## How to run

1. Launch the `acme` and `madrid` servers.  Note the URL.
2. Compile and run the `ibm-quantum` binary using `cargo`.  `cargo run --release <prog.json> --acme <acme_url> --madrid <madrid_url>` will do the trick.

## How to test
### Cargo unit tests
1. Run `cargo test`
### e2e tests
1. Install `pytest`.
2. Run `pytest`.  Ensure servers are started.
### fuzzing
1. Run `fuzz.py`, optionally with settings configured as in `generate_quantum_programs.py`


## Closing

This has been lots of fun!  Let me know if you have any questions.