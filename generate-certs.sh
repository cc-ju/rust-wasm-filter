#!/usr/bin/env bash

# see https://www.envoyproxy.io/docs/envoy/latest/start/sandboxes/double-proxy

rm certs/*.key
rm certs/*.csr

openssl genrsa -out certs/ca.key 4096
openssl genrsa -out certs/example.com.key 2048
openssl req -x509 -new -nodes -key certs/ca.key -sha256 -days 1024 -out certs/ca.crt \
    -subj "/C=DE/ST=NRW/O=MyExample, Inc./CN=My CA"

openssl req -new -sha256 \
     -key certs/example.com.key \
     -subj "/C=DE/ST=NRW/O=MyExample, Inc./CN=proxy-kafka-frontend.example.com" \
     -out certs/proxy-kafka-frontend.example.com.csr

openssl req -new -sha256 \
     -key certs/example.com.key \
     -subj "/C=US/ST=CA/O=MyExample, Inc./CN=proxy-kafka-backend.example.com" \
     -out certs/proxy-kafka-backend.example.com.csr


openssl x509 -req \
     -in certs/proxy-kafka-frontend.example.com.csr \
     -CA certs/ca.crt \
     -CAkey certs/ca.key \
     -CAcreateserial \
     -extfile <(printf "subjectAltName=DNS:proxy-kafka-frontend.example.com") \
     -out certs/kafka-frontend.example.com.crt \
     -days 500 \
     -sha256

cat certs/kafka-frontend.example.com.crt certs/example.com.key > certs/kafka-frontend.example.com-bundle.pem

openssl x509 -req \
     -in certs/proxy-kafka-backend.example.com.csr \
     -CA certs/ca.crt \
     -CAkey certs/ca.key \
     -CAcreateserial \
     -extfile <(printf "subjectAltName=DNS:proxy-kafka-backend.example.com") \
     -out certs/kafka-backend.example.com.crt \
     -days 500 \
     -sha256

cat certs/kafka-backend.example.com.crt certs/example.com.key > certs/kafka-backend.example.com-bundle.pem