pub mod grid;

use crate::grid::Grid;
use anyhow::Result;
use async_trait::async_trait;
use grid_node_core::Network;
use grid_node_router::Routing;
use std::{future::Future, net::IpAddr, sync::Arc};

//------------------------------------------
// Node
//------------------------------------------

/// Node Enum.
///
/// Lists the available Node types.
///
pub enum Node<N: Network> {
    /// Grid Node Type
    ///
    /// Mainly responsible for sequencing incoming
    /// transactions
    ///
    Grid(Arc<Grid<N>>),
}

impl<N: Network> Node<N> {
    pub fn new_grid() -> Self {
        let node_ip: IpAddr = "127.0.0.1".parse().unwrap();
        let node_type = NodeType::Grid;
        let rpc_port = 8080;
        let rpc_pubsub_port = 8081;
        Self::Grid(Arc::new(
            Grid::new(node_ip, rpc_port, rpc_pubsub_port, node_type).unwrap(),
        ))
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum NodeType {
    Grid,
}

/// NodeScaffolding Trait.
///
/// Defines the behaviors expected from a crude Node.
/// - Setup
/// - Startup
/// - Task Scheduling
/// - Graceful Teardown
/// - Shutdown
/// - Node Getters
///
/// Import Note:
/// A Node is expected to have Routing, because what is a node
/// without routing?
#[async_trait]
pub trait NodeScaffolding<N: Network>: Routing<N> {
    //------------------------------------------
    // Associated Functions
    //------------------------------------------

    /// Prepares Node before running.
    fn prepare(&self);
    /// Gracefully shuts down Node and its running services.
    fn shutdown(&self);

    //------------------------------------------
    // Asynchronous Associated Functions
    //------------------------------------------

    /// Runs Node and initial services.
    async fn run(&self) -> Result<()>;

    //------------------------------------------
    // Getters
    //------------------------------------------

    /// Get Node type.
    fn node_type(&self) -> NodeType;
}
