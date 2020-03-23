FROM ubuntu:19.10

RUN apt update && apt install tor -y

RUN mkdir -p /run/tor \
  && chown -R debian-tor:debian-tor /run/tor \
  && chmod 700 -R /run/tor

COPY torrc /etc/tor/torrc

USER debian-tor

EXPOSE 9050

ENTRYPOINT ["tor"]
