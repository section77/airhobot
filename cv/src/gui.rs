use super::*;
use crossbeam_channel as chan;
use log::debug;
use opencv::highgui;
use std::time;
use xstd::prelude::*;

/// Simple GUI
#[derive(Debug)]
pub struct GUI {
    name: String,
}

impl GUI {
    pub fn new(name: &str) -> GUI {
        highgui::named_window(name, 1).unwrap();
        GUI { name: name.to_string() }
    }

    pub fn destroy(&self) {
        highgui::destroy_window(&self.name).unwrap();
    }

    pub fn show<T>(&self, frame: &Mat<T>) -> Result<()> {
        highgui::imshow(&self.name, &frame.unpack())?;
        Ok(())
    }

    // Show the given frame for the given duration
    pub fn show_for<T>(&self, frame: &Mat<T>, dur: time::Duration) -> Result<char> {
        self.show(frame)?;

        let millis = dur.as_millis() as i32;
        Ok(highgui::wait_key(millis)? as u8 as char)
    }

    pub fn slider<S>(&self, name: S, start: i32, end: i32) -> chan::Receiver<i32>
    where
        S: AsRef<str>,
    {
        let (tx, rx) = chan::unbounded();
        let mut start = start;
        highgui::create_trackbar(
            name.as_ref(),
            &self.name,
            &mut start,
            end,
            Some(Box::new({
                move |v| {
                    if let Err(err) = tx.send(v) {
                        debug!("send slider value err: {}", err);
                    }
                }
            })),
        )
        .expect("unable to create trackbar");
        rx
    }

    pub fn mouse_events(&self) -> MouseEvents {
        let (tx, rx) = chan::unbounded();
        self.register_mouse_callback(move |event| {
            if let Err(err) = tx.send(event) {
                debug!("publish mouse_events err: {}", err);
            }
        });
        rx
    }

    pub fn mouse_events_for<T: 'static>(&self) -> MouseEvents {
        let (tx, rx) = chan::unbounded();
        self.register_mouse_callback(move |event| {
            if event.is_kind::<T>() {
                if let Err(err) = tx.send(event) {
                    debug!("publish mouse_events_for err: {}", err);
                }
            }
        });
        rx
    }


    fn register_mouse_callback<CB>(&self, mut cb: CB)
    where
        CB: FnMut(MouseEvent) + Sized + Sync + Send + 'static,
    {
        highgui::set_mouse_callback(
            &self.name,
            Some(Box::new({
                move |event, x, y, _flags| {
                    let p = Point::new(x, y);
                    if let Some(mouse_event) = match event {
                        highgui::EVENT_MOUSEMOVE   => Some(MouseEvent::Move(p)),
                        highgui::EVENT_LBUTTONDOWN => Some(MouseEvent::LeftBtnDown(p)),
                        highgui::EVENT_LBUTTONUP   => Some(MouseEvent::LeftBtnUp(p)),
                        highgui::EVENT_RBUTTONDOWN => Some(MouseEvent::RightBtnDown(p)),
                        highgui::EVENT_RBUTTONUP   => Some(MouseEvent::RightBtnUp(p)),
                        _ => None,
                    } {
                        cb(mouse_event);
                    }
                }
            })),
        )
        .expect("register_mouse_callback");
    }

    pub fn unregister_mouse_callback(&self) {
        highgui::set_mouse_callback(&self.name, None).expect("unregister_mouse_callback");
    }
}




pub type MouseEvents = crossbeam_channel::Receiver<MouseEvent>;

macro_rules! mouse_event {
    ($(($enum:ident, $struct:ident)),+) => {
        pub mod mouse_events {
            $(
                #[derive(Debug, Clone, Copy)]
                pub struct $struct;
            )+
        }

        #[derive(Debug)]
        pub enum MouseEvent {
            $($enum(crate::Point),)+
        }
        impl MouseEvent {
            pub fn point(&self) -> crate::Point {
                match self {
                    $(MouseEvent::$enum(p) => *p,)+
                }
            }
            pub fn is_kind<T: 'static>(&self) -> bool {
                match self {
                    $(MouseEvent::$enum(_) => $struct.is_instance_of::<T>(),)+
                }
            }
        }
    }
}
mouse_event!((Move, MouseMove),
             (LeftBtnDown, MouseLeftBtnDown),
             (LeftBtnUp, MouseLeftBtnUp),
             (LeftBtnDplClick, MouseLeftBtnDplClick),
             (RightBtnDown, MouseRightBtnDown),
             (RightBtnUp, MouseRightBtnUp),
             (RightBtnDplClick, MouseRightBtnDplClick)
);



#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // // this test verifies only if the code compiles
    // // and if the api is easy to use
    // fn mouse_events_api() {
    //     let gui = GUI::new("mouse-events-api");
    //     let mouse_events = gui.mouse_events();
    //     match mouse_events.try_recv() {
    //         Ok(MouseEvent::Move(p)) => p,
    //         _ => Point::default(),
    //     };
    //     gui.destroy();
    // }

    // #[test]
    // // this test verifies only if the code compiles
    // // and if the api is easy to use
    // fn mouse_events_for_api() {
    //     let gui = GUI::new("mouse-events-for-api");
    //     let mouse_events = gui.mouse_events_for::<MouseLeftBtnDown>();
    //     if let Ok(event) = mouse_events.try_recv() {
    //         let _p: crate::Point = event.point();
    //     }
    //     gui.destroy();
    // }

}
