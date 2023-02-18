use std::fs;

fn grabids(modsdir: String) -> Vec<(String, String)> {
    let mut dir_list = Vec::new();
    if let Ok(entries) = fs::read_dir(modsdir) {
        for entry in entries {
            if let Ok(entry) = entry {
                if let Some(name) = entry.file_name().to_str() {
                    dir_list.push(name.to_string());
                }
            }
        }
    }
    dir_list

}


fn main() {
    // Enter data folder

    let idlist = grabids("data/mods");
    // Iterate through all mods in mod directory
        // Mod dir structure
            // mods\<workshopid#>\mods\<modname>\mod.info
                // Inside mod info there will be id=<modid>

        // For each workshop id in mod dir
            // Create datastructure to hold both workshopid and modid
            // Parse the mod.info and grab the modid
            // Insert object into list of custom data type
        
    // Build string like:       "WorkshopItems=<workshopid>1;<workshopid>2;<workshopid>3;"
    // Build a second string:   "Mods=<modid>1;<modid>2;<modid>3;"

    // Replace the lines in data\<server.ini> with the built strings above

}
