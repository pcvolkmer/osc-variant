---
title: osc-variant
section: 1
header: OSC-Variant Manual
---

# NAME

osc-variant - Anwendung zum Anpassen einer OSC-Datei an einen Standort

# SYNOPSIS

osc-variant <COMMAND>

# DESCRIPTION

Diese Anwendung dient zur Anpassung einer OSC-Datei an einen Standort und enthält verschiedene Befehle zur Verwaltung
und Bearbeitung von OSC-Dateien.

Zudem ist es möglich, OSC-Dateien zu exportieren und zu importieren, um sie in verschiedenen Umgebungen zu nutzen.

Für weitere Informationen, siehe auch: https://github.com/pcvolkmer/osc-variant

# COMMANDS

## sha256sum

Das Berechnen der SHA256 Prüfsumme ist mit dem Unterbefehl `sha256sum` auch unter Windows einfach möglich
und erzeugt eine Ausgabe analog dem Befehl auf Linux-Systemen:

```
osc-variant sha256sum meine-beispieldatei.osc
```

## list

Zum Auflisten der Inhalte einer Datei wird folgender Befehl verwendet:

```
osc-variant list meine-beispieldatei.osc
```

Mit der Option `--filter` kann die Ausgabe eingeschränkt werden.

Die Option `-v` sorgt dafür, dass eine Prüfsumme für Kataloge und Formulare berechnet und angezeigt wird.
Dadurch können inhaltliche Unterschiede bei identischer Revisionsnummer erkannt werden.

## tree

Zum Auflisten der Inhalte mit allen Abhängigkeiten, z.B. Daten- und Merkmalskataloge und bei Formularen wird der Befehl
`tree` verwendet, wie in folgendem Beispiel:

```
osc-variant tree meine-beispieldatei.osc
```

Abhängigkeiten werden je nach Art gekennzeichnet:

* `+`: Datenkatalog
* `-`: Merkmalskatalog
* `>`: Formularverweis
* `*`: Unterformular

Für Formularverweise und Unterformulare werden dabei die verwendeten Datenkataloge nicht erneut ausgegeben.

Achtung! Dies erzeugt eine sehr umfangreiche Ausgabe.

Mit der Option `--filter` kann auch hier die Ausgabe eingeschränkt werden.

Wie bei `list` sorgt auch hier die Option `-v` dafür,
dass die eine Prüfsumme für Kataloge und Formulare berechnet und angezeigt wird.

## modify

Zum Anpassen des Inhalts einer Datei siehe folgendes Beispiel:

```
osc-variant modify meine-beispieldatei.osc --profile ukw-profil.yml --output ukw-beispieldatei.osc
```

Die Parameter `--profile`, `--notices` und `--output` sind optional.

Mit dem ebenfalls optionalen Parameter `--interactve` oder `-i` können die Parameter zur kompakten Ausgabe, zum
Sortieren und dem Entfernen von Inhalten der Systembibliothek interaktiv gesetzt werden.

Ohne Profildatei wird die Datei lediglich eingelesen, Leerzeichen am Ende eines XML-Tags entfernt und wieder ausgegeben.

Ohne eine Angabe der Ausgabedatei wird auf die Standardausgabe ausgegeben.

## diff

Zum Vergleich zweier OSC-Dateien wird der Unterbefehl `diff` verwendet.
Der optionale Parameter `--strict` vergleicht auch den Inhalt der OSC-Datei.
Ohne diesen wird nur das Vorhandensein von Inhalten und die Revision verglichen.

Beispiele:

```
osc-variant diff meine-beispieldatei.osc andere-beispieldatei.osc
```

bzw.

```
osc-variant diff meine-beispieldatei.osc andere-beispieldatei.osc --strict
```

## check

Der Unterbefehl `check` prüft eine OSC- oder OSB-Datei (sofern unterstützt) auf bekannte Probleme und gibt eine Liste
mit erkannten Problemen aus.
In OSB-Dateien werden nur enthaltene OSC-Dateien mit Onkostar-Formularen geprüft.

Eine Liste mit bekannten Problemen wird mit `check --list` ausgegeben.

## export-notice-csv

Dies exportiert eine CSV-Datei mit den Ausfüllhinweisen für die angegebene OSC-Datei.

## bundle

Dieser Unterbefehl ermöglicht das Erstellen von OSC-Dateien als Bundle in einem Git-Repository und hat weitere
Unterbefehle.

# BUNDLE COMMANDS

Die lokale Kopie des Repositorys wird bei der ersten Verwendung automatisch erstellt und bei folgender Nutzung
von Bundles aktualisiert.

Hierdurch ist es möglich, weitere OSC-Dateien aus einem zentralen Repository zu beziehen.

## search
Suche nach einem Bundle

## info
Informationen zu einem Bundle

## list
Liste den Inhalt eines Bundles auf

## export
Exportiere ein Bundle als OSC-Datei

# AUTHOR

osc-variant wird entwickelt von Paul-Christian Volkmer.
**Quellcode:** `https://github.com/pcvolkmer/osc-variant`