use proc_macro::TokenStream;
use quote::quote;

pub(crate) fn impl_invoke(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Invoke for #name {
            fn invoke(&self) -> Result<()> {
                let pattern: UIInvokePattern = self.as_ref().get_pattern()?;
                pattern.invoke()
            }
        }
    };
    gen.into()
}

pub(crate) fn impl_selection_item(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl SelectionItem for #name {
            fn select(&self) -> Result<()> {
                let pattern: UISelectionItemPattern = self.as_ref().get_pattern()?;
                pattern.select()
            }

            fn add_to_selection(&self) -> Result<()> {
                let pattern: UISelectionItemPattern = self.as_ref().get_pattern()?;
                pattern.add_to_selection()
            }

            fn remove_from_selection(&self) -> Result<()> {
                let pattern: UISelectionItemPattern = self.as_ref().get_pattern()?;
                pattern.remove_from_selection()
            }

            fn is_selected(&self) -> Result<bool> {
                let pattern: UISelectionItemPattern = self.as_ref().get_pattern()?;
                pattern.is_selected()
            }
        }
    };
    gen.into()
}

pub(crate) fn impl_multiple_view(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl MultipleView for #name {
            fn get_supported_views(&self) -> Result<Vec<i32>> {
                let pattern: UIMultipleViewPattern = self.as_ref().get_pattern()?;
                pattern.get_supported_views()
            }

            fn get_view_name(&self, view: i32) -> Result<String> {
                let pattern: UIMultipleViewPattern = self.as_ref().get_pattern()?;
                pattern.get_view_name(view)
            }

            fn get_current_view(&self) -> Result<i32> {
                let pattern: UIMultipleViewPattern = self.as_ref().get_pattern()?;
                pattern.get_current_view()
            }

            fn set_current_view(&self, view: i32) -> Result<()> {
                let pattern: UIMultipleViewPattern = self.as_ref().get_pattern()?;
                pattern.set_current_view(view)
            }
        }
    };
    gen.into()
}

pub(crate) fn impl_item_container(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl ItemContainer for #name {
            fn find_item_by_property(&self, start_after: UIElement, property_id: i32, value: Variant) -> Result<UIElement> {
                let pattern: UIItemContainerPattern = self.as_ref().get_pattern()?;
                pattern.find_item_by_property(start_after, property_id, value)
            }        
        }
    };
    gen.into()
}

pub(crate) fn impl_scroll_item(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl ScrollItem for #name {
            fn scroll_into_view(&self) -> Result<()> {
                let pattern: UIScrollItemPattern = self.as_ref().get_pattern()?;
                pattern.scroll_into_view()
            }
        }
    };
    gen.into()
}

pub(crate) fn impl_window(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Window for #name {
            fn close(&self) -> Result<()> {
                let pattern: UIWindowPattern = self.as_ref().get_pattern()?;
                pattern.close()
            }
        
            fn wait_for_input_idle(&self, milliseconds: i32) -> Result<bool> {
                let pattern: UIWindowPattern = self.as_ref().get_pattern()?;
                pattern.wait_for_input_idle(milliseconds)
            }
        
            fn is_normal(&self) -> Result<bool> {
                let pattern: UIWindowPattern = self.as_ref().get_pattern()?;
                Ok(pattern.get_window_visual_state()? == windows::Win32::UI::Accessibility::WindowVisualState_Normal)
            }
        
            fn normal(&self) -> Result<()> {
                let pattern: UIWindowPattern = self.as_ref().get_pattern()?;
                pattern.set_window_visual_state(windows::Win32::UI::Accessibility::WindowVisualState_Normal)
            }
        
            fn can_maximize(&self) -> Result<bool> {
                let pattern: UIWindowPattern = self.as_ref().get_pattern()?;
                pattern.can_maximize()
            }
        
            fn is_maximized(&self) -> Result<bool> {
                let pattern: UIWindowPattern = self.as_ref().get_pattern()?;
                Ok(pattern.get_window_visual_state()? == windows::Win32::UI::Accessibility::WindowVisualState_Maximized)
            }
        
            fn maximize(&self) -> Result<()> {
                let pattern: UIWindowPattern = self.as_ref().get_pattern()?;
                pattern.set_window_visual_state(windows::Win32::UI::Accessibility::WindowVisualState_Maximized)
            }
        
            fn can_minimize(&self) -> Result<bool> {
                let pattern: UIWindowPattern = self.as_ref().get_pattern()?;
                pattern.can_minimize()
            }
        
            fn is_minimized(&self) -> Result<bool> {
                let pattern: UIWindowPattern = self.as_ref().get_pattern()?;
                Ok(pattern.get_window_visual_state()? == windows::Win32::UI::Accessibility::WindowVisualState_Minimized)
            }
        
            fn minimize(&self) -> Result<()> {
                let pattern: UIWindowPattern = self.as_ref().get_pattern()?;
                pattern.set_window_visual_state(windows::Win32::UI::Accessibility::WindowVisualState_Minimized)
            }
        
            fn is_modal(&self) -> Result<bool> {
                let pattern: UIWindowPattern = self.as_ref().get_pattern()?;
                pattern.is_modal()
            }
        
            fn is_topmost(&self) -> Result<bool> {
                let pattern: UIWindowPattern = self.as_ref().get_pattern()?;
                pattern.is_topmost()
            }
        
            fn get_window_interaction_state(&self) -> Result<windows::Win32::UI::Accessibility::WindowInteractionState> {
                let pattern: UIWindowPattern = self.as_ref().get_pattern()?;
                pattern.get_window_interaction_state()
            }
        }
    };
    gen.into()
}

pub(crate) fn impl_transform(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Transform for #name {
            fn can_move(&self) -> Result<bool> {
                let pattern: UITransformPattern = self.as_ref().get_pattern()?;
                pattern.can_move()
            }
        
            fn move_to(&self, x: f64, y: f64) -> Result<()> {
                let pattern: UITransformPattern = self.as_ref().get_pattern()?;
                pattern.move_to(x, y)
            }
        
            fn can_resize(&self) -> Result<bool> {
                let pattern: UITransformPattern = self.as_ref().get_pattern()?;
                pattern.can_resize()
            }
        
            fn resize(&self, width: f64, height: f64) -> Result<()> {
                let pattern: UITransformPattern = self.as_ref().get_pattern()?;
                pattern.resize(width, height)
            }
        
            fn can_rotate(&self) -> Result<bool> {
                let pattern: UITransformPattern = self.as_ref().get_pattern()?;
                pattern.can_rotate()
            }
        
            fn rotate(&self, degrees: f64) -> Result<()> {
                let pattern: UITransformPattern = self.as_ref().get_pattern()?;
                pattern.rotate(degrees)
            }
        
            fn can_zoom(&self) -> Result<bool> {
                let pattern: UITransformPattern = self.as_ref().get_pattern()?;
                pattern.can_zoom()
            }
        
            fn get_zoom_level(&self) -> Result<f64> {
                let pattern: UITransformPattern = self.as_ref().get_pattern()?;
                pattern.get_zoom_level()
            }
        
            fn get_zoom_minimum(&self) -> Result<f64> {
                let pattern: UITransformPattern = self.as_ref().get_pattern()?;
                pattern.get_zoom_minimum()
            }
        
            fn get_zoom_maximum(&self) -> Result<f64> {
                let pattern: UITransformPattern = self.as_ref().get_pattern()?;
                pattern.get_zoom_maximum()
            }
        
            fn zoom(&self, zoom_value: f64) -> Result<()> {
                let pattern: UITransformPattern = self.as_ref().get_pattern()?;
                pattern.zoom(zoom_value)
            }
        
            fn zoom_by_unit(&self, zoom_unit: windows::Win32::UI::Accessibility::ZoomUnit) -> Result<()> {
                let pattern: UITransformPattern = self.as_ref().get_pattern()?;
                pattern.zoom_by_unit(zoom_unit)
            }
        }
    };
    gen.into()
}