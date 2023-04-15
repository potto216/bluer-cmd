#!/bin/bash
sudo btmgmt --index hci0 power off
sudo btmgmt --index hci0 bredr off
sudo btmgmt --index hci0 power on
sudo btmgmt --index hci1 power off
sudo btmgmt --index hci1 bredr off
sudo btmgmt --index hci1 power on

