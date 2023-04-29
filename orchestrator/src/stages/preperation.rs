use std::{marker::PhantomData, time::SystemTime};

use anyhow::anyhow;
use crossbeam::channel::{Receiver, Sender};

use crate::{
    config::Config,
    htp_test::{HtpTest, Prepared, Queued, Runnable, Terminated, Validated},
};

pub struct Preparer {
    input: Receiver<HtpTest<Validated>>,
    output: Sender<HtpTest<Prepared>>,
    output_terminated: Sender<HtpTest<Terminated>>,
}
impl Preparer {
    pub fn new(
        input: Receiver<HtpTest<Validated>>,
        output: Sender<HtpTest<Prepared>>,
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
        let Ok(mut to_prepare) = self.input.try_recv() else {
            return Ok(());
        };

        to_prepare.stats_sink.write("preperation", "started");
        let prepared = to_prepare.prepare();
        match prepared {
            Ok(mut prepared) => {
                prepared
                    .stats_sink
                    .write("preperation", "finished successfully");

                self.output.send(prepared)?
            }
            Err(mut prepare_error) => {
                prepare_error
                    .terminated
                    .stats_sink
                    .write("preperation", "failed");
                println!("Err: {}", prepare_error.msg);
                self.output_terminated.send(prepare_error.terminated)?
            }
        };
        println!("Prepared");
        Ok(())
    }
    pub fn close(&mut self) -> anyhow::Result<()> {
        println!("Closing");
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Preperation error: {msg}")]
pub struct PreperationError {
    msg: String,
    #[source]
    source: anyhow::Error,
    terminated: HtpTest<Terminated>,
}

impl HtpTest<Validated> {
    pub fn prepare(self) -> Result<HtpTest<Prepared>, PreperationError> {
        // TODO
        let ok: anyhow::Result<()> = Ok(());

        match ok {
            Ok(()) => Ok(self.clone_into()),
            Err(err) => Err(PreperationError {
                msg: "Config creation failed".into(),
                source: err,
                terminated: self.clone_into(),
            }),
        }
    }
}
