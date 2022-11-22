FROM debian:bookworm-slim

COPY ./bin/docker_entrypoint.sh .

RUN chmod +x /docker_entrypoint.sh

COPY ./bin/emerald_herald /usr/local/bin/emerald_herald
CMD ["/docker_entrypoint.sh"]