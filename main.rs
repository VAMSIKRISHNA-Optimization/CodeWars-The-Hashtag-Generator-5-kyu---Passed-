pub fn generate_hashtag(s: &str) -> Option<String> 
{
    if s.is_empty() { return None; }
    
    let mut is_next_capital = true;
    
    let Title_Case: String = s.chars()
                             .enumerate().map(|(ind, c)| 
                            {
                                if c.is_whitespace()
                                {
                                    is_next_capital = true;
                                    c.to_string()
                                }
                                else if is_next_capital 
                                {
                                    is_next_capital = false;
                                    c.to_uppercase().to_string()
                                } 
                                else 
                                {
                                    c.to_lowercase().to_string()
                                }
                            }).collect();
    
    let Hastag_String = format!("#{}", Title_Case.chars().filter(|c| !c.is_whitespace()).collect::<String>());
    
    if Hastag_String.len() <= 140 { Some(Hastag_String) } else { None }
}
