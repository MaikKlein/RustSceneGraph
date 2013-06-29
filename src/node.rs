extern mod lmath;
struct Mesh;
pub struct RootSceneNode{
	  node: SceneNode
}
impl  RootSceneNode {
	
	pub fn create_child_node<'a>(&'a mut self,name: ~str) -> &'a mut SceneNode{
		self.node.create_child_node(name)
	}
	pub fn traverse(&mut self, f: &fn(~str)){
		self.node.traverse(f,self.node.model);
	}
	pub fn new(name: ~str) -> RootSceneNode{
		RootSceneNode { node: SceneNode::new(name) }
	}
}

struct SceneNode{
	name: ~str,
	child:  ~[SceneNode],
	model: lmath::mat::mat4,
	position: lmath::vec::Vec4<f32>,
	mesh: Mesh
}
impl SceneNode {

	pub fn translate<'a>(&'a mut self,v: lmath::vec::Vec4<f32>) ->&'a mut SceneNode {
		
		let mut translate = lmath::mat::Mat4::identity::<f32>();
		translate.x.x = 1.0f32;
		translate.x.w = v.x;
		translate.y.w = v.y;
		translate.z.w = v.z;
		translate.w.w = v.w;
		self.model = self.model.mul_m(&translate);
		self
	}
	pub fn rotate<'a>(&'a mut self)-> &'a mut SceneNode{
		self
	}
	fn new(name: ~str )-> SceneNode{
		let mut m = lmath::mat::Mat4::identity::<f32>();
		m.x.x = 1.0f32;
		SceneNode {name: name,child: ~[],model: m,position: lmath::vec::Vec4::zero(),mesh: Mesh}
	}
  	pub fn create_child_node<'a>(&'a mut self, name: ~str)-> &'a mut SceneNode {
		self.child.push(SceneNode::new(name));
		&mut(self.child[self.child.len()-1])
		
    }
  	pub fn get_children<'a>(&'a mut self) -> &'a mut ~[SceneNode]{
		&mut(self.child)
	}

     fn traverse(&mut self, f: &fn(~str),m: lmath::mat::mat4 ){
    	self.model = self.model.mul_m(&m);
  		f( copy self.name);
  		f( fmt!("%?",self.model));
  		for self.child.mut_iter().advance() |x| {
  			x.traverse(f,self.model);
  		}
  	}
}



