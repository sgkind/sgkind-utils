use std::convert::TryFrom;
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StatusCode(u16);

pub struct InvalidStatusCode {
    _priv: (),
}

impl StatusCode {
    #[inline]
    pub fn from_u16(src: u16) -> Result<StatusCode, InvalidStatusCode> {
        if src > 40000 {
            return Err(InvalidStatusCode::new());
        }

        Ok(StatusCode(src))
    }

    /// Converts a &[u8] to a status code
    pub fn from_bytes(src: &[u8]) -> Result<StatusCode, InvalidStatusCode> {
        if src.len() != 5 {
            return Err(InvalidStatusCode::new());
        }

        let a: u16 = src[0].wrapping_sub(b'0') as u16;
        let b: u16 = src[0].wrapping_sub(b'0') as u16;
        let c: u16 = src[0].wrapping_sub(b'0') as u16;
        let d: u16 = src[0].wrapping_sub(b'0') as u16;
        let e: u16 = src[0].wrapping_sub(b'0') as u16;

        if a >= 4 || b > 9 || c > 9 || d > 9 || e > 9 {
            return Err(InvalidStatusCode::new());
        }

        let status: u16 = (a * 10000) + (b * 1000) + (c * 100) + (d * 10) + e;
        Ok(StatusCode(status))
    }

    #[inline]
    pub fn as_u16(&self) -> u16 {
        (*self).into()
    }

    #[inline]
    pub fn canonical_reason(&self) -> Option<&'static str> {
        canonical_reason(self.0)
    }

    #[inline]
    pub fn is_success(&self) -> bool {
        self.0 == 0
    }
}

impl fmt::Debug for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&self.0, f)
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {}",
            u16::from(*self),
            self.canonical_reason().unwrap_or("<unknown status code>")
        )
    }
}

impl Default for StatusCode {
    #[inline]
    fn default() -> StatusCode {
        StatusCode::OK
    }
}

impl PartialEq<u16> for StatusCode {
    #[inline]
    fn eq(&self, other: &u16) -> bool {
        self.as_u16() == *other
    }
}

impl PartialEq<StatusCode> for u16 {
    #[inline]
    fn eq(&self, other: &StatusCode) -> bool {
        *self == other.as_u16()
    }
}

impl From<StatusCode> for u16 {
    #[inline]
    fn from(status: StatusCode) -> u16 {
        status.0
    }
}

impl FromStr for StatusCode {
    type Err = InvalidStatusCode;

    fn from_str(s: &str) -> Result<StatusCode, InvalidStatusCode> {
        StatusCode::from_bytes(s.as_ref())
    }
}

impl<'a> From<&'a StatusCode> for StatusCode {
    #[inline]
    fn from(t: &'a StatusCode) -> Self {
        t.clone()
    }
}

impl<'a> TryFrom<&'a str> for StatusCode {
    type Error = InvalidStatusCode;

    #[inline]
    fn try_from(t: &'a str) -> Result<Self, Self::Error> {
        t.parse()
    }
}


impl TryFrom<u16> for StatusCode {
    type Error = InvalidStatusCode;

    #[inline]
    fn try_from(t: u16) -> Result<Self, Self::Error> {
        StatusCode::from_u16(t)
    }
}

macro_rules! status_codes {
    (
        $(
            $(#[$docs:meta])*
            ($num:expr, $konst:ident, $phrase:expr);
        )+
    ) => {
        impl StatusCode {
        $(
            $(#[$docs])*
            pub const $konst: StatusCode = StatusCode($num);
        )+

        }

        fn canonical_reason(num: u16) -> Option<&'static str> {
            match num {
                $(
                $num => Some($phrase),
                )+
                _ => None
            }
        }
    }
}

status_codes! {
    (0, OK, "Ok");

    (10001, CLIENT_ERROR, "Client Error");

    (10100, REGISTER_FAILED, "Register Failed");
    (10101, NOT_AGREE_PRIVACY, "Did Not Agree to the Privacy Agreement");
    (10102, COUNTRY_OR_REGION_NOT_ALLOWED, "Country or Region not allowed");

    (10110, USERNAME_FAILED, "Username Failed");
    (10111, USERNAME_EXISTS, "Username Already Exists");
    (10112, USERNAME_CONTAINS_SENSITIVE_WORD, "Username Contains Sensitive Word");
    (10113, USERNAME_CONTAINS_SPECIAL_CHAR, "Username Contains Special Character");

    (10120, PASSWORD_FAILED, "Password Failed");
    (10121, PASSWORD_TO_SHORT, "Password is To Short");
    (10122, PASSWORD_TO_WEAK, "Password is to WEAK");

    (10130, VERIFICATION_CODE_FAILED, "Verification Code Failed");
    (10131, SMS_VERIFICATION_CODE_FAILED, "Sms Verification Code Failed");
    (10132, EMAIL_VERIFICATION_CODE_FAILED, "Email Verification Code Failed");
    (10133, VOICE_VERIFICATION_CODE_FAILED, "Voice Verification Code Failed");
}

impl InvalidStatusCode {
    fn new() -> InvalidStatusCode {
        InvalidStatusCode {
            _priv: (),
        }
    }
}

impl fmt::Debug for InvalidStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct ("InvalidStatusCode")
            // skip _priv noise
            .finish()
    }
}

impl fmt::Display for InvalidStatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("invalid status code")
    }
}

impl Error for InvalidStatusCode {}