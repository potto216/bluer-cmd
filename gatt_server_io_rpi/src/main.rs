//! Serves a Bluetooth GATT application using the IO programming model.

use bluer::{
    adv::Advertisement,
    gatt::{
        local::{
            characteristic_control, service_control, Application, Characteristic, CharacteristicControlEvent,
            CharacteristicNotify, CharacteristicNotifyMethod, CharacteristicWrite, CharacteristicWriteMethod,
            Service,
        },
        CharacteristicReader, CharacteristicWriter,
    },
};
use futures::{future, pin_mut, StreamExt};
use std::{collections::BTreeMap, time::Duration};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    time::{interval, sleep},
};

include!("gatt.inc");

use std::error::Error;
use std::thread;

use rppal::gpio::Gpio;
use rppal::system::DeviceInfo;

//Channel No.   RPi Pin No. wiringPi    BCM	Descriptions
//CH1           37          P25         26  Channel 1
//CH2           38          P28         20  Channel 2
//CH3           40          P29         21  Channel 3

// Gpio uses BCM pin numbering. BCM GPIO 23 is tied to physical pin 16.
const GPIO_LED: u8 = 23;
const GPIO_RELAY_CH1: u8 = 26;
const GPIO_RELAY_CH2: u8 = 20;
const GPIO_RELAY_CH3: u8 = 21;

use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "gatt_server_io_rpi", about = "A command tool to be a GATT server for LE data for the RPI")]
struct Opt {
    /// Activate debug mode
    // short and long flags (-d, --debug) will be deduced from the field's name
    #[structopt(short, long, help="Show additional information for troubleshooting such as details about the adapters")]
    debug: bool,
    // short and long flags (-a, --advertiser) will be deduced from the field's name     
    #[structopt(short, long, required=true, help="The GATT server address in the form XX:XX:XX:XX:XX:XX  ex: 5C:F3:70:A1:71:0F")]
    server: String,

    // short and long flags (-u, --uuid-service) will be deduced from the field's name     
    #[structopt(short, long, default_value="", help="This is the service to except from the advertiser. ex: 123e4567-e89b-12d3-a456-426614174000")]
    uuid_service: String,

}



#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(),Box<dyn Error>> { //bluer::Result<()> {

    let opt = Opt::from_args();
    
    env_logger::init();

    let debug_mode = opt.debug;    
    if debug_mode
    {
        println!("{:?}", opt);
    }


    //******Setup GPIO*************/    
    println!("Blinking an LED on a {}.", DeviceInfo::new()?.model());

    //let mut rpi_pin = Gpio::new()?.get(GPIO_LED)?.into_output();
    let mut rpi_pin_relay_ch1 = Gpio::new()?.get(GPIO_RELAY_CH1)?.into_output();
    let mut rpi_pin_relay_ch2 = Gpio::new()?.get(GPIO_RELAY_CH2)?.into_output();
    let mut rpi_pin_relay_ch3 = Gpio::new()?.get(GPIO_RELAY_CH3)?.into_output();
/*
    // Blink the LED by setting the pin's logic level high for 500 ms.
    println!("CH 1 pin high.");
    rpi_pin_relay_ch1.set_high();
    thread::sleep(Duration::from_millis(1000));
    println!("CH 1 pin low.");
    rpi_pin_relay_ch1.set_low();
    thread::sleep(Duration::from_millis(1000));
    println!("CH 1 pin high.");
    rpi_pin_relay_ch1.set_high();
    

    println!("CH 2 pin high.");
    rpi_pin_relay_ch2.set_high();
    thread::sleep(Duration::from_millis(1000));
    println!("CH 2 pin low.");
    rpi_pin_relay_ch2.set_low();
    thread::sleep(Duration::from_millis(1000));
    println!("CH 2 pin high.");
    rpi_pin_relay_ch2.set_high();
    


    println!("CH 3 pin high.");
    rpi_pin_relay_ch3.set_high();
    thread::sleep(Duration::from_millis(1000));
    println!("CH 3 pin low.");
    rpi_pin_relay_ch3.set_low();
    thread::sleep(Duration::from_millis(1000));
    println!("CH 3 pin high.");
    rpi_pin_relay_ch3.set_high();    
*/

    //******Setup Bluetooth*****************/
    let my_address = opt.server;

    let session = bluer::Session::new().await?;

    let _uuid_service = opt.uuid_service;         
        
    let adapter_names = session.adapter_names().await?;
    let adapter_name = adapter_names.first().expect("No Bluetooth adapter present");
    let mut adapter = session.adapter(adapter_name)?;
    for adapter_name in adapter_names {
        println!("Checking Bluetooth adapter {}:", &adapter_name);
        let adapter_tmp = session.adapter(&adapter_name)?;
        let address = adapter_tmp.address().await?;
        if  address.to_string() == my_address {
            adapter =  adapter_tmp;
            break;
        }
    };
    //let adapter_name = adapter_names.first().expect("No Bluetooth adapter present");
    //let adapter = session.adapter(adapter_name)?;
    let adapter_name = adapter.name();
    adapter.set_powered(true).await?;

    if debug_mode
    {
        println!("    Adapter name:               {}", adapter_name);
        println!("    Address:                    {}", adapter.address().await?);
        println!("    Address type:               {}", adapter.address_type().await?);
        println!("    Friendly name:              {}", adapter.alias().await?);
        println!("    System name:                {}", adapter.system_name().await?);
        println!("    Modalias:                   {:?}", adapter.modalias().await?);
        println!("    Powered:                    {:?}", adapter.is_powered().await?);        
    }

    let mut manufacturer_data = BTreeMap::new();
    manufacturer_data.insert(MANUFACTURER_ID, vec![0x21, 0x22, 0x23, 0x24]);
    let le_advertisement = Advertisement {
        service_uuids: vec![SERVICE_UUID].into_iter().collect(),
        manufacturer_data,
        discoverable: Some(true),
        local_name: Some("gatt_server".to_string()),
        ..Default::default()
    };
    let adv_handle = adapter.advertise(le_advertisement).await?;

    println!("Serving GATT service on Bluetooth adapter {}", adapter.name());
    println!("SERVICE_UUID = {}", SERVICE_UUID);
    println!("CHARACTERISTIC_UUID = {}", CHARACTERISTIC_UUID);
    println!("CHARACTERISTIC_SWITCH_UUID = {}", CHARACTERISTIC_SWITCH_UUID);

    let (service_control, service_handle) = service_control();
    let (char_control, char_handle) = characteristic_control();
    let (char_control_dio, char_handle_dio) = characteristic_control();
    let app = Application {
        services: vec![Service {
            uuid: SERVICE_UUID,
            primary: true,
            characteristics: vec![Characteristic {
                uuid: CHARACTERISTIC_UUID,
                write: Some(CharacteristicWrite {
                    write: true,
                    write_without_response: true,
                    method: CharacteristicWriteMethod::Io,
                    ..Default::default()
                }),
                notify: Some(CharacteristicNotify {
                    notify: true,
                    method: CharacteristicNotifyMethod::Io,
                    ..Default::default()
                }),
                control_handle: char_handle,
                ..Default::default()
            },
            Characteristic {
                uuid: CHARACTERISTIC_SWITCH_UUID,
                write: Some(CharacteristicWrite {
                    write: true,
                    write_without_response: true,
                    method: CharacteristicWriteMethod::Io,
                    ..Default::default()
                }),           
                notify: Some(CharacteristicNotify {
                    notify: true,
                    method: CharacteristicNotifyMethod::Io,
                    ..Default::default()
                }),
                control_handle: char_handle_dio,
                ..Default::default()
            }
            
            ],
            control_handle: service_handle,
            ..Default::default()
        }],
        ..Default::default()
    };
    let app_handle = adapter.serve_gatt_application(app).await?;

    println!("Service handle is 0x{:x}", service_control.handle()?);
    println!("Characteristic handle is 0x{:x}", char_control.handle()?);
    println!("Characteristic digital IO handle is 0x{:x}", char_control_dio.handle()?);    

    println!("Service ready. Press enter to quit.");
    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();

    let mut value: Vec<u8> = vec![0x10, 0x01, 0x01, 0x10];
    let mut read_buf = Vec::new();
    let mut reader_opt: Option<CharacteristicReader> = None;
    let mut writer_opt: Option<CharacteristicWriter> = None;

    let mut dio_value:  Vec<u8> = vec![0x00];
    let mut dio_read_buf = Vec::new();
    let mut dio_reader_opt: Option<CharacteristicReader> = None;
    let mut dio_writer_opt: Option<CharacteristicWriter> = None;


    let mut interval = interval(Duration::from_secs(1));
    pin_mut!(char_control);
    pin_mut!(char_control_dio);    

    loop {
        tokio::select! {
            _ = lines.next_line() => break,
            evt = char_control.next() => {
                match evt {
                    Some(CharacteristicControlEvent::Write(req)) => {
                        println!("Accepting write event with MTU {}", req.mtu());
                        read_buf = vec![0; req.mtu()];
                        reader_opt = Some(req.accept()?);
                    },
                    Some(CharacteristicControlEvent::Notify(notifier)) => {
                        println!("Accepting notify request event with MTU {}", notifier.mtu());
                        writer_opt = Some(notifier);
                    },
                    None => break,
                }
            }

            evt_dio = char_control_dio.next() => {
                match evt_dio {
                    Some(CharacteristicControlEvent::Write(req)) => {
                        println!("Accepting DIO Switch write event with MTU {}", req.mtu());
                        dio_read_buf = vec![0; req.mtu()];
                        dio_reader_opt = Some(req.accept()?);
                    },
                    Some(CharacteristicControlEvent::Notify(notifier)) => {
                        println!("Accepting DIO notify Switch request event with MTU {}", notifier.mtu());
                        dio_writer_opt = Some(notifier);
                    },
                    None => break,
                }
            }
            _ = interval.tick() => {
                println!("Decrementing each element by one");
                for v in &mut *value {
                    *v = v.saturating_sub(1);
                }

                println!("Value is {:x?}", &value);
                if let Some(writer) = writer_opt.as_mut() {
                    println!("Notifying with value {:x?}", &value);
                    if let Err(err) = writer.write(&value).await {
                        println!("Notification stream error: {}", &err);
                        writer_opt = None;
                    }
                }
                
                println!("DIO Value is {:x?}", &dio_value);
                if let Some(dio_writer) = dio_writer_opt.as_mut() {
                    println!("Notifying with DIO value {:x?}", &dio_value);
                    if let Err(err) = dio_writer.write(&dio_value).await {
                        println!("Notification stream error: {}", &err);
                        dio_writer_opt = None;
                    }
                }
            }
            read_res = async {
                match &mut reader_opt {
                    Some(reader) => reader.read(&mut read_buf).await,
                    None => future::pending().await,
                }
            } => {
                match read_res {
                    Ok(0) => {
                        println!("Write stream ended");
                        reader_opt = None;
                    }
                    Ok(n) => {
                        value = read_buf[0..n].to_vec();
                        println!("Write request with {} bytes: {:x?}", n, &value);
                    }
                    Err(err) => {
                        println!("Write stream error: {}", &err);
                        reader_opt = None;
                    }
                }
            }
            
            dio_read_res = async {
                match &mut dio_reader_opt {
                    Some(dio_reader) => dio_reader.read(&mut dio_read_buf).await,
                    None => future::pending().await,
                }
            } => {
                match dio_read_res {
                    Ok(0) => {
                        println!("DIO Write stream ended");
                        dio_reader_opt = None;
                    }
                    Ok(n) => {
                        dio_value = dio_read_buf[0..n].to_vec();
                        println!("DIO Write request with {} bytes: {:x?}", n, &dio_value);
                        if dio_value[0] == 0
                        {
                            println!("CH 1 pin high.");
                            rpi_pin_relay_ch1.set_high();
                            
                        } 
                        else if dio_value[0] == 1
                        {
                            println!("CH 1 pin low.");
                            rpi_pin_relay_ch1.set_low();
                        }
                    }
                    Err(err) => {
                        println!("DIO Write stream error: {}", &err);
                        dio_reader_opt = None;
                    }
                }
            }            
        }
    }

    println!("Removing service and advertisement");
    drop(app_handle);
    drop(adv_handle);
    sleep(Duration::from_secs(1)).await;

    Ok(())
}
