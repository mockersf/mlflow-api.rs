use std::env;

/// MLflowClient, providing helpers methods for starting and managinf Mlflow `Run`s
#[derive(Debug)]
pub struct MLflowClient {
    active_experiment_id: Option<String>,
    active_run_id: Option<String>,
    /// API Client used to send requests directly to MLflow.
    pub api: crate::MLflowAPI,
}

impl MLflowClient {
    /// TODO: doc, name
    pub fn new() -> Result<Self, crate::errors::SetupError> {
        Self::new_with_tracking_uri(
            &env::var("MLFLOW_TRACKING_URI")
                .unwrap_or_else(|_| "Missing MLFLOW_TRACKING_URI".to_string()),
        )
    }

    /// TODO: doc, name
    pub fn new_with_tracking_uri(uri: &str) -> Result<Self, crate::errors::SetupError> {
        Ok(MLflowClient {
            active_experiment_id: None,
            active_run_id: None,
            api: crate::MLflowAPI::new(&uri)?,
        })
    }

    /// TODO: return error
    pub fn resume_run(&mut self, run_id: Option<&str>) -> Result<(), ()> {
        if let Some(run_id) = run_id {
            self.api.get_run(run_id).map_err(|_| ())?;
            self.active_run_id = Some(run_id.to_string());
        } else if let Ok(run_id) = env::var("MLFLOW_RUN_ID") {
            self.api.get_run(&run_id).map_err(|_| ())?;
            self.active_run_id = Some(run_id);
        } else {
            return Err(());
        }
        Ok(())
    }

    /// TODO
    pub fn start_run(&mut self, run_name: &str) -> Result<(), ()> {
        self.start_run_internal(None, Some(run_name)).map(|_| ())
    }

    /// TODO
    pub fn start_run_in_experiment(
        &mut self,
        experiment_id: &str,
        run_name: &str,
    ) -> Result<(), ()> {
        self.start_run_internal(Some(experiment_id), Some(run_name))
            .map(|_| ())
    }

    fn start_run_internal(
        &mut self,
        experiment_id: Option<&str>,
        run_name: Option<&str>,
    ) -> Result<crate::Run, ()> {
        if let Some(experiment_id) = experiment_id {
            self.api.get_experiment(experiment_id).map_err(|_| ())?;
            self.active_experiment_id = Some(experiment_id.to_string());
        } else if let Ok(experiment_name) = env::var("MLFLOW_EXPERIMENT_NAME") {
            self.set_experiment(&experiment_name).map_err(|_| ())?;
        } else if let Ok(experiment_id) = env::var("MLFLOW_EXPERIMENT_ID") {
            self.set_experiment(&experiment_id).map_err(|_| ())?;
        }
        if self.active_experiment_id.is_none() {
            self.set_experiment("Default").map_err(|_| ())?;
        }
        let mut tags = vec![crate::RunTag {
            key: "mlflow.user".to_string(),
            value: whoami::username(),
        }];
        if let Some(run_name) = run_name {
            tags.push(crate::RunTag {
                key: "mlflow.runName".to_string(),
                value: run_name.to_string(),
            });
        }
        if let Ok(run) = self.api.create_run(
            &self.active_experiment_id.clone().expect(""),
            Some(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("time went strange there")
                    .as_millis() as u64,
            ),
            Some(tags),
        ) {
            self.active_run_id = Some(run.info.run_id.clone());
            Ok(run)
        } else {
            Err(())
        }
        // .map_err(|_| ())
    }

    /// Set given experiment as active experiment. If experiment does not exist, create an experiment with provided
    /// name.
    pub fn set_experiment(
        &mut self,
        experiment_name: &str,
    ) -> Result<String, crate::errors::ClientError<crate::errors::CreateExperimentErrorCode>> {
        if let Ok(found) = self.api.get_experiment_by_name(experiment_name) {
            self.active_experiment_id = Some(found.experiment_id.clone());
            Ok(found.experiment_id)
        } else {
            self.create_experiment(experiment_name, None)
        }
    }

    /// Create an experiment.
    pub fn create_experiment(
        &mut self,
        experiment_name: &str,
        artifact_path: Option<&str>,
    ) -> Result<String, crate::errors::ClientError<crate::errors::CreateExperimentErrorCode>> {
        let creation = self.api.create_experiment(experiment_name, artifact_path);
        if let Ok(experiment_id) = creation.as_ref() {
            self.active_experiment_id = Some(experiment_id.clone());
        }
        creation
    }

    fn ensure_active_run(&mut self) -> Result<&String, ()> {
        if self.active_run_id.is_none() {
            self.start_run_internal(None, None).map_err(|_| ())?;
        }
        self.active_run_id.as_ref().ok_or(())
    }

    /// Log a parameter under the current run, creating a run if necessary.
    pub fn log_param(&mut self, key: &str, value: &str) -> Result<(), ()> {
        let run_id = self.ensure_active_run()?.clone();
        self.api.log_param(&run_id, key, value).map_err(|_| ())
    }

    /// Log a batch of params for the current run, starting a run if no runs are active.
    pub fn log_params(&mut self, params: &[&crate::Param]) -> Result<(), ()> {
        let run_id = self.ensure_active_run()?.clone();
        self.api
            .log_batch(&run_id, None, Some(params), None)
            .map_err(|_| ())
    }

    /// Log a metric under the current run, creating a run if necessary.
    pub fn log_metric(&mut self, key: &str, value: f32) -> Result<(), ()> {
        self.log_metric_at_step(key, value, 0)
    }

    /// Log a metric under the current run at step, creating a run if necessary.
    pub fn log_metric_at_step(&mut self, key: &str, value: f32, step: u64) -> Result<(), ()> {
        let run_id = self.ensure_active_run()?.clone();
        self.api
            .log_metric(
                &run_id,
                key,
                value,
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .expect("time went strange there")
                    .as_millis() as u64,
                Some(step),
            )
            .map_err(|_| ())
    }

    /// Log a batch of metrics for the current run, starting a run if no runs are active.
    pub fn log_metrics(&mut self, metrics: &[&crate::Metric]) -> Result<(), ()> {
        let run_id = self.ensure_active_run()?.clone();
        self.api
            .log_batch(&run_id, Some(metrics), None, None)
            .map_err(|_| ())
    }

    /// Set a tag under the current run, creating a run if necessary.
    pub fn set_tag(&mut self, key: &str, value: &str) -> Result<(), ()> {
        let run_id = self.ensure_active_run()?.clone();
        self.api.set_run_tag(&run_id, key, value).map_err(|_| ())
    }

    /// Log a batch of tags for the current run, starting a run if no runs are active.
    pub fn set_tags(&mut self, tags: &[&crate::RunTag]) -> Result<(), ()> {
        let run_id = self.ensure_active_run()?.clone();
        self.api
            .log_batch(&run_id, None, None, Some(tags))
            .map_err(|_| ())
    }

    /// Delete a tag from a run. This is irreversible.
    pub fn delete_tag(&mut self, key: &str) -> Result<(), ()> {
        let run_id = self.ensure_active_run()?.clone();
        self.api.delete_run_tag(&run_id, key).map_err(|_| ())
    }

    /// Get the currently active Run, or None if no such run exists.
    pub fn active_run(&self) -> Result<crate::Run, ()> {
        self.active_run_id
            .as_ref()
            .ok_or(())
            .and_then(|run_id| self.api.get_run(&run_id).map_err(|_| ()))
    }

    /// End an active MLflow run (if there is one).
    pub fn end_run(&self) -> Result<(), ()> {
        self.update_run_status(crate::RunStatus::Finished)
    }

    /// Update an active MLflow run (if there is one) with the specified status.
    pub fn update_run_status(&self, status: crate::RunStatus) -> Result<(), ()> {
        let end_time = match status {
            crate::RunStatus::Failed | crate::RunStatus::Finished | crate::RunStatus::Killed => {
                Some(
                    std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .expect("time went strange there")
                        .as_millis() as u64,
                )
            }
            _ => None,
        };
        self.active_run_id.as_ref().ok_or(()).and_then(|run_id| {
            self.api
                .update_run(&run_id, status, end_time)
                .map(|_| ())
                .map_err(|_| ())
        })
    }

    /// Get a list of runs that fit the search criteria.
    pub fn search_runs(
        &self,
        experiment_ids: &[&str],
        filter_string: Option<&str>,
        run_view_type: Option<crate::ViewType>,
        max_result: Option<u32>,
        order_by: Option<&[&str]>,
    ) -> Result<Vec<crate::Run>, ()> {
        self.api
            .search_runs(
                experiment_ids,
                filter_string,
                Some(run_view_type.unwrap_or(crate::ViewType::ActiveOnly)),
                Some(max_result.unwrap_or(100_000)),
                order_by,
                None,
            )
            .map(|v| v.0)
            .map_err(|_| ())
    }

    /// Delete an experiment from the backend store.
    pub fn delete_experiment(&self, experiment_id: &str) -> Result<(), ()> {
        self.api.delete_experiment(experiment_id).map_err(|_| ())
    }

    /// Deletes a run with the given ID.
    pub fn delete_run(&self, run_id: &str) -> Result<(), ()> {
        self.api.delete_experiment(run_id).map_err(|_| ())
    }

    /// Get the absolute URI of the specified artifact in the currently active run. If path is not specified, the
    /// artifact root URI of the currently active run will be returned.
    pub fn get_artifact_uri(&self, artifact_path: Option<&str>) -> Result<String, ()> {
        self.active_run_id
            .as_ref()
            .ok_or(())
            .and_then(|run_id| self.api.get_run(&run_id).map_err(|_| ()))
            .map(|run| format!("{}/{}", run.info.artifact_uri, artifact_path.unwrap_or("")))
    }
}
