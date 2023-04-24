use std::{marker::PhantomData, time::SystemTime};

use anyhow::anyhow;
use crossbeam::channel::{Receiver, Sender};

use crate::{
    config::Config,
    htp_test::{HtpTest, Queued, Runnable, Terminated, Validated},
};

pub struct Aquirer {
    input: Receiver<HtpTest<Validated>>,
    output: Sender<HtpTest<Runnable>>,
    output_terminated: Sender<HtpTest<Terminated>>,
}
impl Aquirer {
    pub fn new(
        input: Receiver<HtpTest<Validated>>,
        output: Sender<HtpTest<Runnable>>,
        output_terminated: Sender<HtpTest<Terminated>>,
    ) -> Self {
        Self {
            input,
            output,
            output_terminated,
        }
    }
    // Dont put a value greater than 5sec. That would be stupid
    pub fn desired_poll_delay(&mut self) -> tokio::time::Duration {
        tokio::time::Duration::from_millis(100)
    }
    pub fn process_one(&mut self) -> anyhow::Result<()> {
        let Ok(mut to_aquire) = self.input.try_recv() else {
            return Ok(());
        };

        to_aquire.stats_sink.write("aquisition", "started");
        let aquired = to_aquire.aquire();
        match aquired {
            Ok(mut aquired) => {
                aquired
                    .stats_sink
                    .write("aquisition", "finished successfully");

                self.output.send(aquired)?
            }
            Err(mut aquire_error) => {
                aquire_error
                    .terminated
                    .stats_sink
                    .write("aquisition", "failed");
                println!("Err: {}", aquire_error.msg);
                self.output_terminated.send(aquire_error.terminated)?
            }
        };
        println!("Aquired");
        Ok(())
    }
    pub fn close(&mut self) -> anyhow::Result<()> {
        println!("Closing");
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Aquisition error: {msg}")]
pub struct AquisitionError {
    msg: String,
    #[source]
    source: anyhow::Error,
    terminated: HtpTest<Terminated>,
}

impl HtpTest<Validated> {
    pub fn aquire(self) -> Result<HtpTest<Runnable>, AquisitionError> {
        // TODO
        let ok: anyhow::Result<()> = Ok(());

        match ok {
            Ok(()) => Ok(self.clone_into()),
            Err(err) => Err(AquisitionError {
                msg: "Config creation failed".into(),
                source: err,
                terminated: self.clone_into(),
            }),
        }
    }
}
