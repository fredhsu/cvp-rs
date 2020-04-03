extern crate reqwest;
extern crate serde;
//use serde::{Deserialize, Serialize};
extern crate serde_json;

pub mod cloudvision {
    use serde::{Deserialize, Serialize};
    use reqwest;

    #[derive(Debug)]
    pub struct Client {
        pub base_url: String,
        pub client: reqwest::blocking::Client,
        pub user: User,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct User {
        #[serde(rename="userId")]
        pub username: String,
        pub password: String,
    }

    impl User {
        pub fn new(username: &str, password: &str) -> User {
            User{username: String::from(username), password: String::from(password)}
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct AuthResponse {
        #[serde(rename="userName")]
        user_name: String,
        #[serde(rename="sessionId")]
        session_id: String,
    }
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Device {
        #[serde(rename="modelName")]
        model_name:            Option<String>,
        #[serde(rename="internalVersion")]
        internal_version:      Option<String>,
        #[serde(rename="systemMacAddress")]
        system_mac_address:     Option<String>,
        #[serde(rename="memTotal")]
        mem_total:             Option<u64>,
        #[serde(rename="memFree")]
        mem_free:              Option<u64>,
        #[serde(rename="bootupTimestamp")]
        bootup_timestamp:      Option<f64>,
        version:              Option<String>,
        architecture:         Option<String>,
        #[serde(rename="internalBuild")]
        internal_build:        Option<String>,
        #[serde(rename="hardwareRevision")]
        hardware_revision:     Option<String>,
        #[serde(rename="domainName")]
        domain_name:          Option<String>,
        hostname:            Option<String>,
        fqdn:                Option<String>,
        #[serde(rename="serialNumber")]
        serial_number:        Option<String>,
        #[serde(rename="danzEnabled")]
        danz_enabled:         Option<bool>,
        #[serde(rename="mlagEnabled")]
        mlag_enabled: Option<bool>,
        #[serde(rename="parentContainerKey")]
        parent_container_key:  Option<String>,
        status:              Option<String>,
        #[serde(rename="complianceCode")]
        compliance_code:      Option<String>,
        #[serde(rename="complianceIndiciation")]
        compliance_indication:  Option<String>,
        #[serde(rename="ztpMode")]
        ztp_mode:              Option<bool>,
        unauthorized:         Option<bool>,
        #[serde(rename="ipAddress")]
        ip_address:            Option<String>,
    }

   
    impl Client {
        pub fn new(base_url: &str, user: User) -> Client {
            let client = reqwest::blocking::Client::builder()
            .cookie_store(true)
            .danger_accept_invalid_certs(true)
            .build()
            .unwrap();
            Client {client, base_url: base_url.to_string(), user}
        }
        pub fn authenticate(self) -> Result<Client, reqwest::Error> {
            println!("authenticating", );
            let auth_url = "/login/authenticate.do";
            let url = [self.base_url.clone(), String::from(auth_url)].concat();

            let body = serde_json::json!(self.user);
            
            let resp = self.client
            .post(url.as_str())
            .json(&body)
            .send()?;
            println!("{:?}", resp);
            Ok(self)
        }
        pub fn get(self, svc_url: &str) -> Result<reqwest::blocking::Response, reqwest::Error>{
            let url = [self.base_url.clone(), String::from(svc_url)].concat();
            self.client
            .get(url.as_str())
            .send()
        }
        pub fn get_inventory(self, provisioned: bool) -> Result<Vec<Device>, reqwest::Error> {
            let mut svc_url: String = "/inventory/devices".to_string();
            if provisioned {
                svc_url.push_str("?provisoined=true");
            }
            let resp = self.get(&svc_url)?;
            println!("{:?}", resp);
            resp.json()
        }
    
    }
}
#[cfg(test)]
mod tests {
    use super::cloudvision;
    #[test]
    fn test_device_list() {
        let user = cloudvision::User::new("cvpadmin", "arista");
        let client = cloudvision::Client::new("https://10.90.224.175/cvpservice", user);
        let auth_client = match client.authenticate() {
                Ok(c) => c,
                Err(error) => {
                    panic!("{}", error)
                },
            };
        let devices = auth_client.client.get("https://10.90.224.175/cvpservice/inventory/devices").send();
        let device_list = match devices {
            Ok(result) => result.text(),
            Err(error) => {
                Err(error)
            }
        };
        match device_list {
            Ok(body) => println!("{:?}", body),
            Err(error) => {
                println!("{:?}", error);
                assert!(false)
            }
        }
        assert!(true);
    }
    #[test]
    fn test_get_inventory() {
        let user = cloudvision::User::new("cvpadmin", "arista");
        let client = cloudvision::Client::new("https://10.90.224.175/cvpservice", user);
        let auth_client = match client.authenticate() {
                Ok(c) => c,
                Err(error) => {
                    panic!("{}", error)
                },
            };
        let inventory = auth_client.get_inventory(false);
        let device_list = match inventory {
            Ok(result) => result,
            Err(error) => {
                println!("{:?}", error);
                panic!(error)
            }
        };
        assert_eq!(device_list.len(), 31);
    }
}
