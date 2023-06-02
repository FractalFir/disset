# This crate does not exist yet. All APIs and examples are mere ideas. 
## What is disset?
`disset` is an extremely flexible system for storing game assets. An asset registry is created from a path to a directory. As all files in a given directory are traversed, relevant encoders get the files.
An encoder may be registered for *any* file extension. All files with that extension will then be processed by that particular encoder, and then saved in a compressed bundle. This means that the system has makes no assumptions about encoded data, and adding another support for another file format is as easy as writing a function that will encode it and registering the encoder. The system automatically tires to split the asset bundle into files with roughly equal, optimal size. This makes updating a game easier, because a large chunk of files with unchanged assets will remain the same and won't need to be downloaded again. Furthermore, the system is equally flexible when it comes to reading data. A read bundle may have any numbers of `Decoder`s, which take in a relative path to the relevant asset an return it's decoded version, if the decoder is registered for that file type.
## Exaples
in *build.rs*
```rust
fn encode_phone_numbers(text_file:&[u8],_config:Option<&HashMap<String,String>>)->Result<Box<[u8]>,String>>{
    // Parse the text file containg numbers
    // Save them in format of your choice.
}
fn encode_cat_pictures(text_file:&[u8],_config:Option<&HashMap<String,String>>)->Result<Box<[u8]>,String>>{
    // Do your work
    if is_dog(picture){
        Err("That is a dog!")
    }
    else{
        Ok(picture.encode())
    }
}

fn encode_shader_code(shader_source:&[u8],config:Option<&HashMap<String,String>>)->Result<Box<[u8]>,String>>{
    // If shaders "shader_name.ascfg" contains "check_shader = true", check the shder
    if config.get("check_shader").is_some_and(|value|{value == "true"}){
        if let Some(error) = check_shader(shader_source){
            return Err(format!("Encountred shader error {error}!"));
        }
    }
    // No conversion to do, shader source code is already encoded.
    Ok(shader_source.into())
}
fn main(){
    let mut registry_builder = AssetRegistryBuilder::new("dir/with/assets/);
    registry_builder.add_encoder(".phnum",encode_phone_numbers);
    //Can be chained
    registry_builder.add_encoder(".catpic",encode_cat_pictures);
}
```
