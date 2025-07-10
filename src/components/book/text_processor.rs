use ratatui::prelude::Rect;

pub struct TextProcessor;

impl TextProcessor {
    pub fn wrap_text(text: &str, max_width: usize) -> Vec<String> {
        text.lines()
            .flat_map(|line| {
                if line.trim().is_empty() {
                    vec![String::new()]
                } else {
                    Self::wrap_line(line, max_width)
                }
            })
            .collect()
    }

    fn wrap_line(line: &str, max_width: usize) -> Vec<String> {
        let words: Vec<&str> = line.split_whitespace().collect();
        let mut wrapped = Vec::new();
        let mut current_line = String::new();

        for word in words {
            if current_line.len() + word.len() + 1 <= max_width {
                if !current_line.is_empty() {
                    current_line.push(' ');
                }
                current_line.push_str(word);
            } else {
                if !current_line.is_empty() {
                    wrapped.push(current_line);
                    current_line = String::new();
                }
                current_line.push_str(word);
            }
        }
        
        if !current_line.is_empty() {
            wrapped.push(current_line);
        }
        
        if wrapped.is_empty() {
            wrapped.push(String::new());
        }
        
        wrapped
    }

    pub fn calculate_max_width_for_page(area: Rect) -> usize {
        // Each page gets 50% of width, minus borders (2 chars) and some padding
        ((area.width as usize / 2).saturating_sub(4)).max(20)
    }

    pub fn calculate_lines_per_page(area: Rect) -> usize {
        // Account for borders (top and bottom) and title at bottom
        (area.height as usize).saturating_sub(3)
    }
}