use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, Read},
    path::{Path, PathBuf},
    process::{Child, Command},
    str::FromStr,
};
use thiserror::Error;
use anyhow::{Context, Result};

use crate::temp_path::TempPath;
use config::node_config::NodeConifg;

#[derive(Debug)]
pub enum SwarmDir {
    Persistent(PathBuf),
    Temporary(TempPath)
}

impl AsRef<Path> for SwarmDir {
    fn as_ref(&self) -> &Path{
        match self {
            SwarmDir::Persistent(path) => path.as_path(),
            SwarmDir::Temporary(path) => path.path(),
        }
    }
}

#[derive(Debug)]
struct AccountAddress {  
}
#[derive(Debug)]
struct NodeDebugclient {
}

pub struct HeNode{
    node: Child,
    node_id: String,
    validator_peer_id:Option<AccountAddress>,
    role:RoleType,
    debug_client:NodeDebugclient,
    port:u16,
    log:PathBuf,
}

impl Drop for HeNode{
    fn drop(&mut self){
        match self.node.try_wait(){
            Ok(Some(_)) => {},
            _=>{
                if let Err(e) = self.node.kill(){
                    panic!("Node process can not be killed {}",e);
                }
            }
        }
    }
}

impl HeNode {
    // add code here
    pub fn prepare(){
        Command::new(get_node_bin_path());
    }

    pub fn launch(node_id:String, role:RoleType, config_path:&Path,log_path:PathBuf) -> Result<Self>{
        let config = NodeConifg::load(&config_path).expect("");
        let log_file = File::create(&log_path)?;

        let validator_peer_id=match role {
            RoleType::Validator => Some(config.validator_network.as_ref().unwrap().peer_id()),
            RoleType::FullNode  => None,
        };

        let mut node_command = Command::new(get_node_bin_path());
        node_command.current_dir("./")
                    .arg("-f")
                    .arg(config_path);

        let node = node_command.spawn().context("Error launching node process")?;

        let debug_client = NodeDebugclient{};


        Ok(Self{
            node,
            node_id:String::from("fuck"),
            validator_peer_id,
            role,
            debug_client,
            port:12345,
            log:log_path,
        })
    }

}

fn get_node_bin_path() -> PathBuf{
    let tmp_path = env::current_exe().ok().map(|mut path|{
        path.pop();
        if path.ends_with("deps"){
            path.pop();
        }
        path
    });

    tmp_path.unwrap().join("he-node")
}


pub struct SwarmConfig {
    
}

pub enum RoleType {
    Validator,
    FullNode,
}

#[derive(Debug, Error)]
enum SwarmLaunchFailure {
    
}


pub struct HeSwarm {
    //the directry to hold the db file config files
    pub dir: SwarmDir,
    
    //Maps the node id of a node to the HeNode struct
    pub nodes: HashMap<String, HeNode>,

    //the config infomations
    pub config: SwarmConfig,
    pub role: RoleType,

    
}

impl HeSwarm {

    pub fn setup_config_dir() -> SwarmDir{
        let temp_dir = TempPath::new();
        temp_dir.create_as_dir().expect("unable to create tmp dir");
    }

    pub fn configure_validator_swarm(num_nodes:usize,config_dir:Option<String>) -> Self{
        let dir  = SwarmDir::Persistent(PathBuf::from(""));
        let nodes:HashMap<String, HeNode> = HashMap::new();
        let config  = SwarmConfig{};
        let role = RoleType::Validator;

        HeSwarm{
            dir,
            nodes,
            config,
            role,
        }

    }

    pub fn launch(&mut self){
        let num_attempts = 5;
        for _ in 0..num_attempts {
            match self.launch_attempt(false) {
                Ok(_) => return,
                Err(err) => error!("Error in launching the swarm {}", err),
            }
        }

        panic!("Still Error alfter attempts {} times", num_attempts);
    }

    pub fn launch_attempt(&self,log:bool) -> Result<(), SwarmLaunchFailure>{
        let logs_dir_path = self.dir.as_ref().join("logs");
        std::fs::create_dir(&logs_dir_path)?;

        for (index, path) in self.config.config_files.iter().enumerate() {
            let node_id = format!("{}", index);
            let node = HeNode::launch(
                node_id.clone(),
                self.role,
                &path,
                logs_dir_path.join(format!("{}.log", index)),
                logs_dir_path.join(format!("{}.struct.log", index)),
                log
            ).unwrap();

            self.nodes.insert(node_id, node);
        }

        let expected_peers = match self.role {
            RoleType::Validator => self.nodes.len() as i64-1,   
            RoleType::FullNode  => 1,
        };

        self.wait_for_startup()?;
        self.wait_for_connectivity(expected_peers)?;
        Ok(())
    }
}

