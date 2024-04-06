//! Perform a Bluetooth LE advertisement.

use bluer::adv::Advertisement;
use std::time::Duration;
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    time::sleep,
};

use std::collections::{BTreeSet};
use std::str::FromStr;
use structopt::StructOpt;
use uuid::Uuid; // Make sure the `uuid` crate is added to your dependencies
//use std::str::FromStr;


#[derive(Debug, StructOpt)]
#[structopt(name = "le_advertise", about = "A command tool to generate BLE advertisements")]
struct Opt {
    /// Activate verbose mode
    // short and long flags (-v, --verbose) will be deduced from the field's name
    #[structopt(short, long, help="Show detailed information for troubleshooting, including details about the adapters")]
    verbose: bool,
 
    // short and long flags (-a, --advertiser) will be deduced from the field's name     
    #[structopt(short, long, default_value="", help="The advertisement address in the form XX:XX:XX:XX:XX:XX  ex: 5C:F3:70:A1:71:0F")]
    advertiser: String,

    // short and long flags (-u, --uuid-service) will be deduced from the field's name     
    #[structopt(short, long, default_value="", help="an optional uuid service to add to the advertisement. ex: 123e4567-e89b-12d3-a456-426614174000")]
    uuid_service: String,
    
     /// UUID service to add to the advertisement. ex: 123e4567-e89b-12d3-a456-426614174000
     #[structopt(short = "u", long, use_delimiter = true, help = "List of service UUIDs separated by commas")]
     service_uuids: Vec<String>,
 
     /// Local name to be used in the advertising report.
     #[structopt(short, long, help = "Local name for the advertisement")]
     local_name: Option<String>,
 
     /// Advertise as general discoverable.
     #[structopt(long, help = "Advertise as general discoverable")]
     discoverable: Option<bool>,
 
     /// Duration of the advertisement in seconds.
     #[structopt(long, help = "Duration of the advertisement in seconds")]
     duration: Option<u64>,
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> bluer::Result<()> {

    let opt = Opt::from_args();
    let verbose = opt.verbose;
    env_logger::init();

    let uuid_service = opt.uuid_service;

    let service_uuids: BTreeSet<Uuid> = opt
    .service_uuids
    .iter()
    .filter_map(|s| Uuid::from_str(s).ok())
    .collect();

    let session = bluer::Session::new().await?;
            
        
    let adapter_names = session.adapter_names().await?;
    let adapter_name = adapter_names.first().expect("No Bluetooth adapter present");
    let mut adapter = session.adapter(adapter_name)?;
    for adapter_name in adapter_names {
        let adapter_tmp = session.adapter(&adapter_name)?;
        let address = adapter_tmp.address().await?;

        if verbose {
            println!("Checking Bluetooth adapter {}: with an address {}", &adapter_name, address);
        }        

        if opt.advertiser.is_empty() || address.to_string() == opt.advertiser {
            adapter = adapter_tmp;
            if verbose {
                println!("Using Bluetooth adapter {}", &adapter_name);
                println!("    Address: {}", address);
                // Print additional adapter details here as needed
            }
            break;
        }
    };
 
    adapter.set_powered(true).await?;    
    if opt.verbose {
        println!("Advertising on Bluetooth adapter {}", adapter.name());
        println!("    Address:                    {}", adapter.address().await?);
        println!("    Address type:               {}", adapter.address_type().await?);
        println!("    Friendly name:              {}", adapter.alias().await?);
        println!("    System name:                {}", adapter.system_name().await?);
        println!("    Modalias:                   {:?}", adapter.modalias().await?);
        println!("    Powered:                    {:?}", adapter.is_powered().await?);        
    }
    let le_advertisement = Advertisement {
        advertisement_type: bluer::adv::Type::Peripheral,
        service_uuids: service_uuids,
        local_name: opt.local_name,
        discoverable: opt.discoverable,
        duration: opt.duration.map(Duration::from_secs),
        ..Default::default()
    };

    if verbose    
    {
        println!("{:?}", &le_advertisement);
    }
    let handle = adapter.advertise(le_advertisement).await?;

    println!("Press enter to quit");
    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();
    let _ = lines.next_line().await;

    if verbose
    {
        println!("Removing advertisement");
    }
    drop(handle);
    sleep(Duration::from_secs(1)).await;

    Ok(())
}