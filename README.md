# Backup di emergenza - PDS Gruppo 2

## Esecuzione

1. Il programma `eb-rs` si presenta come un file eseguibile. All'apertura presenta una finestra in cui è possibile configurare i seguenti parametri (entrambi salvati in `config.conf`, l'apposito file di configurazione):

   - `Lista delle directory sorgente`: contiene tutti i percorsi selezionati che verranno copiati nella periferica esterna al momento della richiesta del backup.
   - `Dispositivo di destinazione`: contiene la periferica nella quale verranno copiate le directory sorgente e il loro contenuto.

2. Una volta premuto il pulsante `Start emergency backup` si avvia un processo in background che rimane in attesa che l'utente disegni due rettangoli consecutivi passando con il mouse attraverso tutti gli angoli del display.

   Inoltre viene creata una directory `emergency-backup-data` contenente due file:

   - `config.conf`: contiene informazioni riguardo la periferica selezionata e i percorsi scelti per il salvataggio.
   - `logger.log`: contiene informazioni circa l'esecuzione del processo, la percentuale di utilizzo della CPU ed eventuali errori.
   - `eb-rs-launcher`: contiene l'applicativo necessario per far ripartire il processo all'avvio del sistema, presente solo nella versione per **MacOS**.

3. Dopo l'esecuzioni delle azioni citate sopra apparirà una `Warning window` con un conto alla rovescia di _15 secondi_. Alla fine del tempo, se non si preme cancella, partirà un feedback sonoro e si avranno _60 secondi_ per disegnare un ulteriore rettangolo per confermare.

4. Alla conferma verranno copiate le `Directory sorgente` nel `Dispositivo di destinazione` e verrà generato un file `backup.log` in cui saranno presenti il messaggio **"Backup successful"** e 2 parametri:

   - `Backup size`: la dimensione totale del contenuto copiato (in bytes).
   - `Time elapsed`: il tempo trascorso dall'inizio del backup allaa sua fine (in secondi).

5. Il processo rimarrà in background anche a partire dal primo avvio del computer successivo alla prima esecuzione del programma.

## Funzioni principali

- `background_process_management.rs`

  - `start_background_process`

    Questa funzione fa partire il processo in background.

  - `start_warning_process`

    Questa funzione avvia il processo della finestra di conferma.

  - `set_auto_launch`

    Questa funzione fa in modo che il processo in background parta all'accensione del sistema.

  - `background_job`

    Questa fuzione si occupa di far funzionare il processo in background. Fa cominciare il processo di tracciamento della CPU, poi fa partire un loop che controlla periodicamente se i rettangoli vengono eseguiti sullo schermo. Si occupa anche di far partire i suoni e la finestra di conferma quando necessario. Infine fa partire la copia delle directory sul dispositivo esterno selezionato.

- `external_devices_management.rs`

  - `get_external_devices`

    Questa funzione prepara un vettore contenente i dispositivi rimovibili visualizzati dal sistema.

- `gui.rs`

  - `start_config_window`

    Questa funzione crea la finestra di configurazione in cui è possibile scegliere il device esterno, su cui effettuare il backup, attraverso un menù a tendina. È presente poi un file dialog che permette di scegliere fino a 5 percorsi di cartelle da salvare.

  - `start_warning_window`

    Questa funzione crea la finestra di conferma e un countdown, dopo il quale, verrà avviata la seconda fase del backup. È presente un tasto `cancel` che permette di interrompere il backup in questa fase.

- `mouse_pattern_recognition.rs`

  - `check_rectangle_pattern_with_duration`

    Questa funziona campiona la posizione del cursore del mouse ogni 50ms, se il cursore viene posizionato in uno degli angoli del diplay, la funzione, passa allo step successivo controllando, sempre ogni 50ms, se il mouse passa per i quattro angoli del display, tornando infine nel punto da cui era partito. Il rettangolo si può tracciare sia in senso orario che in senso antiorario. È inoltre possibile passare come parametro un timer dopo il quale la funzione terminerà a prescindere dalle azioni dell'utente.

- `utility.rs`

  - `create_countdown_timer`

    Questa funzione crea un timer restituendo un handle del thread che aggiorna il contatore e il tempo rimasto. È possibile passare come parametro, oltre alla durata del timer, un flag che indica che si vuole attivare il feedback sonoro durante il countdown.

  - `start_cpu_tracking`

    Questa funzione si occupa di tracciare il consumo della CPU, avviando un apposito thread che scrive sul file `logger.log`, ogni 2 minuti, la percetuale di CPU utilizzata insieme ad informazioni sulla data e ora in cui viene tracciato il consumo.

  - `kill_old_background_job`

    Questa funzione si occupa di terminare eventuali processi `eb-rs` in background nel momento in cui l'utente va ad effettuare una riconfigurazione dei percorsi da salvare o del device in cui effettuare il backup.

  - `sound_hint`, `sound_first_confirm`, `sound_success`, `sound_alert`

    Queste funzioni si occupano di generare i feedback sonori.

## Strutture dati principali

- `gui.rs`

  - `App`

    Questa struttura è una _struct_ che contiene le informazioni necessarie per il funzionamento della finestra di configurazione e della finestra di conferma.

    I campi sono:

    - `status`: indica il tipo di finestra momentaneamente aperta.
    - `picked_path`: rappresenta il vettore di percorsi scelti, ciascuno associato ad un _flag_ che indica se è stato premuto il tasto `-` per rimuovere il percorso dal vettore di percorsi scelti.
    - `picked_device`: rappresenta un _Option_ che, se selezionato, contiene il device scelto per effettuare il backup.
    - `time_left`: rappresenta il tempo rimasto prima che la finestra corrente si chiuda, utile nella finestra di conferma per la seconda fase del backup.

  - `WindowStatus`

    Questa struttura è una _enum_ che indica il tipo di finestra momentaneamente aperta, modificando il comportamento della funzione `update` (che aggiorna il contenuto della finestra), in base al proprio valore.

    I possibili valori sono:

    - `ConfigWindow`: finestra di configurazione.
    - `WarningWindow`: finestra di conferma.

## Installazione

La procedura di installazione andrà a compilare l'eseguibile per poi copiarlo nel **Desktop**. Dopo aver clonato il repository, eseguire i comandi elencati nella **root directory** del repository stesso.

Si consiglia inoltre di non spostare l'eseguibile dopo aver eseguito la configurazione.

### Installazione MacOS e Linux

```
cargo build --release
mv ./target/release/eb-rs ~/Desktop
```

#### Requisiti Linux (Debian-based)

Si consiglia di installare i seguenti pacchetti prima di procedere:

```
sudo apt install build-essential
sudo apt install gcc
sudo apt install libxdo-dev
sudo apt install libxcb1 libxrandr2
sudo apt install libasound2-dev
```

### Installazione Windows

Si consiglia di eseguire tali comandi attraverso la powershell nel caso di **Windows**.

```
cargo build --release
mv ./target/release/eb-rs.exe ~/Desktop
```
