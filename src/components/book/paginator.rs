use ratatui::prelude::Rect;
use super::text_processor::TextProcessor;

pub struct Paginator {
    current_page: usize,
}

impl Paginator {
    pub fn new() -> Self {
        Self { current_page: 0 }
    }

    pub fn current_page(&self) -> usize {
        self.current_page
    }

    pub fn next_page(&mut self, total_pages: usize) {
        if self.current_page + 1 < total_pages {
            self.current_page += 1;
        }
    }

    pub fn previous_page(&mut self) {
        if self.current_page > 0 {
            self.current_page -= 1;
        }
    }

    pub fn split_text_into_pages(&self, text: &str, area: Rect) -> Vec<Vec<String>> {
        let max_width = TextProcessor::calculate_max_width_for_page(area);
        let available_lines = TextProcessor::calculate_lines_per_page(area);
        let wrapped_lines = TextProcessor::wrap_text(text, max_width);

        // Split lines into pages (each page contains left + right content)
        let lines_per_double_page = available_lines * 2; // Both left and right pages
        let mut pages = Vec::new();
        
        for chunk in wrapped_lines.chunks(lines_per_double_page) {
            pages.push(chunk.to_vec());
        }
        
        if pages.is_empty() {
            pages.push(vec!["".to_string()]);
        }
        
        pages
    }

    pub fn get_current_page_content(&self, text: &str, area: Rect) -> (String, String) {
        let pages = self.split_text_into_pages(text, area);
        let available_lines = TextProcessor::calculate_lines_per_page(area);
        
        if self.current_page < pages.len() {
            let current_content = &pages[self.current_page];
            
            let left_content: String = current_content
                .iter()
                .take(available_lines)
                .cloned()
                .collect::<Vec<_>>()
                .join("\n");
                
            let right_content: String = current_content
                .iter()
                .skip(available_lines)
                .take(available_lines)
                .cloned()
                .collect::<Vec<_>>()
                .join("\n");
                
            (left_content, right_content)
        } else {
            ("".to_string(), "".to_string())
        }
    }

    pub fn total_pages(&self, text: &str, area: Rect) -> usize {
        let pages = self.split_text_into_pages(text, area);
        pages.len()
    }
}