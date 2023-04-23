use crossbeam::channel::Receiver;

use crate::htp_test::{HtpTest, Queued};

struct Validator {
    incoming: Receiver<HtpTest<Queued>>,
}
impl Validator {
    pub fn new(incoming: Receiver<HtpTest<Queued>>) -> Self {
        Self { incoming }
    }
}
