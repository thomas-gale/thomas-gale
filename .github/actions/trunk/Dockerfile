FROM thomasgale/basic-trunk
LABEL maintainer="Thomas Gale <mail@thomasjamesgale.com>"

#  GitHub Actions must be run by the default Docker user (root).
USER root

# Copies your code file from your action repository to the filesystem path `/` of the container
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh
ENTRYPOINT ["/entrypoint.sh"]
