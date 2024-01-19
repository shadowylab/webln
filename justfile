fmt:
    cargo fmt --all -- --config format_code_in_doc_comments=true

check: fmt check-crates check-crates-msrv check-docs

check-fmt:
	cargo fmt --all -- --config format_code_in_doc_comments=true --check

check-crates:
	@bash contrib/scripts/check-crates.sh

check-crates-msrv:
	@bash contrib/scripts/check-crates.sh msrv

check-docs:
	@bash contrib/scripts/check-docs.sh