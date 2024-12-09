pub mod grid;

use crate::grid::Grid;
use async_trait::async_trait;
use grid_node_core::Network;
use grid_node_router::Routing;

//------------------------------------------
// Node
//------------------------------------------

/// Node Enum.
///
/// Lists the available Node types.
///
pub enum Node<N: Network> {
    Grid(Grid<N>),
}

impl<N: Network> Node<N> {
    pub fn new_grid() -> Self {
        let node_type = NodeType::Grid;
        Self::Grid(Grid::new(node_type).unwrap())
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

    /// Spawns Node services.
    async fn spawn(&self);
    /// Runs Node and initial services.
    async fn run(&self);

    //------------------------------------------
    // Getters
    //------------------------------------------

    /// Get Node type
    fn node_type(&self) -> NodeType;
}