FROM istio/proxyv2:1.12.0-rc.1
ENTRYPOINT /usr/local/bin/envoy -c /etc/envoy.yaml -l debug --service-cluster proxy 
