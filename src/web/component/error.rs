use leptos::attr::any_attribute::AnyAttribute;
use leptos::prelude::*;
use leptos::tachys;
use leptos::tachys::hydration::Cursor;
use leptos::tachys::no_attrs;
use leptos::tachys::renderer::CastFrom;
use leptos::tachys::renderer::Rndr;
use leptos::tachys::view::{Position, PositionState};
use validator::ValidationErrors;

no_attrs!(WebError);

#[derive(Clone)]
pub struct WebError(pub String);

impl From<ValidationErrors> for WebError {
    fn from(err: ValidationErrors) -> Self {
        //TODO: error message
        Self(err.to_string())
    }
}

impl From<ServerFnError> for WebError {
    fn from(value: ServerFnError) -> Self {
        //TODO: error message
        Self(format!("{}", value))
    }
}

impl Into<Error> for WebError {
    fn into(self) -> Error {
        Error::from(self.0)
    }
}

impl Render for WebError {
    type State = StringState;

    fn build(self) -> Self::State {
        let node = Rndr::create_text_node(&self.0);
        StringState { node, str: self.0 }
    }

    fn rebuild(self, state: &mut Self::State) {
        let StringState { node, str } = state;
        if &self.0 != str {
            Rndr::set_text(node, &self.0);
            *str = self.0;
        }
    }
}

pub struct StringState {
    node: tachys::renderer::types::Text,
    str: String,
}

impl Mountable for StringState {
    fn unmount(&mut self) {
        self.node.unmount()
    }

    fn mount(
        &mut self,
        parent: &tachys::renderer::types::Element,
        marker: Option<&tachys::renderer::types::Node>,
    ) {
        Rndr::insert_node(parent, self.node.as_ref(), marker);
    }

    fn insert_before_this(&self, child: &mut dyn Mountable) -> bool {
        self.node.insert_before_this(child)
    }

    fn elements(&self) -> Vec<tachys::renderer::types::Element> {
        vec![]
    }
}

impl RenderHtml for WebError {
    type AsyncOutput = Self;
    type Owned = Self;
    const MIN_LENGTH: usize = 0;

    fn dry_resolve(&mut self) {}

    async fn resolve(self) -> Self::AsyncOutput {
        self
    }

    fn html_len(&self) -> usize {
        self.0.len()
    }

    fn to_html_with_buf(
        self,
        buf: &mut String,
        position: &mut Position,
        escape: bool,
        mark_branches: bool,
        extra_attrs: Vec<AnyAttribute>,
    ) {
        <&str as RenderHtml>::to_html_with_buf(
            self.0.as_str(),
            buf,
            position,
            escape,
            mark_branches,
            extra_attrs,
        )
    }

    fn hydrate<const FROM_SERVER: bool>(
        self,
        cursor: &Cursor,
        position: &PositionState,
    ) -> Self::State {
        if position.get() == Position::FirstChild {
            cursor.child();
        } else {
            cursor.sibling();
        }

        // separating placeholder marker comes before text node
        if matches!(position.get(), Position::NextChildAfterText) {
            cursor.sibling();
        }

        let node = cursor.current();
        let node = tachys::renderer::types::Text::cast_from(node.clone()).unwrap_or_else(|| {
            panic!("Unrecoverable hydration error.");
        });

        if !FROM_SERVER {
            Rndr::set_text(&node, self.0.as_str());
        }
        position.set(Position::NextChildAfterText);

        StringState { node, str: self.0 }
    }

    fn into_owned(self) -> Self::Owned {
        self
    }
}
