use iced::{
    widget::canvas::{Cache, Event, Frame, Geometry, Program, Text},
    mouse, Point, Rectangle, Renderer, Theme,
};
use kuchiki::NodeRef;

pub struct HTMLCanvas {
    pub dom: Option<NodeRef>,
    cache: Cache,
}

impl HTMLCanvas {
    pub fn new(dom: Option<NodeRef>) -> Self {
        HTMLCanvas {
            dom,
            cache: Cache::new(),
        }
    }
}

impl<Message> Program<Message> for HTMLCanvas {
    type State = ();

    fn update(
        &self,
        _state: &mut Self::State,
        _event: Event,
        _bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> (iced::event::Status, Option<Message>) {
        // We are not handling any interactive updates in this example
        (iced::event::Status::Ignored, None)
    }

    fn draw(
        &self,
        _state: &Self::State,
        renderer: &Renderer,
        _: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<Geometry<Renderer>> {
        let geometry = self.cache.draw(renderer, bounds.size(), |frame| {
            if let Some(dom) = &self.dom {
                self.draw_dom(frame, dom);
            }
        });

        vec![geometry]
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        _bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        mouse::Interaction::default()
    }
}

impl HTMLCanvas {
    fn draw_dom(&self, frame: &mut Frame<Renderer>, dom: &NodeRef) {
        let mut y = 20.0; // Starting Y position for drawing text
        for node in dom.inclusive_descendants() {
            if let kuchiki::NodeData::Text(text) = &node.data() {
                let text_content = text.borrow();
                frame.fill_text(Text {
                    content: text_content.clone(),
                    position: Point::new(20.0, y),
                    color: iced::Color::BLACK,
                    size: iced::Pixels(16.0),
                    ..Default::default()
                });
                y += 20.0; // Move Y down after drawing each line
            }
        }
    }
}
