use std::borrow::Borrow;
use std::path::PathBuf;
use std::collections::HashMap;
struct AssetRegistry{}
struct AssetRegistryBuilder{
    encoders:HashMap<String,fn(&[u8],Option<&HashMap<String,String>>)->Result<Box<[u8]>,String>>,
    base_dir:PathBuf,
    target:PathBuf,
    warning:Option<fn(&str,&str)>,
}
impl AssetRegistryBuilder{
    pub fn new<Path:Borrow<Path>>(path:Path)->Self{
        todo!();
    }
    pub fn add_encoder(&mut self,extension:&str,encoder:fn(&[u8],Option<&HashMap<String,String>>)->Result<Box<[u8]>,String>) -> &mut Self{
        self.encoders.insert(extension.to_owned(),encoder);
        self
    }
    fn encode(self)->Result<AssetRegistry,()>{todo!()}
}
