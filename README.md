
<p style="text-align:center;" align="center">
  <img align="center" src="https://raw.githubusercontent.com/layer5io/layer5/master/assets/images/layer5/layer5-tag-white-bg.png" width="45%" /></p>

![GitHub contributors](https://img.shields.io/github/contributors/layer5io/layer5.svg)
![GitHub](https://img.shields.io/github/license/layer5io/layer5.svg) 
![GitHub issues by-label](https://img.shields.io/github/issues/layer5io/layer5/help%20wanted.svg?color=%23DDDD00)
[![Slack](http://slack.layer5.io/badge.svg)](http://slack.layer5.io)
![Twitter Follow](https://img.shields.io/twitter/follow/layer5.svg?label=Follow&style=social)

# WASM Filters

This repository contains WASM filters in rust exercising different features provided by envoy-wasm.

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

Deploy: 
```bash
cd http-auth
make deploy-filtered
```

Test:
```bash
curl  -H "token":"hello" 0.0.0.0:18000 -v # Authorized
curl  -H "token":"world" 0.0.0.0:18000 -v # Unauthorized
```

## TCP-Metrics

Collects simple metrics for every TCP packet and logs it.

Deploy: 
```bash
cd tcp-metrics
make deploy-filtered
```

Test:
```bash
curl  -H 0.0.0.0:18000 -v -d "request body"
```

Check the logs for the metrics.

## TCP-Packet-Parse

Parses the contents of every TCP packet the proxy recieves and logs it.

Deploy: 
```bash
cd tcp-packet-parse
make deploy-filtered
```

Test:
```bash
curl  -H 0.0.0.0:18000 -v -d "request body"
```

Check the logs for the packet contents.

## Singleton-HTTP-Call

An example which depicts an singleton HTTP WASM service which does an HTTP call once every 2 seconds.

Deploy: 
```bash
cd singleton-http-call
make deploy-filtered
```

Check the logs for the response of the request.

## Metrics-Store

This example showcases communication between a WASM filter and a service via shared queue. It combines the `Singleton-HTTP-Call` and `TCP-Metrics` examples. The filter collects metrics and enqueues it onto the queue while the service dequeues it and sends it to upstream server where it is stored.

Deploy: 
```bash
cd metrics-store
make deploy-filtered
```

Test:
```bash
curl 0.0.0.0:18000 -v -d "request body" # make a few of these calls
curl 0.0.0.0:8080/retrieve -v # Retrieves the stored stats
# x | y | z  === x : downstream bytes, y : upstream bytes, z: the latency for application server to respond 
```

### About Layer5

**Community First**
<p>The <a href="https://layer5.io">Layer5</a> community represents the largest collection of service mesh projects and their maintainers in the world.</p>

**Open Source First**
<p>Our projects establish industry standards and enable service developers, owners, and operators with repeatable patterns and best practices for managing all aspects of distributed services. Our shared commitment to the open source spirit push the Layer5 community and its projects forward.</p>
