use iced::{Border, Color, Element, Event, event, Length, Padding, Point, Rectangle, Shadow, Size, Theme};
use iced::advanced::{
    Clipboard, layout, Layout, mouse, renderer, Shell, Widget,
    renderer::Quad,
    widget::Tree,
};

use std::time::{Duration, SystemTime};

pub struct TitleBar<'a, Message, Renderer> {
    // User Edited
    drag_message:    Message,
    toggle_message:  Message,
    padding:         Padding,
    spacing:         f32,


    title:           Option<&'a str>,
    children:       Vec<Element<'a, Message, Theme, Renderer>>,
    icon_index:     Option<usize>,
    tab_line_index: Option<usize>,
    minimise_index: Option<usize>,
    maximise_index: Option<usize>,
    close_index:    Option<usize>,

    // Internals
    last_pressed: SystemTime,
}

impl<'a, Message, Renderer> TitleBar<'a, Message, Renderer>
    where
        Renderer: iced::advanced::Renderer,
{
    /// Creates a new Titlebar object with no decorations
    pub fn new(drag_message: Message, toggle_message: Message) -> Self {
        Self {
            drag_message,
            toggle_message,
            padding:        Padding::ZERO,
            spacing:        0.0,
            title:          None,
            children:       Vec::new(),
            icon_index:     None,
            tab_line_index: None,
            minimise_index: None,
            maximise_index: None,
            close_index:    None,
            last_pressed:   SystemTime::now(),
        }
    }

    /// Adds padding pixels around the outside of all included widgets. Default == 0
    pub fn padding<P>(mut self, padding: P) -> Self
    where
        P: Into<Padding>,
    {
        self.padding = padding.into();
        self
    }

    /// Adds spacing pixels between the widgets inside the title bar. Default == 0
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Widget to be taken and positioned as the window tab line
    pub fn with_title(mut self, title: &'a str) -> Self {
        if title == "" {
            self.title = None;
        } else {
            self.title = Some(title);
        }

        self
    }

    /// Widget to be taken and positioned as the window tab line
    pub fn with_icon(mut self, icon: Element<'a, Message, Theme, Renderer>) -> Self {
        if let Some(index) = self.icon_index {
            self.children[index] = icon;
        } else {
            self.children.push(icon);
            self.icon_index = Some(self.children.len() - 1)
        }

        self
    }

    /// Widget to be taken and positioned as the window tab line
    pub fn with_tab_line(mut self, tab_line: Element<'a, Message, Theme, Renderer>) -> Self {
        if let Some(index) = self.tab_line_index {
            self.children[index] = tab_line;
        } else {
            self.children.push(tab_line);
            self.tab_line_index = Some(self.children.len() - 1)
        }

        self
    }

    /// Widget to be taken and positioned as the window minimise button
    pub fn with_minimise_button(mut self, minimise_button: Element<'a, Message, Theme, Renderer>) -> Self {
        if let Some(index) = self.minimise_index {
            self.children[index] = minimise_button;
        } else {
            self.children.push(minimise_button);
            self.minimise_index = Some(self.children.len() - 1)
        }

        self
    }

    /// Widget to be taken and positioned as the window maximise button
    pub fn with_maximise_button(mut self, maximise_button: Element<'a, Message, Theme, Renderer>) -> Self {
        if let Some(index) = self.maximise_index {
            self.children[index] = maximise_button;
        } else {
            self.children.push(maximise_button);
            self.maximise_index = Some(self.children.len() - 1)
        }

        self
    }

    /// Widget to be taken and positioned as the window closed button
    pub fn with_close_button(mut self, close_button: Element<'a, Message, Theme, Renderer>) -> Self {
        if let Some(index) = self.close_index {
            self.children[index] = close_button;
        } else {
            self.children.push(close_button);
            self.close_index = Some(self.children.len() - 1)
        }

        self
    }
}


impl<'a, Message, Renderer> Widget<Message, Theme, Renderer> for TitleBar<'a, Message, Renderer>
    where
        Message: Clone,
        Renderer: iced::advanced::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Shrink,
        }
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {

        // Check if Title Bar has any child nodes.
        // If not return default size.
        if self.icon_index.is_none()
        && self.tab_line_index.is_none()
        && self.minimise_index.is_none()
        && self.maximise_index.is_none()
        && self.close_index.is_none()
        {
            return layout::Node::new(Size::new(limits.max().width, 30.0));
        }

        // Track the max height for all the widgets, as well as the current "cursor" location
        // to place the buttons as required in order
        let mut max_widget_height: f32 = 0.0;
        let mut current_location = limits.max().width;
        let mut child_nodes: Vec<layout::Node> = vec![layout::Node::new(Size::new(0.0, 0.0)); self.children.len()];

        // Close Node
        if let Some(index) = self.close_index {
            let mut close_node = self.children[index].as_widget().layout(&mut tree.children[0], renderer, limits);

            current_location = current_location - close_node.size().width - self.padding.right;
            close_node.move_to_mut(Point::new(current_location, self.padding.top));

            max_widget_height = close_node.size().height;
            child_nodes[index] = close_node;
        }

        println!("Close button child index, layout index = {}", self.close_index.unwrap());

        // Maximise Node
        if let Some(index) = self.maximise_index {
            let mut maximise_node = self.children[index].as_widget().layout(&mut tree.children[1], renderer, limits);
            
            current_location = current_location - maximise_node.size().width - self.spacing;
            maximise_node.move_to_mut(Point::new(current_location, self.padding.top));

            max_widget_height = max_widget_height.max(maximise_node.size().height);
            child_nodes[index] = maximise_node;
        }

        // Maximise Node
        if let Some(index) = self.minimise_index {
            let mut minimise_node = self.children[index].as_widget().layout(&mut tree.children[2], renderer, limits);

            current_location = current_location - minimise_node.size().width - self.spacing;
            minimise_node.move_to_mut(Point::new(current_location, self.padding.top));

            max_widget_height = max_widget_height.max(minimise_node.size().height);
            child_nodes[index] = minimise_node;
        }

        layout::Node::with_children(Size::new(limits.max().width, max_widget_height + self.padding.top + self.padding.bottom), child_nodes)
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            Quad {
                bounds: layout.bounds(),
                border: Border {
                    color: Color::WHITE,
                    width: 0.0,
                    radius: 0.0.into(),
                },
                shadow: Shadow::default(),
            },
            Color::WHITE,
        );

        for ((child, state), layout) in self
            .children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
        {
            child.as_widget().draw(
                state,
                renderer,
                theme,
                style,
                layout,
                cursor,
                viewport,
            );
        }
    }

    fn children(&self) -> Vec<Tree> {
        self.children.iter().map(Tree::new).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.children);
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        if let event::Status::Captured = self.children
            .iter_mut()
            .zip(&mut tree.children)
            .zip(layout.children())
            .map(|((child, state), layout)| {
                child.as_widget_mut().on_event(
                    state,
                    event.clone(),
                    layout,
                    cursor,
                    renderer,
                    clipboard,
                    shell,
                    viewport,
                )
            }).fold(event::Status::Ignored, event::Status::merge) {

            event::Status::Captured

        } else {
            if cursor.is_over(layout.bounds()) {
                match event {
                    Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) => {
                        let double_click_duration = self.last_pressed.elapsed().unwrap();
                        if double_click_duration <= Duration::from_millis(200) {
                            shell.publish(self.toggle_message.clone());
                        } else {
                            self.last_pressed = SystemTime::now();
                            shell.publish(self.drag_message.clone());
                        }

                        return event::Status::Captured
                    }
                    _ => {
                        return event::Status::Ignored
                    }
                }
            }
            event::Status::Ignored
        }
    }
}

impl<'a, Message, Renderer> From<TitleBar<'a, Message, Renderer>> for Element<'a, Message, Theme, Renderer>
    where
        Message:  'a + Clone,
        Renderer: 'a + iced::advanced::Renderer,
{
    fn from(widget: TitleBar<'a, Message, Renderer>) -> Self {
        Self::new(widget)
    }
}