use std::any::Any;

use sdl3_sys::{error::SDL_GetError, events::{SDL_Event, SDL_EventType, SDL_PollEvent, SDL_EVENT_MOUSE_MOTION, SDL_EVENT_QUIT, SDL_EVENT_WINDOW_RESIZED, SDL_EVENT_WINDOW_SHOWN}, init::{SDL_Init, SDL_Quit, SDL_INIT_VIDEO}, mouse::SDL_GetMouseState, rect::{SDL_FRect, SDL_Rect}, render::{SDL_CreateRenderer, SDL_DestroyRenderer, SDL_RenderClear, SDL_RenderFillRect, SDL_RenderFillRects, SDL_RenderPresent, SDL_RenderRect, SDL_Renderer, SDL_SetRenderDrawColor, SDL_SetRenderViewport}, video::{SDL_CreateWindow, SDL_DestroyWindow, SDL_GetWindowSize, SDL_ShowWindow, SDL_Window, SDL_WINDOW_MODAL, SDL_WINDOW_RESIZABLE}};

use crate::user_event::UserEvent;
use crate::objects::Quad2D;


pub struct Context {

}

impl Context {
  pub fn new() -> Result<Self, String> {

    unsafe {
      if !SDL_Init(SDL_INIT_VIDEO as u32) {
        let err = SDL_GetError();
        Err(err.cast::<char>().as_ref().unwrap().to_string())
      }
      else {
        Ok(Context {  })
      }
    }
  }

  pub fn create_window(&mut self, name : impl Into<String>, width : i32, height : i32) -> Window {
    Window { handle : unsafe { SDL_CreateWindow(name.into().as_ptr() as _, width, height, SDL_WINDOW_RESIZABLE)} }
  }

  pub fn create_renderer(&mut self, window : &mut Window) -> Renderer {
    let renderer = unsafe { SDL_CreateRenderer(window.handle, "\0".as_ptr() as _) };

    Renderer { renderer }
  }
}

impl Drop for Context {
  fn drop(&mut self) {
    unsafe { SDL_Quit() }
  }
}


pub struct Window {
  handle : * mut SDL_Window
}

impl Window {

  pub fn show(&mut self) {
    unsafe  { SDL_ShowWindow(self.handle); }
  }
  
  pub fn poll_event(&mut self) -> Option<UserEvent> {
    let mut event = SDL_Event::default();
    unsafe {
      if !SDL_PollEvent(&mut event) {
        return None;
      }
    }

    unsafe {
      match SDL_EventType(event.r#type) {
        SDL_EVENT_QUIT => Some(UserEvent::Quit),
        SDL_EVENT_MOUSE_MOTION => {
          let mut x: f32 = 0.0;
          let mut y: f32 = 0.0;
          SDL_GetMouseState(&mut x as _, &mut y as _);
          Some(UserEvent::MouseMoved(x as f32, y as f32))
        }
        SDL_EVENT_WINDOW_RESIZED => {
          let size = self.get_size();
          Some(UserEvent::Resize(size.0, size.1))  
        }
        SDL_EVENT_WINDOW_SHOWN => { 
          let size = self.get_size();
          Some(UserEvent::Resize(size.0, size.1))
        }
        _ => None,
      }
    }
  }

  pub fn get_size(&mut self) -> (u32, u32) {
    let mut w: i32 = 0;
    let mut h: i32 = 0;
    unsafe { 
      SDL_GetWindowSize(self.handle, &mut w as _, &mut h as _); 
    };
    return (w as u32, h as u32)
  }
}

impl Drop for Window {
  fn drop(&mut self) {
    unsafe { SDL_DestroyWindow(self.handle) }
  }
}

pub struct Renderer {

  renderer : *mut SDL_Renderer
}
impl Renderer {

  pub fn resize(&mut self, width : u32, height : u32) {
    unsafe { SDL_SetRenderViewport(self.renderer, &mut SDL_Rect { x: 0, y: 0, w: width as i32, h: height as i32}  as _) };
  }

  pub fn begin(&mut self) {
    unsafe { 
      SDL_SetRenderDrawColor(self.renderer, 0, 255, 255, 255); // white background
      SDL_RenderClear(self.renderer);
    }
  }

  pub fn end(&mut self) {
    unsafe {
      SDL_RenderPresent(self.renderer);
    }
  }

  pub fn draw_quad(&mut self, quad : &Quad2D) {
    println!("{}", quad);
    unsafe {
      
      SDL_SetRenderDrawColor(self.renderer, quad.color.r, quad.color.g, quad.color.b, quad.color.a); // white background
      SDL_RenderFillRect(self.renderer, &mut SDL_FRect {x: quad.x as f32, y: quad.y as f32, w: quad.width as f32, h: quad.height as f32} as _);
    }
  }

  pub fn draw_quads(&mut self, quads : &Vec<Quad2D>) {
    let mut draw_list : Vec<SDL_FRect> = quads.iter().map(|quad| SDL_FRect {x: quad.x as f32, y: quad.y as f32, w: quad.width as f32, h: quad.height as f32} as _).collect();
    
    unsafe {
      SDL_RenderFillRects(self.renderer, draw_list.as_mut_ptr() as _, draw_list.len() as i32);
    }
  }
}


impl Drop for Renderer {
  fn drop(&mut self) {
    unsafe { SDL_DestroyRenderer(self.renderer) }
  }
}