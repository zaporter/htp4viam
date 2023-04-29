use std::{marker::PhantomData, process::Command, time::SystemTime};

use anyhow::anyhow;
use crossbeam::channel::{Receiver, Sender};

use crate::{
    config::Config,
    htp_test::{HtpTest, Queued, Runnable, Terminated, Validated},
};

pub struct Runner {
    input: Receiver<HtpTest<Runnable>>,
    output: Sender<HtpTest<Terminated>>,
    output_terminated: Sender<HtpTest<Terminated>>,
}
impl Runner {
    pub fn new(
        input: Receiver<HtpTest<Runnable>>,
        output: Sender<HtpTest<Terminated>>,
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
        let Ok(mut to_run) = self.input.try_recv() else {
            return Ok(());
        };

        to_run.stats_sink.write("running", "started");
        let rund = to_run.run();
        match rund {
            Ok(mut rund) => {
                rund.stats_sink.write("running", "finished successfully");

                self.output.send(rund)?
            }
            Err(mut run_error) => {
                run_error.terminated.stats_sink.write("running", "failed");
                println!("Err: {}", run_error.msg);
                self.output_terminated.send(run_error.terminated)?
            }
        };
        println!("Ran");
        Ok(())
    }
    pub fn close(&mut self) -> anyhow::Result<()> {
        println!("Closing");
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Aquisition error: {msg}")]
pub struct RunningError {
    msg: String,
    #[source]
    source: anyhow::Error,
    terminated: HtpTest<Terminated>,
}

impl HtpTest<Runnable> {
    pub fn run(mut self) -> Result<HtpTest<Terminated>, RunningError> {
        // TODO
        let command = &self.get_test_spec().on_device_test_script;
        if let Some(command) = command {
            let output = Command::new("sh")
                .arg("-c")
                .arg(command)
                .output()
                .expect("failed to run cmd");
            let out: String = String::from_utf8(output.stdout).unwrap();
            self.stats_sink.write("running", out);
            return Ok(self.clone_into());
        }

        Err(RunningError {
            msg: "Running creation failed, no cmd to run".into(),
            source: anyhow!("Nope"),
            terminated: self.clone_into(),
        })
    }
}
