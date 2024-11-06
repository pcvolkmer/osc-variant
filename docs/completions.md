## Command Completion

Die Anwendung kann selbständig Scripte zur automatischen Ergänzung der Unterbefehle erstellen,
sollte dies nicht im Installationspaket enthalten sein.

### Bash

Erzeugen des Completion-Files mit:

```bash
mkdir -p ~/.local/share/bash-completion/completions
osc-variant completion bash > ~/.local/share/bash-completion/completions/osc-variant
```

Nach einem Neustart des Terminals sollte nun die Completion verfügbar sein.
Alternativ kann im aktuellen Terminal auch folgendes angewendet werden:

```bash
source <(osc-variant completion bash)
```

### Zsh

Erzeugen des Completions-Files mit:

```sh
mkdir -p ~/.osc-variant/completions
osc-variant completion zsh > ~/.osc-variant/completions/_osc-variant
```

Hinzufügen zur Umgebungsvariable `FPATH` zur Konfiguration mit:

```sh
cat <<"EOT" >> ~/.zshrc
FPATH="$HOME/.osc-variant/completions:$FPATH"
autoload -Uz compinit
compinit
EOT
```

Nach einem Neustart des Terminals sollte nun die Completion verfügbar sein.

### Fish

Erzeugen des Completions-Files mit:

```sh
mkdir -p ~/.config/fish/completions
osc-variant completion fish > ~/.config/fish/completions/osc-variant.fish
```

Die Completion sollte sofort verfügbar sein.