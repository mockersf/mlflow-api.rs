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
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Opt::from_args();

    let mlflow = mlflow_api::MlflowClient::new(opt.url)?;
    match opt.command {
        Commands::CreateExperiment { name } => {
            println!("{:?}", mlflow.create_experiment(name.clone(), None)?);
        }
        Commands::ListExperiments => println!("{:?}", mlflow.list_experiments(None)?),
    }

    Ok(())
}
