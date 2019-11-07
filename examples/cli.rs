use structopt::{clap::AppSettings, StructOpt};

#[derive(StructOpt)]
#[structopt(
    setting(AppSettings::ColoredHelp),
    name = "mlflow-api-cli",
    about = "CLI example to interact with MLFlow API"
)]
struct Opt {
    #[structopt(
        short,
        long,
        help = "URI to the MLFlow server",
        env = "MLFLOW_TRACKING_URI"
    )]
    uri: String,
    #[structopt(subcommand)]
    command: Commands,
}

#[derive(StructOpt)]
enum Commands {
    #[structopt(about = "Create an experiment")]
    CreateExperiment {
        #[structopt(help = "Name of the experiment to create")]
        name: String,
    },
    #[structopt(about = "List experiments")]
    ListExperiments,
    #[structopt(about = "Get an experiment")]
    GetExperiment {
        #[structopt(help = "Id of the experiment to get")]
        experiment_id: String,
    },
    #[structopt(about = "Get an experiment by its name")]
    GetExperimentByName {
        #[structopt(help = "Name of the experiment to get")]
        experiment_name: String,
    },
    #[structopt(about = "Delete an experiment")]
    DeleteExperiment {
        #[structopt(help = "Id of the experiment to delete")]
        experiment_id: String,
    },
    #[structopt(about = "Update an experiment")]
    UpdateExperiment {
        #[structopt(help = "Id of the experiment to update")]
        experiment_id: String,
        #[structopt(help = "New name for the experiment")]
        new_name: String,
    },
    #[structopt(about = "Set a tag on an experiment")]
    SetExperimentTag {
        #[structopt(help = "Id of the experiment to tag")]
        experiment_id: String,
        #[structopt(help = "Name of the tag")]
        key: String,
        #[structopt(help = "Value of the tag")]
        value: String,
    },
    #[structopt(about = "Create an run for an experiment")]
    CreateRun {
        #[structopt(help = "ID of the experiment for the new run")]
        experiment_id: String,
    },
    #[structopt(about = "List artifacts of a run")]
    ListArtifacts {
        #[structopt(help = "ID of the run to list artifacts of")]
        run_id: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    let mlflow = mlflow_api::MLflowAPI::new(&opt.uri)?;
    match opt.command {
        Commands::CreateExperiment { name } => {
            println!("{:#?}", mlflow.create_experiment(&name, None)?);
        }
        Commands::ListExperiments => println!("{:#?}", mlflow.list_experiments(None)?),
        Commands::GetExperiment { experiment_id } => {
            println!("{:#?}", mlflow.get_experiment(&experiment_id)?)
        }
        Commands::GetExperimentByName { experiment_name } => {
            println!("{:#?}", mlflow.get_experiment_by_name(&experiment_name)?)
        }
        Commands::DeleteExperiment { experiment_id } => {
            println!("{:#?}", mlflow.delete_experiment(&experiment_id)?)
        }
        Commands::UpdateExperiment {
            experiment_id,
            new_name,
        } => println!(
            "{:#?}",
            mlflow.update_experiment(&experiment_id, &new_name)?
        ),
        Commands::SetExperimentTag {
            experiment_id,
            key,
            value,
        } => println!(
            "{:#?}",
            mlflow.set_experiment_tag(&experiment_id, &key, &value)?
        ),
        Commands::CreateRun { experiment_id } => {
            println!(
                "{:#?}",
                mlflow.create_run(
                    &experiment_id,
                    Some(
                        std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .expect("time went strange there")
                            .as_millis() as u64
                    ),
                    None
                )?
            );
        }
        Commands::ListArtifacts { run_id } => {
            println!("{:#?}", mlflow.list_artifacts(&run_id, None)?);
        }
    }

    Ok(())
}
