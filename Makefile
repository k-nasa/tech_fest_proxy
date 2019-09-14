lazy_up:
	cargo build --release
	systemctl daemon-reload
	systemctl restart nasa-tech.service
