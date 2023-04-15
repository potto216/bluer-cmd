import subprocess
import argparse
import re


def run_command(command):
    # The command to run (replace 'ls' with your desired command)
    if command is None:
        command = "ls" # default command 

    # Run the command and capture the output
    process = subprocess.Popen(command, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True, shell=True)
    stdout, stderr = process.communicate()

    # Check for errors
    if process.returncode != 0:
        print(f"An error occurred while executing the command {command}:\n{stderr}")
    else:
        print(f"Command output:\n{stdout}")


command='gattcat read --bind 5C:F3:70:A1:71:0F 58:BF:25:9C:50:7E b3f8665e-9514-11ed-9f96-37eb16895c01 b5720d32-9514-11ed-985d-7300cdba6b01'
run_command(command)