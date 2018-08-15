extern crate rumqtt;

use self::rumqtt::{MqttClient, MqttOptions, QoS};
use actix_web::{HttpResponse, Json, Responder};
use std::env;

#[derive(Debug, Deserialize)]
pub struct Request {
    topic: String,
    payload: String,
}

impl Request {

    pub fn get_topic(&self) -> &String {
        &self.topic
    }

    pub fn get_payload(&self) -> &String {
        &self.payload
    }

}

pub fn publish(request: Json<Request>) -> impl Responder {
    println!("{:?}", request);

    let mut mq_client = get_mqtt_client();

    mq_client
        .publish(
            request.get_topic().as_str(),
            QoS::Level1,
            request.get_payload().clone().into_bytes(),
        )
        .expect("Publish failure");

    HttpResponse::Ok()
}

fn get_mqtt_client() -> MqttClient {
    let client_options = MqttOptions::new()
        .set_keep_alive(5)
        .set_reconnect(3)
        .set_client_id("wormhole")
        .set_broker(env::var("MQTT_SERVER").unwrap().as_str());

    MqttClient::start(client_options, None).unwrap()
}
