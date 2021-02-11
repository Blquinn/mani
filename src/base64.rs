pub mod base64 {
    extern crate base64;
    use serde::{Serializer, de, Deserialize, Deserializer};

    pub fn serialize<S>(bytes: &[u8], serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_str(&base64::encode(bytes))

        // Could also use a wrapper type with a Display implementation to avoid
        // allocating the String.
        //
        // serializer.collect_str(&Base64(bytes))
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Vec<u8>, D::Error>
        where D: Deserializer<'de>
    {
        let s = <&str>::deserialize(deserializer)?;
        base64::decode(s).map_err(de::Error::custom)
    }
} 

/*
pub mod base64_opt {
    extern crate base64;
    use serde::{Serializer, de, Deserialize, Deserializer};

    pub fn serialize<S>(bytes: &Option<Vec<u8>>, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        if let Some(b) = bytes {
            serializer.serialize_str(&base64::encode(b))
        } else {
            serializer.serialize_none()
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<Vec<u8>>, D::Error>
        where D: Deserializer<'de>
    {
        let s = <Option<&str>>::deserialize(deserializer)?;
        if let Some(ss) = s {
            base64::decode(ss).map_err(de::Error::custom).map(|b| { Some(b) })
        } else {
            Ok(None)
        }
    }
}
*/