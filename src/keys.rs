const NUM_KEYS: usize = 9;

#[derive(Clone, Copy, Debug)]
pub enum Modified {
    Yes(Key),
    No(Key),
}

#[derive(Clone, Copy, Debug)]
pub struct Key {
    pub modifier: u8,
    pub keycodes: [u8; 6],
}
#[derive(Debug)]
pub struct KeypadConfig {
    pub keys: [Modified; NUM_KEYS],
}
impl KeypadConfig {
    pub fn serialize_key(key: &Key) -> [u8; 7] {
        let mut ret = [0; 7];

        ret[0] = key.modifier;
        ret[1..7].copy_from_slice(&key.keycodes[..]);

        ret
    }

    pub fn deserialize_key(buf: &[u8]) -> Option<Key> {
        if buf.len() != 7 {
            return None;
        }
        let modifier = buf[0];
        let mut keycodes = [0u8; 6];
        keycodes.copy_from_slice(&buf[1..]);

        Some(Key { modifier, keycodes })
    }

    pub fn serialize(&self, buf: &mut [u8]) -> Result<(), ()> {
        if buf.len() != 63 {
            return Err(());
        }

        for i in 0..NUM_KEYS {
            let index = i * 7;
            let key = match &self.keys[i] {
                Modified::Yes(k) => k,
                Modified::No(k) => k,
            };
            buf[index..(index + 7)].copy_from_slice(&KeypadConfig::serialize_key(key))
        }

        Ok(())
    }

    #[allow(clippy::needless_range_loop)]
    pub fn deserialize(buf: &[u8]) -> Option<KeypadConfig> {
        if buf.len() != 63 {
            return None;
        }

        let mut keys = [Modified::No(Key {
            modifier: 0,
            keycodes: [0; 6],
        }); NUM_KEYS];

        for i in 0..NUM_KEYS {
            let index = i * 7;
            let key = KeypadConfig::deserialize_key(&buf[index..(index + 7)])?;

            keys[i] = Modified::No(key);
        }

        Some(KeypadConfig { keys })
    }
}
