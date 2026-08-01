#!/bin/bash

# Copy service file to systemd directory
sudo cp distributed-system.service /etc/systemd/system/

# Reload systemd daemon
sudo systemctl daemon-reload

# Enable the service
sudo systemctl enable distributed-system.service

# Start the service
sudo systemctl start distributed-system.service

# Check status
sudo systemctl status distributed-system.service
