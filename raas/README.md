## This was cloned from this repo:

https://github.com/thebracket/ArdanRustService

## Install MD books

See the documentation here:
https://rust-lang.github.io/mdBook/index.html

```sh
cargo install mdbook
```

## Using Tracing

- cargo add tracing-appender
- cargo add tracing-subscriber -F json
- cargo add tracing

## Open Telemetry

Standard for distributed applications.

- cargo add opentelemetry
- cargo add opentelemetry-otlp
- cargo add opentelemetry-http
- cargo add utoipa
- cargo add utopia-redoc
- cargo add utopia-swagger-ui

## Handling Service Configuration

Configuration in a service-oriented environment can be quite complicated. Depending upon your setup, you may want to
receive configuration from one or more of these targets:

- Environment Variables --- particularly in Kubernetes and Docker based systems, passing configuration by environment
  variable is very common, often required.
- Configuration Files --- you may want to read a configuration file and obtain settings from there.
  HTTP --- some orchestration systems provide a unified configuration management setup, expecting your application to
  retrieve configuration over HTTP.
- Command Line --- and you may just want to configure parts of your application from the command-line. If there is setup
  involved in bootstrapping your service (for example, adding first users to an authentication stack), you may even
  require support for this.
- cargo add dotenvy

## Loading config via http

We can load config using HTTP url.

## CLI config with clap

cargo install clap

## gRPC

Many REST APIs look a lot like remote procedure calls (RPC): you call a function remotely, process the response, and
treat it like a regular function call. The only difference being that the function executes elsewhere.

Writing a full Reqwest handler and sharing data types works---but it's laborious. It's even more laborious when you want
to handle clients running other languages.

Google invented gRPC for this use-case. gRPC uses protobuf to define protocols, and provides some automatic framework
creation. Rust isn't on the officially blessed list of languages yet, but the Tokio team have built tonic to help.

### gRPC with Tonic
cargo add tonic