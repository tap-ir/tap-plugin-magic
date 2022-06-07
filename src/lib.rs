pub mod magic;

use std::sync::Arc;
use std::collections::HashMap;

use tap::tree::{Tree, TreeNodeId};
use tap::node::Node;
use tap::value::Value;
use tap_query::attribute::find_vfiles;
use rayon::prelude::*;
use log::info;

/// Return a list of modules that can ben run against some node by checking the data type
pub fn plugins_datatype(tree : &Tree, plugins_types : &HashMap<String, Vec<String>>) -> Vec<(TreeNodeId, String)> 
{
  let arena = tree.arena();
  let mut plugins_nodes: Vec<(TreeNodeId, String)> = Vec::new();

  //could be parellized 
  for node_id in tree.root_id.descendants(&arena)
  {
     let node = tree.get_node_from_id(node_id).unwrap();
     let value_datatype = match node.value().get_value("datatype")
     {
       Some(value) => value,
       None => continue,
     };

     let node_datatype = match value_datatype
     {
       Value::Str(value_datatype) => value_datatype,
       _ => continue,
     };

     for (plugin_name, plugin_datatypes) in plugins_types
     {
       for plugin_datatype in plugin_datatypes
       {
         //if plugins not already applied -> check if nodes was passed to a plugin ...
         //all modules must register a value of plugin name 
         if node_datatype == plugin_datatype.clone() && node.value().get_value(plugin_name) == None
         {
           plugins_nodes.push((node_id, plugin_name.clone()));
         }
       }
     }
  }

  plugins_nodes
}

/// Add datatype for each data found in the tree without datatype 
/// Return a list of all nodes that don't have the datatype attribute before
pub fn datatypes(tree : &Tree) -> Vec<TreeNodeId>
{
   let mut nodes_ids : Vec<(TreeNodeId, Arc<Node>)> = Vec::new();

   for node_id in find_vfiles(tree) //use map or filter check if size > 0 ?
   { 
      let node = tree.get_node_from_id(node_id).unwrap();
      //if there is no datatype value in node this node must be processed 
      if node.value().get_value("datatype").is_none()
      {
         nodes_ids.push((node_id, node));
      }
   }

   info!("datatypes on {} nodes", nodes_ids.len());
   let total : Vec<_> = nodes_ids.par_iter().filter(|nodes_id|  datatype(&nodes_id.1).is_some()).map(|nodes_id| nodes_id.0).collect();
   info!("datatypes found {} valid datatype", total.len());
   total
}

/// Get the type of the data value of a node, and add the type as attribute
pub fn datatype(node : &Node) -> Option<String> 
{
  let value = node.value().get_value("data")?;
 
  let builder = match value
  {
    Value::VFileBuilder(builder) => builder,
    _ => return None,
  };

  if builder.size() == 0
  {
    return None
  }

  let mut file = match builder.open()
  {
    Ok(file) => file,
    Err(err) => { eprintln!("plugin-magic error : {} on {}", err, node.name()); return None }
  };
  
  let mut buffer = [0; 4096];
  if file.read(&mut buffer).is_ok()
  {
     let result : &'static str  = tree_magic_mini::from_u8(&buffer);
     node.value().add_attribute("datatype", Value::from(result), None);
     return Some(result.to_string())
  }
     
  None
}
