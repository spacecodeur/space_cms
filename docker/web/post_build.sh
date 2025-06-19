#!/bin/bash

echo "alias ll='ls -all'" >> /home/appuser/.bashrc

# Set environment variables for Leptos development
export RUST_LOG="info"
export LEPTOS_SITE_ADDR="0.0.0.0:3000"
export LEPTOS_SITE_ROOT="site"

tail -f /dev/null # keep the container running after build