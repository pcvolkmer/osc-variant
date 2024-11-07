## Checks

Die Anwendung kann mehrere Probleme in OSC-Dateien erkennen.

### Problem `2023-0001`: Unterformular mit Markierung 'hat Unterformulare'

Aktuell gibt es keine Unterformulare in Unterformularen, daher sollte dies nicht vorkommen.
Eine mögliche Ursache ist die Speicherung eines Unterformulars als Formular.

### Problem `2023-0002`: Formular hat keine Angabe zum Prozedurdatum

Formulare benötigen die Angabe des Prozedurdatums, anderenfalls führt dies zu Problemen in Onkostar.
Unterformulare können ein Prozedurdatum haben, müssen es aber nicht.
Eine mögliche Ursache ist die Speicherung eines Formulars als Unterformular.",

### Problem `2023-0003`: Leerzeichen am Ende der Plausibilitätsregel-Bezeichnung

Treten Leerzeichen am Ende der Plausibilitätsregel-Bezeichnung auf, führt dies zu Fehlern beim Import der OSC-Datei.
Das Problem wird beim Verwenden des Unterbefehls `modify` automatisch behoben und Leerzeichen entfernt.

### Problem `2023-0004`: Verweis auf noch nicht definiertes Formular

Wenn ein Formular einen Verweis auf ein anderes Formular enthält, das nicht vor diesem Formular in der OSC-Datei
definiert ist, wird der Formularverweis beim Import der OSC-Datei nicht übernommen.
Dies kann bei wechselseitiger Abhängigkeit zwischen zwei (Unter-)Formularen auftreten.

In diesem Fall kann ein erneuter/zweiter Import helfen, da das Onkostar in diesem Fall alle Formulare importiert hat und
der Formularverweis dann gespeichert werden kann.
