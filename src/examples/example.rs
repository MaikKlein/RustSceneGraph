pub mod node;
use node::*;

fn main(){ 
let mut root = RootSceneNode::new(~"root");

let v = lmath::vec::Vec4::new::<f32>(10.0f32,
									 5.0f32,
									 25.0f32,
									 1.0f32);

root.create_child_node(~"n1")
		.translate(v)
		.rotate()
			.create_child_node(~"n1.1")
				.translate(v)
				.rotate()
					.create_child_node(~"n1.1.1")
						.translate(v)
						.rotate();

root.create_child_node(~"n2")
    .create_child_node(~"n2.1")
    .create_child_node(~"n2.1.1");

root.traverse(|x|println(x));

}