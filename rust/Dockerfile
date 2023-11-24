FROM alpine:3.14

ADD ./target/release/gome .
RUN chmod +x gome
EXPOSE 8000

CMD ["gome"]
