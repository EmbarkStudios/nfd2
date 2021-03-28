use std::{
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
};

mod ffi {
    use std::os::raw::{c_char, c_void};

    #[repr(C)]
    #[allow(dead_code)]
    pub(crate) enum FileChooserAction {
        Open,
        Save,
        SelectFolder,
        CreateFolder,
    }

    #[derive(PartialEq)]
    #[repr(C)]
    #[allow(dead_code)]
    pub(crate) enum Response {
        None = -1,
        Reject = -2,
        Accept = -3,
        DeleteEvent = -4,
        Ok = -5,
        Cancel = -6,
        Close = -7,
        Yes = -8,
        No = -9,
        Apply = -10,
        Help = -11,
    }

    #[repr(C)]
    pub(crate) struct Widget {
        _unused: [u8; 0],
    }

    #[repr(C)]
    pub(crate) struct Window {
        _unused: [u8; 0],
    }

    #[repr(C)]
    pub(crate) struct FileFilter {
        _unused: [u8; 0],
    }

    #[repr(C)]
    pub(crate) struct Display {
        _unused: [u8; 0],
    }

    #[repr(C)]
    pub(crate) struct List {
        pub(crate) data: *mut c_void,
        pub(crate) next: *mut List,
    }

    #[link(name = "glib-2.0")]
    extern "C" {
        // https://developer.gnome.org/glib/stable/glib-Memory-Allocation.html#g-free
        pub(crate) fn g_free(ptr: *mut c_void);
        // https://developer.gnome.org/glib/stable/glib-Singly-Linked-Lists.html#g-slist-free
        pub(crate) fn g_slist_free(list: *mut List);
    }

    #[link(name = "gobject-2.0")]
    extern "C" {
        // This doesn't seem to be a function that should be used publicly as it's
        // all hidden behind macros
        pub(crate) fn g_type_check_instance_is_a(
            instance: *mut std::os::raw::c_void,
            itype: usize,
        ) -> i32;
    }

    #[link(name = "gdk-3")]
    extern "C" {
        // https://developer.gnome.org/gdk3/stable/gdk3-Windows.html#gdk-window-get-events
        pub(crate) fn gdk_window_get_events(window: *mut Window) -> u32;
        // https://developer.gnome.org/gdk3/stable/gdk3-Windows.html#gdk-window-set-events
        pub(crate) fn gdk_window_set_events(window: *mut Window, event_mask: u32);

        // https://developer.gnome.org/gdk3/stable/GdkDisplay.html#gdk-display-get-default
        pub(crate) fn gdk_display_get_default() -> *mut Display;
        // This is the function behind GDK_TYPE_X11_DISPLAY
        pub(crate) fn gdk_x11_display_get_type() -> usize;
        // https://developer.gnome.org/gdk3/stable/gdk3-X-Window-System-Interaction.html#gdk-x11-get-server-time
        pub(crate) fn gdk_x11_get_server_time(window: *mut Window) -> u32;
    }

    #[link(name = "gtk-3")]
    extern "C" {
        // https://developer.gnome.org/gtk3/stable/gtk3-General.html#gtk-init-check
        pub(crate) fn gtk_init_check(argc: *mut i32, argv: *mut *mut *mut c_char) -> i32;
        // https://developer.gnome.org/gtk3/stable/gtk3-General.html#gtk-events-pending
        pub(crate) fn gtk_events_pending() -> i32;
        // https://developer.gnome.org/gtk3/stable/gtk3-General.html#gtk-main-iteration
        pub(crate) fn gtk_main_iteration() -> i32;

        // https://developer.gnome.org/gtk3/stable/GtkWidget.html#gtk-widget-show-all
        pub(crate) fn gtk_widget_show_all(widget: *mut Widget);
        // https://developer.gnome.org/gtk3/stable/GtkWidget.html#gtk-widget-get-window
        pub(crate) fn gtk_widget_get_window(widget: *mut Widget) -> *mut Window;
        // https://developer.gnome.org/gtk3/stable/GtkWindow.html#gtk-window-present-with-time
        pub(crate) fn gtk_window_present_with_time(window: *mut Window, timestamp: u32);
        // https://developer.gnome.org/gtk3/stable/GtkDialog.html#gtk-dialog-run
        pub(crate) fn gtk_dialog_run(dialog: *mut Widget) -> Response;
        // https://developer.gnome.org/gtk3/stable/GtkWidget.html#gtk-widget-destroy
        pub(crate) fn gtk_widget_destroy(widget: *mut Widget);

        // https://developer.gnome.org/gtk3/stable/GtkFileChooserDialog.html
        pub(crate) fn gtk_file_chooser_dialog_new(
            title: *const c_char,
            parent: *mut c_void,
            action: FileChooserAction,
            first_button_text: *const c_char,
            ...
        ) -> *mut Widget;
        // https://developer.gnome.org/gtk3/stable/GtkFileChooser.html#gtk-file-chooser-set-current-folder
        pub(crate) fn gtk_file_chooser_set_current_folder(
            widget: *mut Widget,
            path: *const c_char,
        ) -> i32;
        // https://developer.gnome.org/gtk3/stable/GtkFileChooser.html#gtk-file-chooser-set-filename
        pub(crate) fn gtk_file_chooser_set_filename(
            widget: *mut Widget,
            path: *const c_char,
        ) -> i32;
        // https://developer.gnome.org/gtk3/stable/GtkFileChooser.html#gtk-file-chooser-get-filename
        pub(crate) fn gtk_file_chooser_get_filename(widget: *mut Widget) -> *mut c_char;
        // https://developer.gnome.org/gtk3/stable/GtkFileChooser.html#gtk-file-chooser-get-filenames
        pub(crate) fn gtk_file_chooser_get_filenames(widget: *mut Widget) -> *mut List;
        // https://developer.gnome.org/gtk3/stable/GtkFileChooser.html#gtk-file-chooser-set-select-multiple
        pub(crate) fn gtk_file_chooser_set_select_multiple(
            widget: *mut Widget,
            select_multiple: i32,
        );
        // https://developer.gnome.org/gtk3/stable/GtkFileChooser.html#gtk-file-chooser-set-do-overwrite-confirmation
        pub(crate) fn gtk_file_chooser_set_do_overwrite_confirmation(
            widget: *mut Widget,
            confirm: i32,
        );

        // https://developer.gnome.org/gtk3/stable/GtkFileFilter.html#gtk-file-filter-new
        pub(crate) fn gtk_file_filter_new() -> *mut FileFilter;
        // https://developer.gnome.org/gtk3/stable/GtkFileFilter.html#gtk-file-filter-add-pattern
        pub(crate) fn gtk_file_filter_add_pattern(filter: *mut FileFilter, pattern: *const c_char);
        // https://developer.gnome.org/gtk3/stable/GtkFileFilter.html#gtk-file-filter-set-name
        pub(crate) fn gtk_file_filter_set_name(filter: *mut FileFilter, name: *const c_char);
        // https://developer.gnome.org/gtk3/stable/GtkFileChooser.html#gtk-file-chooser-add-filter
        pub(crate) fn gtk_file_chooser_add_filter(chooser: *mut Widget, filter: *mut FileFilter);
    }
}

pub(crate) fn open_dialog(
    filter_list: Option<crate::FilterList<'_>>,
    default_path: Option<&Path>,
) -> crate::Result<crate::Response> {
    unsafe {
        if ffi::gtk_init_check(std::ptr::null_mut(), std::ptr::null_mut()) == 0 {
            return Err("gtk_init_check failed to initilaize GTK+".into());
        }

        let dialog = ffi::gtk_file_chooser_dialog_new(
            b"Open File\0".as_ptr() as *const i8,
            std::ptr::null_mut(),
            ffi::FileChooserAction::Open,
            "_Cancel\0".as_ptr() as *const i8,
            ffi::Response::Cancel,
            b"_Open\0".as_ptr() as *const i8,
            ffi::Response::Accept,
            std::ptr::null::<i8>(),
        );

        add_filters(dialog, filter_list)?;
        if let Some(def_path) = default_path {
            set_default_dir(dialog, def_path);
        }
        fix_window_focus(dialog);

        let response = if ffi::gtk_dialog_run(dialog) == ffi::Response::Accept {
            let filename = ffi::gtk_file_chooser_get_filename(dialog);
            let filename = std::ffi::CStr::from_ptr(filename);

            let pb = PathBuf::from(std::ffi::OsStr::from_bytes(filename.to_bytes()));

            ffi::g_free(filename.as_ptr() as *mut _);
            crate::Response::Okay(pb)
        } else {
            crate::Response::Cancel
        };

        wait_for_cleanup();
        ffi::gtk_widget_destroy(dialog);
        wait_for_cleanup();

        Ok(response)
    }
}

pub(crate) fn open_dialog_multi(
    filter_list: Option<crate::FilterList<'_>>,
    default_path: Option<&Path>,
) -> crate::Result<crate::Response> {
    unsafe {
        if ffi::gtk_init_check(std::ptr::null_mut(), std::ptr::null_mut()) == 0 {
            return Err("gtk_init_check failed to initilaize GTK+".into());
        }

        let dialog = ffi::gtk_file_chooser_dialog_new(
            b"Open Files\0".as_ptr() as *const _,
            std::ptr::null_mut(),
            ffi::FileChooserAction::Open,
            b"_Cancel\0".as_ptr() as *const _,
            ffi::Response::Cancel,
            b"_Open\0".as_ptr() as *const i8,
            ffi::Response::Accept,
            std::ptr::null::<i8>(),
        );

        ffi::gtk_file_chooser_set_select_multiple(dialog, 1);

        add_filters(dialog, filter_list)?;
        if let Some(dp) = default_path {
            set_default_dir(dialog, dp);
        }
        fix_window_focus(dialog);

        let response = if ffi::gtk_dialog_run(dialog) == ffi::Response::Accept {
            let list = ffi::gtk_file_chooser_get_filenames(dialog);

            let mut paths = Vec::new();

            let mut current = list;
            while !current.is_null() {
                let data = (*current).data;
                debug_assert!(!data.is_null());

                let filename = std::ffi::CStr::from_ptr(data as *const i8);

                let pb = PathBuf::from(std::ffi::OsStr::from_bytes(filename.to_bytes()));
                ffi::g_free(data);
                paths.push(pb);

                current = (*current).next;
            }

            ffi::g_slist_free(list);

            crate::Response::OkayMultiple(paths)
        } else {
            crate::Response::Cancel
        };

        wait_for_cleanup();
        ffi::gtk_widget_destroy(dialog);
        wait_for_cleanup();

        Ok(response)
    }
}

pub(crate) fn open_save_dialog(
    filter_list: Option<crate::FilterList<'_>>,
    default_path: Option<&Path>,
) -> crate::Result<crate::Response> {
    unsafe {
        if ffi::gtk_init_check(std::ptr::null_mut(), std::ptr::null_mut()) == 0 {
            return Err("gtk_init_check failed to initilaize GTK+".into());
        }

        let dialog = ffi::gtk_file_chooser_dialog_new(
            b"Save File\0".as_ptr() as *const _,
            std::ptr::null_mut(),
            ffi::FileChooserAction::Save,
            b"_Cancel\0".as_ptr() as *const _,
            ffi::Response::Cancel,
            b"_Save\0".as_ptr() as *const i8,
            ffi::Response::Accept,
            std::ptr::null::<i8>(),
        );

        ffi::gtk_file_chooser_set_do_overwrite_confirmation(dialog, 1);

        add_filters(dialog, filter_list)?;
        if let Some(dp) = default_path {
            set_default_dir(dialog, dp);
        }
        fix_window_focus(dialog);

        let response = if ffi::gtk_dialog_run(dialog) == ffi::Response::Accept {
            let filename = ffi::gtk_file_chooser_get_filename(dialog);
            let filename = std::ffi::CStr::from_ptr(filename);

            let pb = PathBuf::from(std::ffi::OsStr::from_bytes(filename.to_bytes()));

            ffi::g_free(filename.as_ptr() as *mut _);
            crate::Response::Okay(pb)
        } else {
            crate::Response::Cancel
        };

        wait_for_cleanup();
        ffi::gtk_widget_destroy(dialog);
        wait_for_cleanup();

        Ok(response)
    }
}

pub(crate) fn pick_folder(default_path: Option<&Path>) -> crate::Result<crate::Response> {
    unsafe {
        if ffi::gtk_init_check(std::ptr::null_mut(), std::ptr::null_mut()) == 0 {
            return Err("gtk_init_check failed to initilaize GTK+".into());
        }

        let dialog = ffi::gtk_file_chooser_dialog_new(
            b"Select folder\0".as_ptr() as *const _,
            std::ptr::null_mut(),
            ffi::FileChooserAction::SelectFolder,
            b"_Cancel\0".as_ptr() as *const _,
            ffi::Response::Cancel,
            b"_Select\0".as_ptr() as *const i8,
            ffi::Response::Accept,
            std::ptr::null::<i8>(),
        );

        ffi::gtk_file_chooser_set_do_overwrite_confirmation(dialog, 1);

        if let Some(dp) = default_path {
            set_default_dir(dialog, dp);
        }
        fix_window_focus(dialog);

        let response = if ffi::gtk_dialog_run(dialog) == ffi::Response::Accept {
            let filename = ffi::gtk_file_chooser_get_filename(dialog);
            let filename = std::ffi::CStr::from_ptr(filename);

            let pb = PathBuf::from(std::ffi::OsStr::from_bytes(filename.to_bytes()));

            ffi::g_free(filename.as_ptr() as *mut _);
            crate::Response::Okay(pb)
        } else {
            crate::Response::Cancel
        };

        wait_for_cleanup();
        ffi::gtk_widget_destroy(dialog);
        wait_for_cleanup();

        Ok(response)
    }
}

unsafe fn add_filters(
    dialog: *mut ffi::Widget,
    filter_list: Option<crate::FilterList<'_>>,
) -> crate::Result<()> {
    if let Some(filter_list) = filter_list {
        let mut type_buf = String::with_capacity(128);
        type_buf.push_str("*.");

        let mut filter_name = String::with_capacity(128);

        for exts in filter_list {
            let filter = ffi::gtk_file_filter_new();

            if exts.is_empty() {
                return Err("empty type list provided".into());
            }

            filter_name.clear();

            for typ in *exts {
                if typ.is_empty() {
                    return Err("empty type provided".into());
                } else if typ.len() >= 256 {
                    return Err("provided type was too large".into());
                }

                if !filter_name.is_empty() {
                    filter_name.push_str(", ");
                }

                filter_name.push_str(typ);

                type_buf.truncate(2);
                type_buf.push_str(typ);
                // Null terminate
                type_buf.push(0 as char);

                ffi::gtk_file_filter_add_pattern(filter, type_buf.as_ptr() as *const i8);
            }

            filter_name.push(0 as char);
            ffi::gtk_file_filter_set_name(filter, filter_name.as_ptr() as *const i8);
            ffi::gtk_file_chooser_add_filter(dialog, filter);
        }
    }

    // Always append a wildcard option to the end
    let filter = ffi::gtk_file_filter_new();
    ffi::gtk_file_filter_set_name(filter, b"*.*\0".as_ptr() as *const i8);
    ffi::gtk_file_filter_add_pattern(filter, b"*\0".as_ptr() as *const i8);
    ffi::gtk_file_chooser_add_filter(dialog, filter);

    Ok(())
}

// Work around focus issue on X11, see https://github.com/mlabbe/nativefiledialog/issues/79
// for details on the issue and https://github.com/mlabbe/nativefiledialog/pull/92 for
// the fix
unsafe fn fix_window_focus(widget: *mut ffi::Widget) {
    ffi::gtk_widget_show_all(widget);
    let display = ffi::gdk_display_get_default();
    let x11_display_type = ffi::gdk_x11_display_get_type();

    if ffi::g_type_check_instance_is_a(display as *mut _, x11_display_type) != 0 {
        let window = ffi::gtk_widget_get_window(widget);
        let events = ffi::gdk_window_get_events(window);
        ffi::gdk_window_set_events(
            window,
            events | (1 << 16), /* GDK_PROPERTY_CHANGE_MASK */
        );
        ffi::gtk_window_present_with_time(widget as *mut _, ffi::gdk_x11_get_server_time(window));
    }
}

unsafe fn set_default_dir(dialog: *mut ffi::Widget, default: &Path) {
    let mut path = default.as_os_str().to_owned();
    path.push("\0");
    if default.is_dir() {
        ffi::gtk_file_chooser_set_current_folder(dialog, path.as_bytes().as_ptr() as *const i8);
    } else {
        ffi::gtk_file_chooser_set_filename(dialog, path.as_bytes().as_ptr() as *const i8);
    }
}

/// Pump the GTK event loop until there are no pending events
#[inline]
unsafe fn wait_for_cleanup() {
    while ffi::gtk_events_pending() != 0 {
        ffi::gtk_main_iteration();
    }
}
