//use {
 //   std::{
  //      env,
   //     io,
   // },
   // winres::WindowsResource,
//};

use std::io;

#[cfg(target_os = "windows")]
fn main() -> io::Result<()> {
    //if env::var_os("CARGO_CFG_WINDOWS").is_some() {
     //   WindowsResource::new()
            // This path can be absolute, or relative to your crate root.
     //       .set_icon("assets/logo.ico")
      //      .compile()?;
    //}
    //Ok(())
}

#[cfg(target_os = "macos")]
fn main() -> io::Result<()> {
    Ok(())
}