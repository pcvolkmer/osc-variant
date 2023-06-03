# OSC-Variant

Anwendung zum Anpassen einer OSC-Datei an einen Standort.

## Funktion

Diese Anwendung passt listet die Inhalte eine OSC-Datei auf oder passt sie so an, dass (standortbezogene) Formularvarianten für Formularverweise
verwendet werden.

Hierzu wird die Datei deserialisiert, die entsprechenden Formularfelder ermittelt und die Formularvariante
sowie die Anzeige anhand eines Profils angepasst.

Wird in einer OSC-Datei eine noch nicht bekannte Eigenschaft erkannt, wird die weitere Bearbeitung abgebrochen um keine
unvollständigen Ausgabedateien zu erzeugen.

### Beispiele

Zum Auflisten der Inhalte einer Datei wird folgender Befehl verwendet:

```
osc-variant list meine-beispieldatei.osc
```

Zum Anpassen des Inhalts einer Datei:

```
osc-variant modify meine-beispieldatei.osc --profile ukw-profil.yml --output ukw-beispieldatei.osc
```

Die Parameter `--profile` und `--output` sind optional.

Ohne Profildatei wird die Datei lediglich eingelesen, Leerzeichen am Ende eines XML-Tags entfernt und wieder ausgegeben.

Ohne eine Angabe der Ausgabedatei wird auf die Standardausgabe ausgegeben.

## Profile

Zum Erstellen von Varianten einer OSC-Datei wird eine Profildatei im YAML-Format verwendet.

In ihr sind die durchzuführenden Änderungen definiert. Eine Profildatei hat die folgende Form:

```
forms:
  - name: "ExampleForm"
    form_references:
      - name: "ref_first_mtb"
        referenced_data_form: "Formularverweis.Variante"
        anzeige_auswahl: "Referenziertes Formular vom: {Datum}"
        anzeige: "Datum im referenzierten Formular: {Datum}"
```

Hierzu wird die Anwendung angewiesen im Formular "ExampleForm" den Formularverweis im Formularfeld "ref_first_mtb":

* den Verweis auf das Formular "Formularverweis.Variante" zu setzen
* die Anzeige im Auswahlmenü auf "Referenziertes Formular vom: {Datum}" zu setzen
* die Anzeige unterhalb des Auswahlmenüs auf "Datum im referenzierten Formular: {Datum}" zu setzen

und dabei die vorhandenen Angaben für den Formularverweis zu ersetzen.

Die Angaben für `referenced_data_form`, `anzeige_auswahl` und `anzeige` sind optional.
Wird keine Angabe gemacht, wird der bestehende Wert beibehalten.

Es können beliebig viele Formulare mit beliebig vielen Änderungen zu Formularverweisen in einer Profildatei
hinterlegt werden, jedoch ist mindestens eine Angabe zu einem Formularfeld erforderlich.
