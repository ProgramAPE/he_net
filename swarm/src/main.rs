use swarm::he_swarm::HeSwarm;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about="swarm to start local nodes")]
struct Args{
    #[structopt(short="n", long, default_value="1")]
    pub num_nodes: usize,

    #[structopt(short="l", long)]
    pub enable_logging:bool,

    #[structopt(short="s", long)]
    pub start_client:bool,

    #[structopt(short="c", long)]
    pub config_dir:Option<String>,

    #[structopt(short="f", long, default_value="0")]
    pub num_full_nodes:usize,
}


pub fn main(){
    let args = Args::from_args();
    let num_nodes = args.num_nodes;
    let num_full_nodes = args.num_full_nodes;

    let mut validator_swarm = HeSwarm::configure_validator_swarm(num_nodes, args.config_dir.clone());

    validator_swarm.launch_attempt(false);

    // let root_key_path = &validator_swarm.config.root_key_path;;
    // let validator_config = NodeConfig::load(&validator_swarm.config.config_files[0]).unwrap();

    println!("so far ok");
}