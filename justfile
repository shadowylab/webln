fmt:
    cargo fmt --all -- --config format_code_in_doc_comments=true

check:
    cargo check
    cargo clippy
    cargo doc