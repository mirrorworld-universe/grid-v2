use anyhow::Result;
use clap::{Parser, ValueEnum};
use grid_logger::{initialize_logger, tracing::*};
use grid_node::{
    builder::{Builder, NodeBuilder},
    NodeScaffolding,
};
use grid_node_core::prelude::*;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use tokio::runtime::{self, Runtime};

const DEFAULT_RPC_PORT: u16 = 1024;
const DEFAULT_RPC_PUBSUB_PORT: u16 = 1025;
const DEFAULT_NODE_IP: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const DEFAULT_NODE_TYPE: NodeType = NodeType::Grid;

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum ClusterArg {
    CanaryV0,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
pub enum NodeTypeArg {
    Grid,
}

pub type Port = u16;

#[derive(Clone, Debug, Parser)]
#[command(arg_required_else_help(true))]
pub struct Node {
    #[clap(short = 'v', long = "verbosity")]
    pub verbosity: Option<u8>,
    #[clap(short = 'c', long = "cluster")]
    pub cluster: Option<ClusterArg>,
    #[clap(long = "node-ip")]
    pub node_ip: Option<IpAddr>,
    #[clap(long = "node-type")]
    pub node_type: Option<NodeTypeArg>,
    #[clap(long = "rpc-port")]
    pub rpc_port: Option<Port>,
    #[clap(long = "rpc-pubsub-port")]
    pub rpc_pubsub_port: Option<Port>,
}

impl Node {
    /// Starts the Grid node
    pub fn parse(self) -> Result<String> {
        let verbosity = match self.verbosity {
            Some(v) => v,
            None => 1,
        };

        // Initialize Grid Logger.
        initialize_logger(verbosity)?;

        info!("Starting Node");

        let node_ip: IpAddr = match self.node_ip {
            Some(ip) => ip,
            None => DEFAULT_NODE_IP,
        };

        let mut node_type = DEFAULT_NODE_TYPE;

        if let Some(node_type_arg) = self.node_type {
            node_type = match node_type_arg {
                NodeTypeArg::Grid => NodeType::Grid,
            };
        };

        let rpc_port = match self.rpc_port {
            Some(port) => port,
            None => DEFAULT_RPC_PORT,
        };

        let rpc_pubsub_port = match self.rpc_pubsub_port {
            Some(port) => port,
            None => DEFAULT_RPC_PUBSUB_PORT,
        };

        let node = NodeBuilder::<CanaryV0>::grid_node()
            .routing(node_ip, node_type, rpc_port)
            .runtime()
            .build()?;

        debug!("Node instance: {:?}", node);

        Self::runtime()?.block_on(async move {
            let grid_node::Node::Grid(grid) = node;

            info!("Running Node");
            tokio::spawn(async move { grid.run().await });

            info!("Node RPC Gateway: {}", SocketAddr::new(node_ip, rpc_port));
            info!(
                "Node RPC (PubSub): {}",
                SocketAddr::new(node_ip, rpc_pubsub_port)
            );
            std::future::pending::<()>().await;
        });

        Ok(String::new())
    }

    fn runtime() -> Result<Runtime> {
        Ok(runtime::Builder::new_multi_thread().enable_all().build()?)
    }
}
