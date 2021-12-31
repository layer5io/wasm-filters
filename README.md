
<p style="text-align:center;" align="center">
  <img align="center" src="https://raw.githubusercontent.com/layer5io/layer5/master/.github/assets/images/layer5/layer5-tag-white-bg.png" width="45%" /></p>

![GitHub contributors](https://img.shields.io/github/contributors/layer5io/layer5.svg)
![GitHub](https://img.shields.io/github/license/layer5io/layer5.svg) 
![GitHub issues by-label](https://img.shields.io/github/issues/layer5io/layer5/help%20wanted.svg?color=%23DDDD00)
[![Slack](https://img.shields.io/badge/Slack-@layer5.svg?logo=slack)](http://slack.layer5.io)
![Twitter Follow](https://img.shields.io/twitter/follow/layer5.svg?label=Follow&style=social)

# WebAssembly Filters for Envoy

A collection of WebAssemby filters for Envoy proxy written in C,C++,C# and Rust for exercising different features provided by envoy-wasm.

See the [Image Hub](https://layer5.io/projects/image-hub) as a related project (a sample application). Also, see Meshery's filter management capabitilies.

### Weekly Meeting Details

See all community meeting details --> https://meet.layer5.io

- Topic: WebAssembly Filters Meeting
- Day: Weekly on Mondays
- Time: 7:30pm IST / 3:00pm UK / 9:00am Central
- WASM Filters call: https://meet.layer5.io/wasm
- Meeting Minutes: https://bit.ly/3zGZgGg

---

## Join the service mesh community!

<a name="contributing"></a><a name="community"></a>
Our projects are community-built and welcome collaboration. 👍 Be sure to see the <a href="https://docs.google.com/document/d/17OPtDE_rdnPQxmk2Kauhm3GwXF1R5dZ3Cj8qZLKdo5E/edit">Layer5 Community Welcome Guide</a> for a tour of resources available to you and jump into our <a href="http://slack.layer5.io">Slack</a>!

<a href="https://meshery.io/community"><img alt="Layer5 Service Mesh Community" src="img/readme/community.svg" style="margin-left:10px;padding-top:5px;" width="110px" align="right" /></a>

<a href="http://slack.layer5.io"><img alt="Layer5 Service Mesh Community" src="img/readme/slack-128.png" style="margin-right:8px;padding-top:5px;" width="140px" align="left" /></a>

<p>
✔️ <em><strong>Join</strong></em> <a href="https://meet.layer5.io">community meetings</a>. See details on the <a href="https://calendar.google.com/calendar/b/1?cid=bGF5ZXI1LmlvX2VoMmFhOWRwZjFnNDBlbHZvYzc2MmpucGhzQGdyb3VwLmNhbGVuZGFyLmdvb2dsZS5jb20">Layer5 community calendar</a>.<br />
✔️ <em><strong>Watch</strong></em> community <a href="https://www.youtube.com/Layer5io?sub_confirmation=1">meeting recordings</a>.<br />
✔️ <em><strong>Access</strong></em> the Community Drive by completing a community <a href="https://layer5.io/newcomer">Member Form</a>.<br />
✔️ <em><strong>Discuss</strong></em> in the <a href="https://discuss.layer5.io">Community Forum</a>.<br />
</p>
<p align="center">
<i>Not sure where to start?</i> Grab an open issue with the <a href="https://github.com/issues?q=is%3Aopen+is%3Aissue+archived%3Afalse+org%3Alayer5io+org%3Ameshery+org%3Aservice-mesh-performance+org%3Aservice-mesh-patterns+label%3A%22help+wanted%22+">help-wanted label</a>.
</p>

<div>&nbsp;</div>

### About Layer5

**Community First**
<p>The <a href="https://layer5.io">Layer5</a> community represents the largest collection of service mesh projects and their maintainers in the world.</p>

**Open Source First**
<p>Our projects establish industry standards and enable service developers, owners, and operators with repeatable patterns and best practices for managing all aspects of distributed services. Our shared commitment to the open source spirit push the Layer5 community and its projects forward.</p>

---

## Get the Rust toolchain

To compile Rust filters to WASM, the nightly toolchain and support for wasm compilation target is needed.
Make sure you have Rust and Cargo installed [using Rustup](https://www.rust-lang.org/tools/install).
If you're on a *nix system (Unix, Linux, MacOS), in the project root directory, run:
```bash
make rust-toolchain
```
This will also install wasm-pack for you.
Also, take a look at [installing wasm-pack](https://rustwasm.github.io/wasm-pack/installer/) for OS other than *nix.

## Upstream

Upstream is a webserver which is used by few of the filters mentioned above. It provides a route for :

* Mock authentication
* Storing Metrics
* Retrieving Metrics

> Build the docker Image for Upstream before proceeding with the examples.

Build Image:
```bash
cd upstream
make
```

## HTTP-Auth

Simulates handling authentication of requests at proxy level. Requests with a header `token` with value `hello` are accepted as authorized while the rest unauthorized. The actual authentication is handled by the Upstream server. Whenever the proxy recieves a request it extracts the `token` header and makes a request to the Upstream server which validates the token and returns a response.

Build and deploy:
```bash
cd http-auth
make run-filtered
```

Test:
```bash
curl  -H "token":"hello" 0.0.0.0:18000 -v # Authorized
curl  -H "token":"world" 0.0.0.0:18000 -v # Unauthorized
```

## TCP-Metrics

Collects simple metrics for every TCP packet and logs it.

Build and deploy:
```bash
cd tcp-metrics
make run-filtered
```

Test:
```bash
curl 0.0.0.0:18000 -v -d "request body"
```

Check the logs for the metrics.

## TCP-Packet-Parse

Parses the contents of every TCP packet the proxy receives and logs it.

Build and deploy:
```bash
cd tcp-packet-parse
make run-filtered
```

Test:
```bash
curl 0.0.0.0:18000 -v -d "request body"
```

Check the logs for the packet contents.

## Singleton-HTTP-Call

An example which depicts an singleton HTTP WASM service which does an HTTP call once every 2 seconds.

Build and deploy:
```bash
cd singleton-http-call
make run-filtered
```

Check the logs for the response of the request.

## Metrics-Store

This example showcases communication between a WASM filter and a service via shared queue. It combines the `Singleton-HTTP-Call` and `TCP-Metrics` examples. The filter collects metrics and enqueues it onto the queue while the service dequeues it and sends it to upstream server where it is stored.

Build and deploy:
```bash
cd metrics-store
make run-filtered
```

Test:
```bash
curl 0.0.0.0:18000 -v -d "request body" # make a few of these calls
curl 0.0.0.0:8080/retrieve -v # Retrieves the stored stats
# x | y | z  === x : downstream bytes, y : upstream bytes, z: the latency for application server to respond 
```
