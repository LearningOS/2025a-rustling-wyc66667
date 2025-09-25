pub fn animal_habitat(animal: &str) -> &'static str {
    let identifier = if animal == "crab" { 1 } 
                     else if animal == "gopher" { 2 } 
                     else if animal == "snake" { 3 } 
                     else { 0 };
                     
    let habitat = if identifier == 1 { "Beach" } 
                  else if identifier == 2 { "Burrow" } 
                  else if identifier == 3 { "Desert" } 
                  else { "Unknown" };
                  
    habitat
}