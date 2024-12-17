use crate::{config::RoutingLayerConfig, Grid, Node};
use grid_node_core::Network;
use std::marker::PhantomData;

/// Forces a [`Clone`] for the reusable build
/// associated function.
///
pub trait Builder<N: Network>: Clone {
    /// Reusable builder associated function.
    ///
    /// Does not consume Self.
    ///
    fn build(&self) -> Node<N>;
}

#[derive(Clone, Debug)]
pub struct NodeBuilder<N: Network> {
    _network: PhantomData<N>,
}

impl<N: Network> NodeBuilder<N> {
    pub fn new() -> Self {
        Self {
            _network: Default::default(),
        }
    }
}

//------------------------------------------
// Grid Node Builder.
//------------------------------------------

impl<N: Network> NodeBuilder<N> {
    /// Instantiate new Grid Node builder.
    pub fn grid_node() -> GridNodeBuilder {
        GridNodeBuilder::new()
    }
}

#[derive(Clone, Debug)]
pub struct GridNodeBuilder {
    routing_config: Option<RoutingLayerConfig>,
}

impl GridNodeBuilder {
    pub fn new() -> Self {
        Self {
            routing_config: None,
        }
    }

    pub fn routing(mut self, config: RoutingLayerConfig) -> Self {
        self.routing_config = Some(config);
        self
    }
}

impl<N: Network> Builder<N> {
    pub fn build(&self) -> Node {
        Node::new_grid(self.routing_config.clone())
    }
}

//------------------------------------------
// Data Node Builder.
//------------------------------------------

impl NodeBuilder {
    /// Instantiate new Data Node builder.
    pub fn data_node() -> DataNodeBuilder {
        DataNodeBuilder::new()
    }
}

#[derive(Clone, Debug)]
pub struct DataNodeBuilder;

impl DataNodeBuilder {
    pub fn new() -> Self {
        Self {}
    }
}
