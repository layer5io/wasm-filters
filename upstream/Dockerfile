# syntax=docker/dockerfile:1

FROM golang:1.19.1 as bd
WORKDIR /github.com/layer5io/wasm-upstream
ADD . .
RUN GOPROXY=direct GOSUMDB=off go build -a -o /upstream .
CMD ["/upstream"]