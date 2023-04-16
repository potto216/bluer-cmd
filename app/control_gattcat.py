import subprocess
import argparse
import re

attribute_name_to_uuid = {
    "Ramp01ServiceByte": "b3f8665e-9514-11ed-9f96-37eb16895c01",
    "Ramp02ServiceByte": "b3f8665e-9514-11ed-9f96-37eb16895c02",
    "Ramp01MinimumValue": "b5720d32-9514-11ed-985d-7300cdba6b00",
    "Ramp01MaximumValue": "b5720d32-9514-11ed-985d-7300cdba6b01",
    "Ramp01CurrentValue": "b5720d32-9514-11ed-985d-7300cdba6b02",
    "Ramp01Command": "b5720d32-9514-11ed-985d-7300cdba6b03",
    "Ramp01CommandStatus": "b5720d32-9514-11ed-985d-7300cdba6b04",
    "Ramp01Status": "b5720d32-9514-11ed-985d-7300cdba6b05",
    "Ramp01StepTime": "b5720d32-9514-11ed-985d-7300cdba6b06",
    "Ramp02MinimumValue": "b5720d32-9514-11ed-985d-7300cdba6b00",
    "Ramp02MaximumValue": "b5720d32-9514-11ed-985d-7300cdba6b01",
    "Ramp02CurrentValue": "b5720d32-9514-11ed-985d-7300cdba6b02",
    "Ramp02Command": "b5720d32-9514-11ed-985d-7300cdba6b03",
    "Ramp02CommandStatus": "b5720d32-9514-11ed-985d-7300cdba6b04",
    "Ramp02Status": "b5720d32-9514-11ed-985d-7300cdba6b05",
    "Ramp02StepTime": "b5720d32-9514-11ed-985d-7300cdba6b06",
}


ramp_command_name_to_value = {
    "RAMP_COMMAND_STOP": 0,
    "RAMP_COMMAND_START": 1,
    "RAMP_COMMAND_RESET": 2,
    "RAMP_COMMAND_TEST_IO": 3,
    "RAMP_COMMAND_RESULT_SUCCESS": 0,
    "RAMP_COMMAND_RESULT_NONE_RECEIVED": 1,
    "RAMP_COMMAND_RESULT_ERROR": 2,
}



ramp_status_name_to_value = {
    "RAMP_STATUS_STOPPED": 0,
    "RAMP_STATUS_RUNNING": 1,
}


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

# 5C:F3:70:A1:71:0F
adapter_address = '5C:F3:70:7B:F5:66'
command=f'gattcat write --bind {adapter_address} 58:BF:25:9C:50:7E {attribute_name_to_uuid["Ramp01ServiceByte"]} {attribute_name_to_uuid["Ramp01Command"]} {ramp_command_name_to_value["RAMP_COMMAND_TEST_IO"]:02x}'
run_command(command)

command=f'gattcat read --bind {adapter_address} 58:BF:25:9C:50:7E {attribute_name_to_uuid["Ramp01ServiceByte"]} {attribute_name_to_uuid["Ramp01Status"]}'
run_command(command)
