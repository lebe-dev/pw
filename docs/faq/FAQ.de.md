# FAQ

**1. Wo werden Geheimnisse gespeichert?**

Verschlüsselt im Arbeitsspeicher (RAM) der Anwendung.

**2. Wie werden Geheimnisse geschützt?**

Deine Daten werden auf der Client-Seite (im Browser) mit AES-256 und einem 32-Byte-Schlüssel verschlüsselt. Anschließend erhältst du eine spezielle URL zum Geheimnis.
Die Server-Seite kennt den ursprünglichen Inhalt nicht und speichert nur die verschlüsselten Daten (im RAM).

Wenn du eine Geheimnis-URL öffnest, lädt der Client (Browser) die verschlüsselten Daten vom Server und entschlüsselt sie mit dem starken Schlüssel, der in der URL kodiert ist.

Geheimnis-URLs haben eine begrenzte Lebensdauer (eine Stunde, zwei Stunden oder maximal einen Tag).

**3. Welcher Verschlüsselungsalgorithmus wird verwendet?**

AES-256 mit einer Schlüssellänge von 32 Byte.

**4. Kann man Einmal-Links über Messenger versenden?**

Ja, der Link läuft nicht ab.

**5. Wie viele Ressourcen benötigt die Anwendung?**

Docker-Container verbrauchen ca. 12 MB RAM.

```shell
$ docker stats --no-stream

CONTAINER ID   NAME              CPU %     MEM USAGE / LIMIT    MEM %     NET I/O           BLOCK I/O      PIDS
94d9d31ddf83   pw-cache          1.27%     7.977MiB / 1.69GiB   0.46%     16.2kB / 0B       0B / 0B        6
0d3c9c52165a   pw                0.00%     4.082MiB / 1.69GiB   0.24%     63.2kB / 224kB    0B / 0B        2
```

Die Image-Größe beträgt ca. 30 MB:

```shell
$ docker images

REPOSITORY                                                TAG                IMAGE ID       CREATED          SIZE
tinyops/pw                                                1.6.0              ee1c473b8920   30 minutes ago   29.7MB
```

6. Ich möchte mein eigenes Image erstellen. Was muss ich tun?

Besuche das [tutorial](../BUILD.md).
