pub struct Ctx<'a> {
    key: &'a [u8],
}

pub fn new_ctx(key: &[u8]) -> Result<Ctx, &'static str> {
    Ok(Ctx { key })
}

impl Ctx<'_> {
    pub fn encrypt(&self, buf: &mut [u8]) {
        buf.iter_mut()
            .zip(self.key.iter().cycle())
            .for_each(|(b, k)| {
                *b ^= *k;
            });
    }

    pub fn decrypt(&self, buf: &mut [u8]) {
        buf.iter_mut()
            .zip(self.key.iter().cycle())
            .for_each(|(b, k)| {
                *b ^= *k;
            });
    }
}
