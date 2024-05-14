# OSC-Variant

Anwendung zum Anpassen einer OSC-Datei an einen Standort.

## Funktion

Diese Anwendung passt die Inhalte eine OSC-Datei an, sodass (standortbezogene) Formularvarianten für Formularverweise
verwendet werden.
Weiterhin ist das Auflisten der Inhalte einer OSC-Datei möglich.

Hierzu wird die Datei deserialisiert, die entsprechenden Formularfelder ermittelt und die Formularvariante
sowie die Anzeige anhand eines Profils angepasst.

Wird in einer OSC-Datei eine noch nicht bekannte Eigenschaft erkannt, wird die weitere Bearbeitung abgebrochen, um keine
unvollständigen Ausgabedateien zu erzeugen.

### Beispiele

Die folgenden Unterbefehle sind verfügbar

```
Anwendung zum Anpassen einer OSC-Datei an einen Standort

Usage: osc-variant <COMMAND>

Commands:
sha256sum  Berechne SHA256 Prüfsumme für die angegebene Datei
list       Zeigt alle enthaltenen Kataloge und Formulare mit Revision an.
tree       Zeigt Kataloge und Formulare mit Revision und Abhängigkeiten an.
modify     Modifiziert die angegebene Datei anhand der Profildatei
diff       Vergleiche zwei Dateien anhand der Revision der enthaltenen Inhalte
check      Prüfe eine OSC-Datei auf bekannte Problemen
help       Print this message or the help of the given subcommand(s)

Options:
-h, --help     Print help
-V, --version  Print version
```

#### Unterbefehl `sha256sum`

Das Berechnen der SHA256 Prüfsumme ist mit dem Unterbefehl `sha256sum` auch unter Windows einfach möglich
und erzeugt eine Ausgabe analog dem Befehl auf Linux-Systemen:

```
osc-variant sha256sum meine-beispieldatei.osc
```

#### Unterbefehl `list`

Zum Auflisten der Inhalte einer Datei wird folgender Befehl verwendet:

```
osc-variant list meine-beispieldatei.osc
```

Mit der Option `--filter` kann die Ausgabe eingeschränkt werden.

*Bei Verwendung der OSB-Funktionalität kann die Eingabe eines Passworts erforderlich sein.*

#### Unterbefehl `tree`

Zum Auflisten der Inhalte mit allen Abhängigkeiten, z.B. Daten- und Merkmalskataloge und bei Formularen wird der Befehl
`tree` verwendet:

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

#### Unterbefehl `diff`

Zum Vergleich zweier OSC-Dateien wird der Unterbefehl `diff` verwendet.
Der optionale Parameter `--strict` vergleicht auch den Inhalt der OSC-Datei.
Ohne diesen wird nur das Vorhandensein von Inhalten und die Revision verglichen.

```
osc-variant diff meine-beispieldatei.osc andere-beispieldatei.osc
```

bzw.

```
osc-variant diff meine-beispieldatei.osc andere-beispieldatei.osc --strict
```

#### Unterbefehl `modify`

Zum Anpassen des Inhalts einer Datei:

```
osc-variant modify meine-beispieldatei.osc --profile ukw-profil.yml --output ukw-beispieldatei.osc
```

Die Parameter `--profile` und `--output` sind optional.
Mit dem ebenfalls optionalen Parameter `--interactve` oder `-i` können die Parameter zur kompakten Ausgabe, zum
Sortieren
und dem Entfernen von Inhalten der Systembibliothek interaktiv gesetzt werden.

Ohne Profildatei wird die Datei lediglich eingelesen, Leerzeichen am Ende eines XML-Tags entfernt und wieder ausgegeben.

Ohne eine Angabe der Ausgabedatei wird auf die Standardausgabe ausgegeben.

##### Enthaltene Profile

Die im Ordner [`examples/`](/examples) enthaltenen Profile für Standorte sind in der ausführbaren Anwendung enthalten
und die Dateien müssen nicht explizit als Datei vorliegen:

* `--profile examples/dnpm-ukm.yml` => `--profile UKM` für **Marburg**
* `--profile examples/dnpm-ukr.yml` => `--profile UKR` für **Regensburg**
* `--profile examples/dnpm-ukw.yml` => `--profile UKW` für **Würzburg**
* `--profile examples/dnpm-umg.yml` => `--profile UMG` für **Göttingen**

#### Unterbefehl `unzip-osb`

Ab Version 0.6.0 ist die Anwendung zudem in der Lage, die für eine Aktualisierung der OS-Bibliothek genutzten
OSB-Dateien zu entpacken:

```
osc-variant unzip-osb OSBIB-6.10.osb
```

Dieser Befehl kennt die beiden optionalen Parameter

* `-d`: Optionale Angabe des Zielverzeichnisses. Wenn keine Angabe vorhanden ist, wird das aktuelle Verzeichnis
  verwendet.
* `-p`/`--password`: Optionale Angabe des Passworts zum Entpacken der OSB-Datei.

Dies setzt voraus, dass die Anwendung mit dem Feature `unzip-osb` compiliert wurde.

#### Unterbefehl `check`

Der Unterbefehl `check` prüft eine OSC- oder OSB-Datei (sofern unterstützt) auf bekannte Probleme und gibt eine Liste
mit erkannten Problemen aus.
In OSB-Dateien werden nur enthaltene OSC-Dateien mit Onkostar-Formularen geprüft.

Eine Liste mit bekannten Problemen wird mit `check --list` ausgegeben.

*Bei Verwendung der OSB-Funktionalität kann die Eingabe eines Passworts erforderlich sein.*

#### Kompakte Ausgabe

OSC-Dateien sind XML-Dateien. Diese Anwendung ermöglicht optional die Ausgabe als kompaktere XML-Datei ohne
Zeilenumbrüche.
Hierzu ist die Option `--compact` vorgesehen. Es können, je nach Datei, bis zu 30 % eingespart werden.

#### Filter

Bei der Auflistung von Inhalten ist es möglich, die Anzeige für die Unterbefehle `list` und `tree` anhand des Namens zu
filtern.
Hierzu ist die Option `--filter=` vorgesehen.
Wird diese angewendet, werden nur Inhalte angezeigt, deren Name die angegebene Zeichenkette beinhalten.

#### Sortierung

Bei der Auflistung der Inhalte, kann die Option `--sorted` dazu verwendet werden, die angezeigten Einträge alphabetisch
zu sortieren.
Die Sortierung erfolgt dabei nach Namen des Katalogs oder des Formulars.

Beim Modifizieren der Inhalte kann ebenfalls die Option `--sorted` dazu verwendet werden, die Einträge im Anschluss an
die Modifikation
nach Namen und für Formulare der Abhängigkeit von Formularverweisen und Unterformularen zu sortieren.

Formulare, die von anderen Formularen in einem Formularverweis oder als Unterformular verwendet werden, werden dabei
weiter oben angeordnet,
da Onkostar einen Formularimport sequenziell, ohne Berücksichtigung von Abhängigkeiten, durchführt.

Dies erlaubt eine konsistente Reihenfolge der Einträge, wodurch ein direkter Vergleich mit Vorversionen ermöglicht wird.

*Die Einteilung in Formualre und Unterformualare wird hierdurch nicht angepasst.*

##### Entfernen von Inhalten der Systembibliothek bei Modifikation

Mit der die experimentelle Option `--strip` ist es möglich, die in der OSC-Datei enthaltenen und beim Import nicht
genutzten Inhalte aus der Systembibliothek zu entfernen.

Hierbei werden alle Inhalte entfernt, die im Ordner "ONKOSTAR Bibliothek" enthalten sind, beim Import jedoch ignoriert
werden.

## Profile

Zum Erstellen von Varianten einer OSC-Datei wird eine Profildatei im YAML-Format verwendet.

In ihr sind die durchzuführenden Änderungen definiert. Eine Profildatei hat die folgende Form:

```
forms:
  - name: "ExampleForm"
    form_field:
      - name: "formularfeld"
        hide: true
      - name: "otherformfield"
        default_value: "T"
    form_references:
      - name: "ref_first_mtb"
        referenced_data_form: "Formularverweis.Variante"
        anzeige_auswahl: "Referenziertes Formular vom: {Datum}"
        anzeige: "Datum im referenzierten Formular: {Datum}"
        scripts_code: |
          // Beispielcode
          console.log(getFieldValue('ref_first_mtb'));
    menu_category:
      name: Beispielformulare
      position: 1.0
      column: 0
```

Hierzu wird die Anwendung angewiesen im Formular "ExampleForm" den Formularverweis im Formularfeld "ref_first_mtb":

* den Verweis auf das Formular "Formularverweis.Variante" zu setzen
* die Anzeige im Auswahlmenü auf "Referenziertes Formular vom: {Datum}" zu setzen
* die Anzeige unterhalb des Auswahlmenüs auf "Datum im referenzierten Formular: {Datum}" zu setzen
* den Code zur Ausführung "nach Aktualisierung" für das Formularfeld auf die angegebene, mehrzeilige Zeichenkette
  anzupassen

und dabei die vorhandenen Angaben für den Formularverweis zu ersetzen.

Die Angaben für `referenced_data_form`, `anzeige_auswahl`, `anzeige` und `scripts_code` sind optional.
Wird keine Angabe gemacht, wird der bestehende Wert beibehalten.

Zudem wird im Formular "ExampleForm" das Formularfeld "formularfeld" ausgeblendet, indem der Filter auf "false" gesetzt
wird.
Dadurch wird das Formularfeld nie angezeigt.
Ein zuvor bestehender Filter wird ersetzt.
Weiterhin wird die Eigenschaft "Speichern" des Formularfelds auf "Immer speichern" gesetzt um sicherzustellen, dass
zuvor
enthaltene Daten weiterhin gespeichert bleiben und werden, auch wenn das Formularfeld nicht sichtbar ist.

Der Standardwert des Feldes `otherformfield` ist nun auf `T` gesetzt.
Zum Löschen eines Standardwerts ist `""` anzugeben.

**Achtung!** Diese Anwendung überprüft keine Scripts und verwendet angegebene Scripts als "valid" im resultierenden
OSC-File.

Zudem kann die Menükategorie angepasst werden.
Die Angabe einer `menu_category` ist fakultativ.
Wird sie angeben, sind die Felder `name`, `position` und `column` verpflichtend.

Es können beliebig viele Formulare mit beliebig vielen Änderungen zu Formularverweisen in einer Profildatei
hinterlegt werden, jedoch ist mindestens eine Angabe zu einem Formularfeld erforderlich.

Beispiele für eine Profildatei sind unter [`examples/`](/examples) zu finden.
