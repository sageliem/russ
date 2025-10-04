use ratatui::{
    crossterm::event::{KeyEvent, MouseEvent},
    layout::Rect
};

pub trait Component {
    fn init(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
    
    fn handle_event(&mut self, event: Option<Event>) -> {
        
    }
    
    fn handle_key_event(&mut self, key: KeyEvent) {
        
    }
    
    fn handle_mouse_event(&mut self, mouse: MouseEvent) {
        
    }
    
    fn update(&mut self, action: Action) -> Action {
        
    }
    
    fn render(&mut self, f: &mut Frame, rect: Rect);
}