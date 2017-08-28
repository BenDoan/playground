#[macro_use] extern crate nickel;
extern crate postgres;
extern crate time;
extern crate rustc_serialize;

use nickel::{Nickel, JsonBody, FormBody};
use nickel::status::StatusCode;
use postgres::{Connection, SslMode};
use std::collections::HashMap;
use rustc_serialize::json::{Json, ToJson};
use std::collections::BTreeMap;

#[derive(RustcDecodable, RustcEncodable)]
struct BatteryData {
    machine_id: String
}

//impl ToJson for BatteryData {
    //fn to_json(&self) -> Json {
        //let mut map = BTreeMap::new();
        //map.insert("machine_id".to_string(), self.machine_id.to_json());
        //Json::Object(map)
    //}
//}

fn init_routing(server: &mut nickel::Nickel){
    server.utilize(router! {
        get "/" => |request, response| {
            "Hello world!"
        }
        post "/insert/:table_name" => |req, res| {
            //let bdata = try_with!(response, {
                //request.json_as::<HashMap>().map_err(|e| (StatusCode::BadRequest, e))
            //});
            //let form_body = try_with!(res, req.form_body());
            let form_body = req.form_body().unwrap();
            //let table_name = req.param("table_name").unwrap();
            //table_name
            println!("{:?}", form_body.get("hello"));
            format!("{:?}", form_body)
        }
    });
}

fn start_server(){
    let mut server = Nickel::new();
    init_routing(&mut server);
    server.listen("127.0.0.1:6767");
}

fn init_database(){
    let conn = Connection::connect("postgres://livia:postgrespass@localhost/livia", SslMode::None).unwrap();

    conn.execute("CREATE EXTENSION IF NOT EXISTS hstore", &[]).unwrap();
    conn.execute("CREATE TABLE IF NOT EXISTS table_metadata (
                id              SERIAL PRIMARY KEY,
                name            TEXT UNIQUE NOT NULL,
                creation_time   TIMESTAMP NOT NULL,
                battery_mapping HSTORE NOT NULL
            )", &[]).unwrap();

    let mut battery_mapping = HashMap::new();
    battery_mapping.insert("machine_id".to_string(), Some("i32".to_string()));
    battery_mapping.insert("time".to_string(), Some("Timespec".to_string()));
    battery_mapping.insert("percent".to_string(), Some("i32".to_string()));

    let name = "battery_data";
    let now = time::get_time();

    conn.execute("INSERT INTO table_metadata
                 (name, creation_time, battery_mapping)
                 VALUES ($1, $2, $3)
                 ON CONFLICT DO NOTHING",
                 &[&name, &now, &battery_mapping]).unwrap();

    conn.execute("CREATE TABLE IF NOT EXISTS battery_data (
                id              SERIAL PRIMARY KEY,
                machine_id      INT NOT NULL,
                time            TIMESTAMP NOT NULL,
                percent         INT NOT NULL
            )", &[]).unwrap();
}

fn main() {
    init_database();
    start_server();
}
