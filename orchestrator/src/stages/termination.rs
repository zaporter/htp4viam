use crossbeam::channel::Receiver;

use crate::htp_test::{HtpTest, Terminated};

pub struct TerminatedSink {
    input: Receiver<HtpTest<Terminated>>,
}
impl TerminatedSink {
    pub fn new(input: Receiver<HtpTest<Terminated>>) -> Self {
        Self { input }
    }
    pub fn desired_poll_delay(&mut self) -> tokio::time::Duration {
        tokio::time::Duration::from_millis(100)
    }
    pub fn process_one(&mut self) -> anyhow::Result<()> {
        let Ok(to_process) = self.input.try_recv() else {
            return Ok(());
        };
        println!("{:?} was terminated", to_process);

        {
            let mut map = to_process.test_map.lock().unwrap();
            map.map.retain(|p| p.id != to_process.test_spec_id);
        }

        Ok(())
    }
    pub fn close(&mut self) -> anyhow::Result<()> {
        println!("Closing");
        Ok(())
    }
}
