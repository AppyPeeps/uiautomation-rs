use uiautomation_derive::Invoke;
use uiautomation_derive::ItemContainer;
use uiautomation_derive::MultipleView;
use uiautomation_derive::ScrollItem;
use uiautomation_derive::SelectionItem;
use windows::Win32::UI::Accessibility::UIA_ButtonControlTypeId;
use windows::Win32::UI::Accessibility::UIA_ListControlTypeId;
use windows::Win32::UI::Accessibility::UIA_ListItemControlTypeId;

use crate::actions::*;
use crate::Error;
use crate::Result;
use crate::UIElement;
use crate::errors::ERR_TYPE;
use crate::patterns::UIInvokePattern;
use crate::patterns::UIItemContainerPattern;
use crate::patterns::UIMultipleViewPattern;
use crate::patterns::UIScrollItemPattern;
use crate::patterns::UISelectionItemPattern;
use crate::variants::Variant;

/// Wrapper an button element as a control.
#[derive(Invoke)]
pub struct ButtonControl {
    control: UIElement
}

impl TryFrom<UIElement> for ButtonControl {
    type Error = Error;

    fn try_from(control: UIElement) -> Result<Self> {
        if control.get_control_type()? == UIA_ButtonControlTypeId {
            Ok(Self {
                control
            })
        } else {
            Err(Error::new(ERR_TYPE, "Error Control Type"))
        }
    }
}

impl Into<UIElement> for ButtonControl {
    fn into(self) -> UIElement {
        self.control
    }
}

impl AsRef<UIElement> for ButtonControl {
    fn as_ref(&self) -> &UIElement {
        &self.control
    }
}

/// Wrapper a list element as a control. The control type of the element must be `UIA_ListControlTypeId`.
#[derive(MultipleView, ItemContainer)]
pub struct ListControl {
    control: UIElement
}

impl TryFrom<UIElement> for ListControl {
    type Error = Error;

    fn try_from(control: UIElement) -> Result<Self> {
        if control.get_control_type()? == UIA_ListControlTypeId {
            Ok(Self {
                control
            })
        } else {
            Err(Error::new(ERR_TYPE, "Error Control Type"))
        }
    }
}

impl Into<UIElement> for ListControl {
    fn into(self) -> UIElement {
        self.control
    }
}

impl AsRef<UIElement> for ListControl {
    fn as_ref(&self) -> &UIElement {
        &self.control
    }
}

/// Wrapper a listitem element as a control. The control type of the element must be `UIA_ListItemControlTypeId`.
#[derive(Invoke, SelectionItem, ScrollItem)]
pub struct ListItemControl {
    control: UIElement
}

impl TryFrom<UIElement> for ListItemControl {
    type Error = Error;

    fn try_from(control: UIElement) -> Result<Self> {
        if control.get_control_type()? == UIA_ListItemControlTypeId {
            Ok(Self {
                control
            })
        } else {
            Err(Error::new(ERR_TYPE, "Error Control Type"))
        }
    }
}

impl Into<UIElement> for ListItemControl {
    fn into(self) -> UIElement {
        self.control
    }
}

impl AsRef<UIElement> for ListItemControl {
    fn as_ref(&self) -> &UIElement {
        &self.control
    }
}
