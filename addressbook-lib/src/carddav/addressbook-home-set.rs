use serde::Deserialize;
use tracing::{debug, trace};

use crate::{
    http::{Request, SendHttpRequest},
    tcp,
};

use super::{
    client::Authentication,
    response::{HrefProp, Multistatus},
    Config,
};

#[derive(Debug)]
pub struct AddressbookHomeSet {
    http: SendHttpRequest,
}

impl AddressbookHomeSet {
    const BODY: &'static str = include_str!("./addressbook-home-set.xml");

    pub fn new(config: &Config, uri: impl AsRef<str>) -> Self {
        let mut request =
            Request::propfind(uri.as_ref(), config.http_version.as_ref()).content_type_xml();

        if let Authentication::Basic(user, pass) = &config.authentication {
            request = request.basic_auth(user, pass);
        };

        Self {
            http: SendHttpRequest::new(request.body(Self::BODY)),
        }
    }

    pub fn output(self) -> Result<Option<String>, quick_xml::de::DeError> {
        let body = self.http.take_body();

        let response: Response = quick_xml::de::from_reader(body.as_slice())?;

        let Some(responses) = response.responses else {
            return Ok(None);
        };

        for response in responses {
            trace!(?response, "process multistatus");

            if let Some(status) = response.status {
                if !status.is_success() {
                    debug!(?status, "multistatus response error");
                    continue;
                }
            };

            let Some(propstats) = response.propstats else {
                continue;
            };

            for propstat in propstats {
                if !propstat.status.is_success() {
                    debug!(?propstat.status, "multistatus propstat response error");
                    continue;
                }

                return Ok(Some(propstat.prop.addressbook_home_set.href.value));
            }
        }

        Ok(None)
    }
}

pub type Response = Multistatus<Prop>;

#[derive(Clone, Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct Prop {
    pub addressbook_home_set: HrefProp,
}

impl AsMut<tcp::State> for AddressbookHomeSet {
    fn as_mut(&mut self) -> &mut tcp::State {
        self.http.as_mut()
    }
}

impl Iterator for AddressbookHomeSet {
    type Item = tcp::Io;

    fn next(&mut self) -> Option<Self::Item> {
        self.http.next()
    }
}
