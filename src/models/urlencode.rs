pub(crate) struct UrlEncodingBuffer<'a> {
    buffer: &'a mut Vec<String>,
    prefix: String,
}

impl<'a> UrlEncodingBuffer<'a> {
    pub fn from(buffer: &'a mut Vec<String>) -> Self {
        UrlEncodingBuffer {
            buffer,
            prefix: String::new(),
        }
    }

    pub fn append(&mut self, prefix: &str) -> UrlEncodingBuffer<'_> {
        UrlEncodingBuffer {
            buffer: self.buffer,
            prefix: format!("{}{}", self.prefix, prefix),
        }
    }

    pub fn set(&mut self, key: &str, value: &str) {
        self.buffer.push(format!(
            "{}{}={}",
            urlencoding::encode(&self.prefix),
            urlencoding::encode(key),
            urlencoding::encode(value).replace("%20", "+")
        ));
    }
}

/// Used to serialize firewalls and their configurations
pub(crate) trait UrlEncode {
    fn encode_into(&self, f: UrlEncodingBuffer<'_>);
    fn encode(&self) -> String {
        let mut buffer = Vec::new();
        let encoder = UrlEncodingBuffer::from(&mut buffer);

        self.encode_into(encoder);
        buffer.join("&")
    }
}
