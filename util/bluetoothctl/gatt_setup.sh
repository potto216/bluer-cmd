echo registering mobiwan2 GATT service

coproc bluetoothctl
echo -e 'menu gatt\n' >&${COPROC[1]}
echo -e 'register-service 49535343-fe7d-4ae5-8fa9-9fafd205e000 7\n' >&${COPROC[1]}
echo -e 'yes\n' >&${COPROC[1]}
echo -e 'register-characteristic 49535343-aca3-481c-91ec-d85e28a60318 write,notify 9\n' >&${COPROC[1]} 
echo -e '0\n' >&${COPROC[1]}
echo -e 'register-characteristic 49535343-1e4d-4bd9-ba61-23c647249616 notify 12\n' >&${COPROC[1]}
echo -e '0\n' >&${COPROC[1]}
echo -e 'register-characteristic 49535343-8841-43f4-a8d4-ecbe34729bb3 write 15\n' >&${COPROC[1]}
echo -e '0\n' >&${COPROC[1]}
echo -e 'register-application\n' >&${COPROC[1]}
echo -e 'back\n' >&${COPROC[1]}
echo -e 'advertise.discoverable on\n' >&${COPROC[1]}
echo -e 'advertise on\n' >&${COPROC[1]}

output=$(cat <&${COPROC[0]})
echo $output
