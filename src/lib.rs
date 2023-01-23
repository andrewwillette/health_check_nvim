use nvim_oxi::{self as oxi, lua, print, Dictionary, Function, Object};
use oxi::conversion::{self, FromObject, ToObject};
use oxi::serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};
use std::error::Error;
use url::Url;

#[derive(Serialize, Deserialize)]
struct ServiceEndpoint {
    url: String,
    expected_response_code: usize,
}

impl FromObject for ServiceEndpoint {
    fn from_object(obj: Object) -> Result<Self, conversion::Error> {
        Self::deserialize(Deserializer::new(obj)).map_err(Into::into)
    }
}

impl ToObject for ServiceEndpoint {
    fn to_object(self) -> Result<Object, conversion::Error> {
        self.serialize(Serializer::new()).map_err(Into::into)
    }
}

impl lua::Poppable for ServiceEndpoint {
    unsafe fn pop(lstate: *mut lua::ffi::lua_State) -> Result<Self, lua::Error> {
        let obj = Object::pop(lstate)?;
        Self::from_object(obj).map_err(lua::Error::pop_error_from_err::<Self, _>)
    }
}

impl lua::Pushable for ServiceEndpoint {
    unsafe fn push(self, lstate: *mut lua::ffi::lua_State) -> Result<std::ffi::c_int, lua::Error> {
        self.to_object()
            .map_err(lua::Error::push_error_from_err::<Self, _>)?
            .push(lstate)
    }
}

#[oxi::module]
fn health_check_nvim() -> oxi::Result<Dictionary> {
    Ok(Dictionary::from_iter([(
        "health_check",
        Function::from_fn(health_check),
    )]))
}

/// get health of endpoints
fn health_check(service_endpoint: Vec<ServiceEndpoint>) -> oxi::Result<bool> {
    for endpoint in service_endpoint {
        let (is_healthy, status_code) =
            match check_health(&endpoint.url, endpoint.expected_response_code as u16) {
                Ok((healthy, code)) => (healthy, code),
                Err(e) => {
                    print!("error: {}", e);
                    (false, 0)
                }
            };
        print!(
            "{}: healthy {}: expected {}: actual {}",
            endpoint.url, is_healthy, endpoint.expected_response_code, status_code
        );
    }
    Ok(true)
}

fn check_health(url: &String, status_code: u16) -> Result<(bool, u16), Box<dyn Error>> {
    let resp = reqwest::blocking::get(Url::parse(&url)?)?;
    let status_as_int = resp.status().as_u16();
    if status_as_int == status_code {
        return Ok((true, status_as_int));
    } else {
        return Ok((false, status_as_int));
    }
}

#[test]
fn test_check_health() {
    let (healthy, status_code) = check_health(&"http://www.google.com".to_string(), 200).unwrap();
    assert_eq!(healthy, true);
    assert_eq!(status_code, 200);
}
