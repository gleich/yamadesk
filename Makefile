.PHONY: sync


sync:
	rsync -avh --delete --progress --exclude 'target' --exclude '.git' . matt@gleichpi.local:~/yamadesk/

install:
	cargo install --path .
	sudo mv yamadesk.service /etc/systemd/system/
	sudo systemctl daemon-reload
	sudo systemctl enable yamadesk.service
	sudo systemctl start yamadesk.service