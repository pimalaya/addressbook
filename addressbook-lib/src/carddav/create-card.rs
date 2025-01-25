use crate::{
    http::{Request, SendHttpRequest},
    tcp::{Io, Read, Write},
    Card,
};

use super::{client::Authentication, Config};

#[derive(Debug)]
pub struct CreateCard {
    card: Card,
    http: SendHttpRequest,
}

impl CreateCard {
    pub fn new(config: &Config, addressbook_id: impl AsRef<str>, card: Card) -> Self {
        let addressbook_id = addressbook_id.as_ref();
        let base_uri = config.home_uri.trim_end_matches('/');
        let uri = &format!("{base_uri}/{addressbook_id}/{}.vcf", card.id);
        let mut request = Request::put(uri, config.http_version.as_ref()).content_type_vcard();

        if let Authentication::Basic(user, pass) = &config.authentication {
            request = request.basic_auth(user, pass);
        };

        request = request.body(&card.to_string());

        Self {
            card,
            http: SendHttpRequest::new(request),
        }
    }

    pub fn output(self) -> Card {
        self.card
    }
}

impl Write for CreateCard {
    fn get_buffer(&mut self) -> &[u8] {
        self.http.get_buffer()
    }

    fn set_wrote_bytes_count(&mut self, count: usize) {
        self.http.set_wrote_bytes_count(count)
    }
}

impl Read for CreateCard {
    fn get_buffer_mut(&mut self) -> &mut [u8] {
        self.http.get_buffer_mut()
    }

    fn set_read_bytes_count(&mut self, count: usize) {
        self.http.set_read_bytes_count(count)
    }
}

impl Iterator for CreateCard {
    type Item = Io;

    fn next(&mut self) -> Option<Self::Item> {
        self.http.next()
    }
}
