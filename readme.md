# trigger kafka traffic

```
docker run -it --rm \
    --network front \
    bitnami/kafka:latest kafka-topics.sh --list  --bootstrap-server kafka.example.com:9092
```
# Docs
- c++ code using the api (`getValue` as a wrapper around `get_property`): https://github.com/istio/proxy/blob/master/extensions/common/context.cc
- Attributes/Properties https://www.envoyproxy.io/docs/envoy/latest/intro/arch_overview/advanced/attributes.html
- some dude trying things: https://github.com/envoyproxy/envoy-wasm/issues/474
- some cisco guys talking about it: https://events.istio.io/istiocon-2021/slides/g4p-Kafka-SebastianZsolt.pdf
