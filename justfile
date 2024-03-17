set windows-shell := ["powershell.exe", "-NoLogo", "-Command"]

release:
    cargo build -r
    upx --best --lzma ./target/release/rust-afk.exe