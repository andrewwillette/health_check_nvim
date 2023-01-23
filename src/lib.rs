use nvim_oxi::{self as oxi, lua, print, Dictionary, Function, Object};
use oxi::conversion::{self, FromObject, ToObject};
use oxi::serde::{Deserializer, Serializer};
use serde::{Deserialize, Serialize};

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
        print!(
            "endpoint: {}, expected_response_code: {}",
            endpoint.url, endpoint.expected_response_code
        );
    }
    Ok(true)
}
