use windows::Win32::UI::Accessibility::DockPosition;
use windows::Win32::UI::Accessibility::IUIAutomationAnnotationPattern;
use windows::Win32::UI::Accessibility::IUIAutomationCustomNavigationPattern;
use windows::Win32::UI::Accessibility::IUIAutomationDockPattern;
use windows::Win32::UI::Accessibility::IUIAutomationInvokePattern;
use windows::Win32::UI::Accessibility::NavigateDirection;
use windows::Win32::UI::Accessibility::UIA_AnnotationPatternId;
use windows::Win32::UI::Accessibility::UIA_CustomNavigationPatternId;
use windows::Win32::UI::Accessibility::UIA_DockPatternId;
use windows::Win32::UI::Accessibility::UIA_InvokePatternId;
use windows::core::IUnknown;
use windows::core::Interface;

use crate::core::UIElement;
use crate::errors::Error;
use crate::errors::Result;

pub trait UIPattern : Sized {
    fn pattern_id() -> i32;
    fn new(pattern: IUnknown) -> Result<Self>;
}

pub struct UIInvokePattern {
    pattern: IUIAutomationInvokePattern
}

impl UIInvokePattern {
    pub fn invoke(&self) -> Result<()> {
        unsafe {
            self.pattern.Invoke()?;
        }
        Ok(())
    }
}

impl UIPattern for UIInvokePattern {
    fn pattern_id() -> i32 {
        UIA_InvokePatternId
    }

    fn new(pattern: IUnknown) -> Result<Self> {
        UIInvokePattern::try_from(pattern)
    }
}

impl TryFrom<IUnknown> for UIInvokePattern {
    type Error = Error;

    fn try_from(pattern: IUnknown) -> core::result::Result<Self, Self::Error> {
        let pattern: IUIAutomationInvokePattern = pattern.cast()?;
        Ok(Self {
            pattern
        })
    }
}

impl From<IUIAutomationInvokePattern> for UIInvokePattern {
    fn from(pattern: IUIAutomationInvokePattern) -> Self {
        Self {
            pattern
        }
    }
}

impl Into<IUIAutomationInvokePattern> for UIInvokePattern {
    fn into(self) -> IUIAutomationInvokePattern {
        self.pattern
    }
}

impl AsRef<IUIAutomationInvokePattern> for UIInvokePattern {
    fn as_ref(&self) -> &IUIAutomationInvokePattern {
        &self.pattern
    }
}

pub struct UIAnnotationPattern {
    pattern: IUIAutomationAnnotationPattern
}

impl UIAnnotationPattern {
    pub fn get_type_id(&self) -> Result<i32> {
        let id = unsafe {
            self.pattern.CurrentAnnotationTypeId()?
        };
        Ok(id)
    }

    pub fn get_type_nane(&self) -> Result<String> {
        let name = unsafe {
            self.pattern.CurrentAnnotationTypeName()?
        };
        Ok(name.to_string())
    }

    pub fn get_author(&self) -> Result<String> {
        let author = unsafe {
            self.pattern.CurrentAuthor()?
        };
        Ok(author.to_string())
    }

    pub fn get_datetime(&self) -> Result<String> {
        let datetime = unsafe {
            self.pattern.CurrentDateTime()?
        };
        Ok(datetime.to_string())
    }

    pub fn get_target(&self) -> Result<UIElement> {
        let target = unsafe {
            self.pattern.CurrentTarget()?
        };
        Ok(target.into())
    }
}

impl UIPattern for UIAnnotationPattern {
    fn pattern_id() -> i32 {
        UIA_AnnotationPatternId
    }

    fn new(pattern: IUnknown) -> Result<Self> {
        UIAnnotationPattern::try_from(pattern)
    }
}

impl TryFrom<IUnknown> for UIAnnotationPattern {
    type Error = Error;

    fn try_from(value: IUnknown) -> core::result::Result<Self, Self::Error> {
        let pattern: IUIAutomationAnnotationPattern = value.cast()?;
        Ok(Self {
            pattern
        })
    }
}

impl From<IUIAutomationAnnotationPattern> for UIAnnotationPattern {
    fn from(pattern: IUIAutomationAnnotationPattern) -> Self {
        Self {
            pattern
        }
    }
}

impl Into<IUIAutomationAnnotationPattern> for UIAnnotationPattern {
    fn into(self) -> IUIAutomationAnnotationPattern {
        self.pattern
    }
}

impl AsRef<IUIAutomationAnnotationPattern> for UIAnnotationPattern {
    fn as_ref(&self) -> &IUIAutomationAnnotationPattern {
        &self.pattern
    }
}

pub struct UICustomNavigationPattern {
    pattern: IUIAutomationCustomNavigationPattern
}

impl UICustomNavigationPattern {
    pub fn navigate(&self, direction: NavigateDirection) -> Result<UIElement> {
        let element = unsafe {
            self.pattern.Navigate(direction)?
        };
        Ok(element.into())
    }
}

impl UIPattern for UICustomNavigationPattern {
    fn pattern_id() -> i32 {
        UIA_CustomNavigationPatternId
    }

    fn new(pattern: IUnknown) -> Result<Self> {
        UICustomNavigationPattern::try_from(pattern)
    }
}

impl TryFrom<IUnknown> for UICustomNavigationPattern {
    type Error = Error;

    fn try_from(value: IUnknown) -> core::result::Result<Self, Self::Error> {
        let pattern: IUIAutomationCustomNavigationPattern = value.cast()?;
        Ok(Self {
            pattern
        })
    }
}

impl From<IUIAutomationCustomNavigationPattern> for UICustomNavigationPattern {
    fn from(pattern: IUIAutomationCustomNavigationPattern) -> Self {
        Self {
            pattern
        }
    }
}

impl Into<IUIAutomationCustomNavigationPattern> for UICustomNavigationPattern {
    fn into(self) -> IUIAutomationCustomNavigationPattern {
        self.pattern
    }
}

impl AsRef<IUIAutomationCustomNavigationPattern> for UICustomNavigationPattern {
    fn as_ref(&self) -> &IUIAutomationCustomNavigationPattern {
        &self.pattern
    }
}

pub struct UIDockPattern {
    pattern: IUIAutomationDockPattern
}

impl UIDockPattern {
    pub fn get_dock_position(&self) -> Result<DockPosition> {
        let pos = unsafe {
            self.pattern.CurrentDockPosition()?
        };
        Ok(pos)
    }

    pub fn set_doc_position(&self, position: DockPosition) -> Result<()> {
        unsafe {
            self.pattern.SetDockPosition(position)?
        };
        Ok(())
    }
}

impl UIPattern for UIDockPattern {
    fn pattern_id() -> i32 {
        UIA_DockPatternId
    }

    fn new(pattern: IUnknown) -> Result<Self> {
        UIDockPattern::try_from(pattern)
    }
}

impl TryFrom<IUnknown> for UIDockPattern {
    type Error = Error;

    fn try_from(value: IUnknown) -> core::result::Result<Self, Self::Error> {
        let pattern: IUIAutomationDockPattern = value.cast()?;
        Ok(Self {
            pattern
        })
    }
}

impl From<IUIAutomationDockPattern> for UIDockPattern {
    fn from(pattern: IUIAutomationDockPattern) -> Self {
        Self {
            pattern
        }
    }
}

impl Into<IUIAutomationDockPattern> for UIDockPattern {
    fn into(self) -> IUIAutomationDockPattern {
        self.pattern
    }
}

impl AsRef<IUIAutomationDockPattern> for UIDockPattern {
    fn as_ref(&self) -> &IUIAutomationDockPattern {
        &self.pattern
    }
}