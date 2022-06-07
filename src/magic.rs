//! Magic plugin take a data attribute from a node and extract magic info if compatible with this plugin 

use std::str;
use std::collections::HashMap;

use tap::config_schema;
use tap::plugin;
use tap::plugin::{PluginInfo, PluginInstance, PluginConfig, PluginArgument, PluginResult, PluginEnvironment};
use tap::tree::{TreeNodeId, TreeNodeIdSchema}; 

use schemars::{JsonSchema};
use serde::{Serialize, Deserialize};

use crate::{datatypes, plugins_datatype};

plugin!("magic", "Metadata", "Detect magic and file data compatible with plugins", Magic, Arguments);


#[derive(Default)]
pub struct Magic 
{
}

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct Arguments
{
 #[schemars(with = "TreeNodeIdSchema")] 
  root_id : TreeNodeId,
  plugins_types : HashMap<String, Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize,Default)]
pub struct Results
{
  datatypes : Vec<TreeNodeId>,
  nodes_plugins : Vec<(TreeNodeId, String)>,
}

impl Magic
{
  fn run(&mut self, args : Arguments, env : PluginEnvironment) -> anyhow::Result<Results>
  {
    let datatypes = datatypes(&env.tree);
    let nodes_plugins = plugins_datatype(&env.tree, &args.plugins_types);

    Ok(Results{ datatypes, nodes_plugins })
  }
}
