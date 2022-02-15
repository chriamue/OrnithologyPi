use crate::Sighting;
use base64;
use bluer::{
    adv::Advertisement,
    agent::Agent,
    rfcomm::{Profile, Role, Stream},
    Address,
};
use futures::StreamExt;
use image::{self, imageops::FilterType};
use std::error::Error;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    time::sleep,
};

use super::Message;

use super::MANUFACTURER_ID;
pub const SERVICE_UUID: uuid::Uuid = uuid::Uuid::from_u128(0xF00DC0DE00001);
pub const CHARACTERISTIC_UUID: uuid::Uuid = uuid::Uuid::from_u128(0xF00DC0DE00002);
pub const CHANNEL: u8 = 7;
pub const MTU: u16 = 8192;

async fn handle_connection(
    sightings: Arc<Mutex<Vec<Sighting>>>,
    stream: &mut Stream,
    addr: Address,
) -> Result<(), Box<dyn Error>> {
    let recv_mtu = MTU;

    println!(
        "Accepted connection from {:?} with receive MTU {} bytes",
        &addr, &recv_mtu
    );

    if let Err(err) = stream
        .write_all(format!("{:?}", sightings.lock().unwrap().len()).as_bytes())
        .await
    {
        println!("Write failed: {}", &err);
    }

    loop {
        let buf_size = recv_mtu;
        let mut buf = vec![0; buf_size as _];

        let n = match stream.read(&mut buf).await {
            Ok(0) => {
                println!("Stream ended");
                break;
            }
            Ok(n) => n,
            Err(err) => {
                println!("Read failed: {}", &err);
                break;
            }
        };
        let buf = &buf[..n];

        let message = serde_json::from_slice::<Message>(buf);
        match message {
            Ok(Message::Ping) => {
                println!("{:?}", Message::Ping);
                let response = serde_json::to_vec(&Message::Pong).unwrap();

                if let Err(err) = stream.write_all(&response).await {
                    println!("Write failed: {}", &err);
                    continue;
                }
            }
            Ok(Message::Pong) => {
                println!("{:?}", Message::Pong);
            }
            Ok(Message::CountRequest) => {
                let count = {
                    let len = sightings.lock().unwrap().len();
                    len as u64
                };
                let response = serde_json::to_vec(&Message::CountResponse { count }).unwrap();
                if let Err(err) = stream.write_all(&response).await {
                    println!("Write failed: {}", &err);
                    continue;
                }
            }
            Ok(Message::LastRequest) => {
                let sighting = {
                    let mutex = sightings.lock().unwrap();
                    let last = mutex.last();

                    last.unwrap().clone()
                };
                println!("{:?}", sighting);
                let response = serde_json::to_vec(&Message::LastResponse {
                    last: sighting.clone(),
                })
                .unwrap();

                if let Err(err) = stream.write_all(&response).await {
                    println!("Write failed: {}", &err);
                    continue;
                }
                if let Err(err) = stream.write_all(&serde_json::to_vec(&'\n').unwrap()).await {
                    println!("Write failed: {}", &err);
                    continue;
                }
            }
            Ok(Message::ImageRequest { uuid }) => {
                println!("{}", uuid);
                let filename = {
                    let sightings = sightings.lock().unwrap();
                    let sighting = sightings
                        .iter()
                        .filter(|sighting| sighting.uuid == uuid)
                        .last()
                        .cloned();
                    let sighting = sighting.unwrap_or_default();
                    format!("{}_{}.jpg", sighting.species, sighting.uuid)
                };
                let buf = match image::open(format!("sightings/{}", filename)) {
                    Ok(base_img) => {
                        let base_img = base_img.resize(24, 16, FilterType::Gaussian);
                        let mut buf = vec![];
                        base_img
                            .write_to(&mut buf, image::ImageOutputFormat::Jpeg(60))
                            .unwrap();
                        buf
                    }
                    Err(err) => {
                        println!("{:?}", err);
                        vec![]
                    }
                };
                let base64_img = format!("data:image/jpeg;{}", base64::encode(&buf));
                let response = serde_json::to_vec(&Message::ImageResponse {
                    base64: base64_img.clone(),
                })
                .unwrap();
                println!("{}", base64_img);

                if let Err(err) = stream.write_all(&response).await {
                    println!("Write failed: {}", &err);
                    continue;
                }
                if let Err(err) = stream.write_all(&serde_json::to_vec(&'\n').unwrap()).await {
                    println!("Write failed: {}", &err);
                    continue;
                }
            }
            _ => {
                let text = std::str::from_utf8(buf).unwrap();
                println!("Echoing {} bytes: {}", buf.len(), text);
                if let Err(err) = stream.write_all(buf).await {
                    println!("Write failed: {}", &err);
                    continue;
                }
            }
        }
    }

    println!("{} disconnected", &addr);
    Ok(())
}

pub async fn run(sightings: Arc<Mutex<Vec<Sighting>>>) -> bluer::Result<()> {
    let session = bluer::Session::new().await?;
    let adapter_names = session.adapter_names().await?;
    let adapter_name = adapter_names.first().expect("No Bluetooth adapter present");
    let adapter = session.adapter(adapter_name)?;
    adapter.set_powered(true).await?;
    adapter.set_discoverable(true).await?;
    adapter.set_discoverable_timeout(0).await?;
    adapter.set_pairable(false).await?;

    let agent = Agent::default();
    let _agent_hndl = session.register_agent(agent).await?;

    let profile = Profile {
        uuid: SERVICE_UUID,
        name: Some("ornithology-pi".to_string()),
        channel: Some(CHANNEL.into()),
        role: Some(Role::Server),
        require_authentication: Some(false),
        require_authorization: Some(false),
        auto_connect: Some(true),
        ..Default::default()
    };

    let le_advertisement = Advertisement {
        service_uuids: vec![SERVICE_UUID].into_iter().collect(),
        discoverable: Some(true),
        local_name: Some("ornithology-pi".to_string()),
        ..Default::default()
    };
    let _adv_handle = adapter.advertise(le_advertisement).await?;

    eprintln!("Registered profile");

    println!(
        "Advertising on Bluetooth adapter {} with address {} and service {}",
        &adapter_name,
        adapter.address().await?,
        profile.uuid
    );
    let mut hndl = session.register_profile(profile).await?;

    println!("Listening on channel {}", CHANNEL);

    loop {
        println!("\nWaiting for connection...");
        let req = hndl.next().await.expect("received no connect request");
        let sa = req.device();
        let mut stream = match req.accept() {
            Ok(v) => v,
            Err(err) => {
                println!("Accepting connection failed: {}", &err);
                continue;
            }
        };
        let recv_mtu = MTU;

        println!(
            "Accepted connection from {:?} with receive MTU {} bytes",
            &sa, &recv_mtu
        );
        match handle_connection(sightings.clone(), &mut stream, sa).await {
            Err(err) => println!("{:?}", err),
            _ => (),
        }
    }

    println!("Removing advertisement");
    drop(hndl);
    drop(_adv_handle);
    drop(_agent_hndl);
    sleep(Duration::from_secs(1)).await;
    Ok(())
}
