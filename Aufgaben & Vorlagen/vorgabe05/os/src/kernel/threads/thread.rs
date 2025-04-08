
impl Thread {

    // Thread fuer eine App anlegen
    // Hier wird der Code & BSS eingemappt & ein Thread mit eigenem Adressraum erzeugt
    pub fn new_app_thread(app: AppRegion)-> Box<Thread> {
        let app_thread = Self::new(app.entry, false); 

        // App-Image mappen
        pages::pg_mmap_user_app(app_thread.pml4_addr, app);

        app_thread
    }

    // Neuen Thread anlegen
    pub fn new(myentry: extern "C" fn(), kernel_thread: bool) -> Box<Thread> {
			
			 /* 
			  * Hier muss Code eingefuegt werden. 
			  * 
			  * Die Einstiegsfunktion von User-Threads wird direkt auf 1 TiB gesetzt
			  */
			  
    }
    
    
    fn switch_to_usermode(&mut self) {

			 /* 
			  * Hier muss Code eingefuegt werden. 
			  * 
			  * 'kickoff_user_addr' kann nicht mehr verwendet werden
			  * => Einstiegsfunktion des Threads direkt auf 
			  *    dem Stack als Ruecksprungadresse eintragen
			  */
			  
    }

}
