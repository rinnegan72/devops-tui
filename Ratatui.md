
# Ratutui example app

This is the sample app to create a Counter app
Below is the rough outline of the code required
to create the app


```
use statements for crosterm and ratatui

// This structure defines the state of the application
struct <Structure>


// This main function call calls the ratatui run function
fn main() {
  ratatui::run
}

// This is the main loop of the app untill the user exits the application
impl <Structure>{

  fn run(){
      terminal.draw
  }

  fn draw(){
      frame.render_widget()
   }

  // Any events from crossterm are handled here
  fn handle_events(){
      self.handle_key_event(key_event)
  }

  // This function handles keyboard events
  fn handle_key_event(key_event){
      KeyCode::<key> => self.<function>
  }

  // This is an example of a function that modifies application state
  fn <function>{
      <logic>
  }
}

//This is the function responsible for rendering the TUI ui
impl Widget for &<Structure> {
  fn render(){
    <Apearance config code>
  }
}

```
