mod operators;
mod trigonometry;

use crate::traits::NodeBehavior;
pub use operators::{AddNode, DivideNode, ModuloNode, MultiplyNode, PowNode, SubtractNode};
pub use trigonometry::{CosNode, CotNode, SinNode, TanNode};

pub static MATH_NODES_FACTORIES: [fn() -> Box<dyn NodeBehavior>; 10] = [
    AddNode::new,
    DivideNode::new,
    ModuloNode::new,
    MultiplyNode::new,
    PowNode::new,
    SubtractNode::new,
    CosNode::new,
    CotNode::new,
    SinNode::new,
    TanNode::new,
];
