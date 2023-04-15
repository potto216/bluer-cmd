#!/bin/bash
set -x
sudo systemctl stop bluetooth
sudo sh -c 'rm -rf /var/lib/bluetooth/*'
sudo systemctl start bluetooth

