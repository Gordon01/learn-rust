use poem::{error::BadRequest, FromRequest};
use poem_openapi::Object;

#[derive(Clone, PartialEq, Eq, Hash, Debug, Object, Default)]
pub struct HwId {
    hw_addr: [u8; 6],
    text: String,
}

impl ToString for HwId {
    fn to_string(&self) -> String {
        self.text.to_owned()
    }
}

impl TryFrom<&str> for HwId {
    type Error = Error;
    fn try_from(value: &str) -> Result<HwId, Self::Error> {
        let len = value.len();
        let mut hw_addr = [0u8; 6];

        if len == 12 {
            let mut written = 0;
            for i in 0..6 {
                if let Some(slice) = value.get(i * 2..(i + 1) * 2) {
                    if let Ok(byte) = u8::from_str_radix(slice, 16) {
                        hw_addr[written] = byte;
                        written += 1;
                    }
                }
            }
            if written == 6 {
                Ok(HwId {
                    hw_addr,
                    text: value.to_owned(),
                })
            } else {
                Err(Error::Length)
            }
        } else if len == 6 {
            let mut written = 0;
            for i in 0..3 {
                if let Some(slice) = value.get(i * 2..(i + 1) * 2) {
                    // println!("{:#?}", slice);
                    if let Ok(byte) = u8::from_str_radix(slice, 16) {
                        hw_addr[written + 3] = byte;
                        written += 1;
                    }
                }
            }
            if written == 3 {
                Ok(HwId {
                    hw_addr,
                    text: value.to_owned(),
                })
            } else {
                Err(Error::Length)
            }
        } else {
            Err(Error::Length)
        }
    }
}

impl<'a> FromRequest<'a> for HwId {
    async fn from_request(
        req: &'a poem::Request,
        _body: &mut poem::RequestBody,
    ) -> poem::Result<Self> {
        let path = req.path_params::<String>()?;
        HwId::try_from(path.as_str()).map_err(BadRequest)
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Wrong length")]
    Length,
}

#[cfg(test)]
mod test {
    // use poem::{http::Uri, Body, FromRequest, Request, RequestBody};

    use super::HwId;

    #[test]
    fn hex_str() {
        assert_eq!(0x0a, 10);
    }

    // 0a0a0a -> HwId { addr: [0xab, 0xcd, 0xef, 0x0a, 0x0a, 0x0a] }
    // 010203 -> [0xab, 0xcd, 0xef, 0x01, 0x02, 0x03]
    // 0a0b0c010203 -> [0x0a, 0x0b, 0x0c, 0x01, 0x02, 0x03]
    #[test]
    fn all_zero() {
        let id = HwId::try_from("000000").unwrap();
        assert_eq!([0; 6], id.hw_addr);
    }

    #[test]
    fn short_mac() {
        let id = HwId::try_from("0a0b0c").unwrap();
        // TODO: Replace first 3 bytes with correct prefix
        assert_eq!([0, 0, 0, 0x0a, 0x0b, 0x0c], id.hw_addr);
    }

    #[test]
    fn long_mac() {
        let id = HwId::try_from("abcdef0a0b0c").unwrap();
        // TODO: Replace first 3 bytes with correct prefix
        assert_eq!([0xab, 0xcd, 0xef, 0x0a, 0x0b, 0x0c], id.hw_addr);
    }

    #[test]
    fn bad_input() {
        let id = HwId::try_from("❤️0aro");
        assert!(id.is_err());
    }
}
