use std::{path::PathBuf, sync::mpsc::Receiver};

use crossbeam::channel::Sender;
use tokio::{task::JoinHandle, runtime::Runtime};

use crate::{stages::{validation::Validator, termination::TerminatedSink, aquiring::Aquirer, running::Runner}, htp_test::{HtpTest, Queued,  PRIORITY_ADMIN, Validated}, config::Config, rcfolder::RcFolder};

pub struct Orchestrator {
    // validator: Validator,
    main_input : Sender<HtpTest<Queued>>,
    validator_handle: JoinHandle<anyhow::Result<()>>,
    aquirer_handle: JoinHandle<anyhow::Result<()>>,
    runner_handle: JoinHandle<anyhow::Result<()>>,
    terminated_sink_handle: JoinHandle<anyhow::Result<()>>,
    runtime: Runtime,
    close_sender: Sender<()>,
}

impl Orchestrator {
    pub fn new() -> Self {
        let (main_input, valid_receiver) = crossbeam::channel::unbounded();
        let (valid_sender, aquire_receiver) = crossbeam::channel::unbounded();
        let (aquire_sender, run_receiver) = crossbeam::channel::unbounded();

        let (terminated_sender, terminated_receiver) = crossbeam::channel::unbounded();
        let (close_sender, close_receiver) = crossbeam::channel::unbounded();

        let mut validator= Validator::new(valid_receiver, valid_sender.clone(), terminated_sender.clone());
        let mut aquirer = Aquirer::new(aquire_receiver, aquire_sender.clone(), terminated_sender.clone());
        let mut runner = Runner::new(run_receiver, terminated_sender.clone(), terminated_sender);
        let mut terminated_sink = TerminatedSink::new(terminated_receiver);

        let runtime = Runtime::new().unwrap();

        let handle = runtime.handle();

        let close_receiver_inst = close_receiver.clone();
        let validator_handle = handle.spawn( async move {
            loop {
                if close_receiver_inst.try_recv().is_ok() {
                    return validator.close();
                }
                tokio::time::sleep(validator.desired_poll_delay()).await;
                validator.process_one()?;
            }
        });
        let close_receiver_inst = close_receiver.clone();
        let aquirer_handle = handle.spawn( async move {
            loop {
                if close_receiver_inst.try_recv().is_ok() {
                    return aquirer.close();
                }
                tokio::time::sleep(aquirer.desired_poll_delay()).await;
                aquirer.process_one()?;
            }
        });
        let close_receiver_inst = close_receiver.clone();
        let runner_handle = handle.spawn( async move {
            loop {
                if close_receiver_inst.try_recv().is_ok() {
                    return runner.close();
                }
                tokio::time::sleep(runner.desired_poll_delay()).await;
                runner.process_one()?;
            }
        });
        let close_receiver_inst = close_receiver.clone();
        let terminated_sink_handle = handle.spawn( async move {
            loop {
                if close_receiver_inst.try_recv().is_ok() {
                    return terminated_sink.close();
                }
                tokio::time::sleep(terminated_sink.desired_poll_delay()).await;
                terminated_sink.process_one()?;
            }
        });
        Self {
            main_input,
            runtime,
            validator_handle ,
            aquirer_handle,
            runner_handle,
            terminated_sink_handle,
            close_sender
        }
    }
    pub fn start(&mut self) -> anyhow::Result<()> {
        let config_path = PathBuf::from("../config");
        let test_spec_id = ("general".into(),"simpleconn".into());
        let priority = PRIORITY_ADMIN;
        let test = HtpTest::<Queued>::new(RcFolder::new(config_path), test_spec_id, priority);
        self.main_input.send(test?)?;
        Ok(())
    }
    pub fn stop( self) -> anyhow::Result<()> {
        self.close_sender.send(())?;
        self.close_sender.send(())?;
        self.close_sender.send(())?;
        self.close_sender.send(())?;
        self.runtime.block_on(async {self.validator_handle.await?})?;
        self.runtime.block_on(async {self.aquirer_handle.await?})?;
        self.runtime.block_on(async {self.runner_handle.await?})?;
        self.runtime.block_on(async {self.terminated_sink_handle.await?})?;
        Ok(())
    }
}
