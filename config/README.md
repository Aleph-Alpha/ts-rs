# ts-rs-config
This crate contains the config for future ts-rs versions.  
Currently, it's not really possible no use the config within the proc macro due to issues regarding incremental compilation.  
A workaround would be to have `include_str!("ts.toml")` everywhere, but I don't think we should do that.  
Instead, let's wait for https://github.com/rust-lang/rust/issues/73921