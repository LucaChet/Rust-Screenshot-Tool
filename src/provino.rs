fn main() {
    // ... Resto del codice ...

    let launcher = AppLauncher::with_window(main_window);

    // Avvia l'applicazione
    let initial_data = AppData {
        // Inizializza qui i dati del tuo programma
    };

    launcher
        .launch(initial_data)
        .expect("Failed to launch application");
}

fn build_ui() -> impl Widget<AppData> {
    // ... Resto del codice ...

    let save_button = Button::new("Save Image")
        .on_click(|ctx, _data: &mut AppData, _env| {
            // Crea un FileDialogOptions per il dialogo di salvataggio
            let file_dialog_options = FileDialogOptions {
                accept_label: "Save",
                allowed_types: vec![FileSpec::new("PNG", &["png"])],
                ..Default::default()
            };

            // Apre il dialogo di salvataggio
            let result = FileDialog::new().get_save_path(file_dialog_options);

            match result {
                Some(path) => {
                    // Qui puoi implementare la logica effettiva per salvare l'immagine
                    // Utilizzando il percorso 'path' selezionato dall'utente

                    // Ad esempio, puoi utilizzare la libreria image per creare un'immagine
                    // fittizia e salvarla come file PNG.
                    // Per questa operazione, dovrai adattare il codice al tuo caso d'uso specifico.

                    // Ecco un esempio di come potrebbe apparire la logica:
                    // (Nota: questo è solo un esempio di base e dovrai adattarlo al tuo caso d'uso)

                    use image::{ImageBuffer, Rgba};
                    let width = 100;
                    let height = 100;

                    // Crea un'immagine di esempio (100x100 pixel, colore rosso)
                    let mut img = ImageBuffer::new(width, height);

                    for (_, _, pixel) in img.enumerate_pixels_mut() {
                        *pixel = Rgba([255, 0, 0, 255]);
                    }

                    // Salva l'immagine nel percorso selezionato dall'utente
                    if let Err(err) = img.save(path) {
                        eprintln!("Errore nel salvataggio dell'immagine: {:?}", err);
                    } else {
                        println!("Immagine salvata con successo in: {:?}", path);
                    }
                }
                None => {
                    // L'utente ha annullato il dialogo di salvataggio
                    println!("Save dialog canceled");
                }
            }
        });

    // Costruisci l'interfaccia utente
    // ... Resto del codice ...
}
