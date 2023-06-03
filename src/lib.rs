use std::borrow::Borrow;
use std::path::PathBuf;
use std::path::Path;
use std::collections::HashMap;
use std::ffi::OsStr;
pub struct AssetEncodeContext<'a>{
    config:Option<&'a HashMap<String,String>>,
    path:&'a str,
    warn:Option<fn(&str,&str)>,
    inform:Option<fn(&str,&str)>,
}
impl<'a> AssetEncodeContext<'a>{
    fn new(arb:&AssetRegistryBuilder,config:Option<&'a HashMap<String,String>>,path:&'a str)->Self{
        AssetEncodeContext{config,path,warn:arb.warn,inform:arb.inform}
    }
    pub fn warn(&self,warning:&str){
        match self.warn{
            Some(warn)=>warn(self.path,warning),
            None=>(),
        }
    }
    pub fn inform(&self,message:&str){
        match self.inform{
            Some(inform)=>inform(self.path,message),
            None=>(),
        }
    }
}
struct EncodedAssetRegistry{}
struct AssetRegistryBuilder{
    encoders:HashMap<String,fn(&[u8],&AssetEncodeContext)->Result<Box<[u8]>,String>>,
    base_dir:PathBuf,
    target:PathBuf,
    warn:Option<fn(&str,&str)>,
    inform:Option<fn(&str,&str)>
}
impl AssetRegistryBuilder{
    pub fn new<SrcDirPath:AsRef<OsStr> + ?Sized,TargetPath:AsRef<OsStr> + ?Sized>(base_dir:&SrcDirPath,target:&TargetPath)->Self{
        Self{encoders:HashMap::new(),warn:None,inform:None,base_dir:Path::new(&base_dir).to_owned(),target:Path::new(&target).to_owned()}
    }
    pub fn add_encoder(&mut self,extension:&str,encoder:fn(&[u8],&AssetEncodeContext)->Result<Box<[u8]>,String>) -> &mut Self{
        self.encoders.insert(extension.to_owned(),encoder);
        self
    }
    pub fn set_warning_lisener(&mut self,mut warn:Option<fn(&str,&str)>)->Option<fn(&str,&str)>{
        std::mem::swap(&mut warn,&mut self.warn);
        warn
    }
    pub fn set_information_lisener(&mut self,mut inform:Option<fn(&str,&str)>)->Option<fn(&str,&str)>{
        std::mem::swap(&mut inform,&mut self.inform);
        inform
    }
    fn collect_files(path:&Path)->Result<Vec<PathBuf>,AssetBuildError>{
        let mut files = Vec::new();
        for entry in std::fs::read_dir(path)?{
            let entry = entry?;
            let ftype = entry.file_type()?;
            if ftype.is_dir(){
                files.extend(Self::collect_files(&entry.path())?);
            }
            else if ftype.is_file(){
                files.push(entry.path().to_owned());
            }
        }
        Ok(files)
    }
    fn encode(self)->Result<EncodedAssetRegistry,AssetBuildError>{
        //println!("{:?}",self.base_dir.canonicalize());
        let files = Self::collect_files(&self.base_dir)?;
        let hs:std::collections::HashSet<PathBuf> = files.into_iter().collect();
        //let files_configs = hs.iter().map(|val|{hs.get(&format!("{}.asscfg",val))}).collect();
        todo!("files:{hs:?}");
    }
}
#[derive(Debug)]
pub enum AssetBuildError{
    EncodeError(String),
    IOError(std::io::Error),
}
impl From<std::io::Error> for AssetBuildError{
    fn from(err: std::io::Error) -> Self {
        Self::IOError(err)
    }
}
#[test]
fn create_arb(){
    let _arb = AssetRegistryBuilder::new("./testing/","target/asset_reg/");
}
#[test]
fn add_encoder(){
    fn do_nothing(_:&[u8],_:&AssetEncodeContext)->Result<Box<[u8]>,String>{
        Ok((*b"").into())
    }
    let mut arb = AssetRegistryBuilder::new("testing/","target/asset_reg/");
    arb.add_encoder(".uwu",do_nothing);
}
#[test]
fn set_warning_lisener(){
    fn warn(_:&str,_:&str){}
    let mut arb = AssetRegistryBuilder::new("testing/","target/asset_reg/");
    arb.set_warning_lisener(Some(warn));
}
#[test]
fn set_information_lisener(){
    fn inform(_:&str,_:&str){}
    let mut arb = AssetRegistryBuilder::new("testing/","target/asset_reg/");
    arb.set_information_lisener(Some(inform));
}
#[test]
fn test_empty_encode(){
    let arb = AssetRegistryBuilder::new("testing/","target/asset_reg/");
    let _ = arb.encode().unwrap();
}
