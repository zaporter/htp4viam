use std::{marker::PhantomData, time::SystemTime};

use crossbeam::channel::{Receiver, Sender};

use crate::{
    config::Config,
    htp_test::{HtpTest, Queued, Terminated, Validated},
};

pub struct Validator {
    input: Receiver<HtpTest<Queued>>,
    output: Sender<HtpTest<Validated>>,
    output_terminated: Sender<HtpTest<Terminated>>,
}
impl Validator {
    pub fn new(
        input: Receiver<HtpTest<Queued>>,
        output: Sender<HtpTest<Validated>>,
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
        let Ok(mut to_validate) = self.input.try_recv() else {
            return Ok(());
        };

        to_validate.stats_sink.write("validation", "started");
        let validated = to_validate.validate();
        //TODO(is_ok) is wrong
        match validated {
            Ok(mut validated) => {
                validated
                    .stats_sink
                    .write("validation", "finished successfully");

                self.output.send(validated)?
            }
            Err(mut validate_error) => {
                validate_error
                    .terminated
                    .stats_sink
                    .write("validation", "failed");
                println!("Err: {}", validate_error.msg);
                self.output_terminated.send(validate_error.terminated)?
            }
        };
        println!("Validated");
        Ok(())
    }
    pub fn close(&mut self) -> anyhow::Result<()> {
        println!("Closing");
        Ok(())
    }
}

#[derive(thiserror::Error, Debug)]
#[error("Validation error: {msg}")]
pub struct ValidateError {
    msg: String,
    #[source]
    source: anyhow::Error,
    terminated: HtpTest<Terminated>,
}

impl HtpTest<Queued> {
    pub fn validate(mut self) -> Result<HtpTest<Validated>, ValidateError> {
        // TODO
        // Actually start it...
        let config = Config::new(self.config_folder.get_path());
        match config {
            Ok(config) => {
                self.config = Some(config);
                Ok(self.clone_into())
            }
            Err(err) => Err(ValidateError {
                msg: "Config creation failed".into(),
                source: err,
                terminated: self.clone_into(),
            }),
        }
    }
}
