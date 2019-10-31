use structopt::{clap::AppSettings, StructOpt};

#[derive(StructOpt)]
#[structopt(
    setting(AppSettings::ColoredHelp),
    name = "mlflow-cli",
    about = "CLI example to interact with MLFlow"
)]
struct Opt {
    #[structopt(short, long, help = "URL to the MLFlow server")]
    url: String,
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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    let mlflow = mlflow_api::MlflowClient::new(opt.url)?;
    match opt.command {
        Commands::CreateExperiment { name } => {
            println!("{:#?}", mlflow.create_experiment(name.clone(), None)?);
        }
        Commands::ListExperiments => println!("{:#?}", mlflow.list_experiments(None)?),
        Commands::GetExperiment { experiment_id } => {
            println!("{:#?}", mlflow.get_experiment(experiment_id)?)
        }
        Commands::GetExperimentByName { experiment_name } => {
            println!("{:#?}", mlflow.get_experiment_by_name(experiment_name)?)
        }
        Commands::DeleteExperiment { experiment_id } => {
            println!("{:#?}", mlflow.delete_experiment(experiment_id)?)
        }
        Commands::UpdateExperiment {
            experiment_id,
            new_name,
        } => println!("{:#?}", mlflow.update_experiment(experiment_id, new_name)?),
        Commands::SetExperimentTag {
            experiment_id,
            key,
            value,
        } => println!(
            "{:#?}",
            mlflow.set_experiment_tag(experiment_id, key, value)?
        ),
    }

    Ok(())
}
