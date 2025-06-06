# Use Node.js as the base image for Next.js
ARG NODE_VERSION=22
FROM node:${NODE_VERSION}-bookworm-slim

RUN apt update

# install some usefull tools/shortcuts

RUN apt install tree -y
RUN apt install curl -y

# create container user with same rights than host user

RUN groupadd -g 1000 appuser && \
    useradd -m -u 1000 -g 1000 -s /bin/bash appuser

# global configuration done ! let's focus on the project itself

WORKDIR /app
RUN chown -R appuser:appuser /app

USER appuser

# Install Next.js dependencies when container starts
# The actual npm install will be handled by the start script to ensure
# package.json is available from the mounted volume