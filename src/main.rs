use std::fs;

fn get_appid(dir: String)  {

}

fn grabids(modsdir: String) -> Vec<(String, String)> {
    let mut dir_list = Vec::new();
    
    // Iterate through all mods in mod directory
    if let Ok(entries) = fs::read_dir(modsdir) {
        // For each workshop id in mod dir
        for entry in entries {

            // Get list of Appid, Mod Names that exist inside of each entry.

            // Add the list we get back to a main list that we return

                // Create datastructure to hold both workshopid and modid
                // Parse the mod.info and grab the modid
                // Insert object into list of custom data type

                // if let Ok(entry) = entry {
                //     if let Some(name) = entry.file_name().to_str() {
                //         let workshopid = name.to_string();
                        
                //         let appid = get_appid()

                //         // Enter the current entry and iterate through mods dir
                //         // For each entry within 
                //     }
                // }
        }
    }
    dir_list

}

fn main() {
    // Enter data folder

    let idlist = grabids("data/mods".to_string());
    
        // Mod dir structure
            // mods\<workshopid#>\mods\<modname>\mod.info
                // Inside mod info there will be id=<modid>

        
            
        
    // Build string like:       "WorkshopItems=<workshopid>1;<workshopid>2;<workshopid>3;"
    // Build a second string:   "Mods=<modid>1;<modid>2;<modid>3;"

    // Replace the lines in data\<server.ini> with the built strings above

}
