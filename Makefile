.PHONY: sync


sync:
	rsync -avh --delete --progress --exclude 'target' --exclude '.git' . matt@gleichpi.local:~/yamadesk/