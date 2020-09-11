use std::env;

extern crate cidr_utils;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

#[derive(Serialize)]
struct IpV4Cidr {
    first: String,
    last: String,
    size: u64,
}

fn main() {
    let args: Vec<String> = env::args().collect();

    // args[0] has the name of binary
    let cidr_input = &args[1];

    // check if input is CIDR
    let cidr = cidr_utils::cidr::IpCidr::from_str(cidr_input);
    let cidr = match cidr {
        Ok(c) => c,
        Err(e) => panic!(
            "Failed to parse a given string! Was it really a CIDR? {:?}",
            e
        ),
    };

    let minimal_cidr = IpV4Cidr {
        first: cidr.first_as_ip_addr().to_string(),
        last: cidr.last_as_ip_addr().to_string(),
        size: cidr.size().to_str_radix(10).parse().unwrap(),
    };

    let serialized = serde_json::to_string(&minimal_cidr);
    let serialized = match serialized {
        Ok(d) => d,
        Err(e) => panic!("Failed to deserialize {:?}", e),
    };
    println!("{}", serialized);
}
