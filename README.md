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

Zum Auflisten der Inhalte einer Datei wird folgender Befehl verwendet:

```
osc-variant list meine-beispieldatei.osc
```

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

Zum Anpassen des Inhalts einer Datei:

```
osc-variant modify meine-beispieldatei.osc --profile ukw-profil.yml --output ukw-beispieldatei.osc
```

Die Parameter `--profile` und `--output` sind optional.

Ohne Profildatei wird die Datei lediglich eingelesen, Leerzeichen am Ende eines XML-Tags entfernt und wieder ausgegeben.

Ohne eine Angabe der Ausgabedatei wird auf die Standardausgabe ausgegeben.

#### Kompakte Ausgabe

OSC-Dateien sind XML-Dateien. Diese Anwendung ermöglicht optional die Ausgabe als kompaktere XML-Datei ohne Zeilenumbrüche.
Hierzu ist die Option `--compact` vorgesehen. Es können, je nach Datei, bis zu 30% eingespart werden.

#### Sortierung

Bei der Auflistung der Inhalte, kann die Option `--sorted` dazu verwendet werden, die angezeigten Einträge alphabetisch zu sortieren.
Die Sortierung erfolgt dabei nach Namen des Katalogs oder des Formulars.

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
* den Code zur Ausführung "nach Aktualisierung" für das Formularfeld auf die angegebene, mehrzeilige Zeichenkette anzupassen

und dabei die vorhandenen Angaben für den Formularverweis zu ersetzen.

Die Angaben für `referenced_data_form`, `anzeige_auswahl`, `anzeige` und `scripts_code` sind optional.
Wird keine Angabe gemacht, wird der bestehende Wert beibehalten.

**Achtung!** Diese Anwendung überprüft keine Scripts und verwendet angegebene Scripts als "valid" im resultierenden OSC-File.

Zudem kann die Menükategorie angepasst werden.
Die Angabe einer `menu_category` ist fakultativ.
Wird sie angeben, sind die Felder `name`, `position` und `column` verpflichtend.

Es können beliebig viele Formulare mit beliebig vielen Änderungen zu Formularverweisen in einer Profildatei
hinterlegt werden, jedoch ist mindestens eine Angabe zu einem Formularfeld erforderlich.

Beispiele für eine Profildatei sind unter [`examples/`](examples/) zu finden.
