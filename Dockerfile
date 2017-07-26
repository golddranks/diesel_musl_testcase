FROM alpine:latest
ADD target/release/serve /usr/local/bin/serve
CMD ["/usr/local/bin/serve"]
