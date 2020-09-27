/*
   Copyright (c) 2016 Saurav Sachidanand

   Permission is hereby granted, free of charge, to any person obtaining a copy
   of this software and associated documentation files (the "Software"), to deal
   in the Software without restriction, including without limitation the rights
   to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
   copies of the Software, and to permit persons to whom the Software is
   furnished to do so, subject to the following conditions:

   The above copyright notice and this permission notice shall be included in
   all copies or substantial portions of the Software.

   THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
   IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
   FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
   AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
   LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
   OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
   THE SOFTWARE.
*/

#![warn(
    clippy::all,
    clippy::doc_markdown,
    clippy::dbg_macro,
    clippy::todo,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::pub_enum_variant_names,
    clippy::mem_forget,
    clippy::use_self,
    clippy::filter_map_next,
    clippy::needless_continue,
    clippy::needless_borrow,
    rust_2018_idioms,
    future_incompatible,
    missing_copy_implementations,
    trivial_numeric_casts,
    unstable_features,
    nonstandard_style,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]

pub mod error;
pub mod ffi;

use error::NFDError;
use ffi::*;
use std::{
    ffi::{CStr, CString},
    os::raw::c_char,
    path::{Path, PathBuf},
};

/// Result of opening a file dialog. Note that the underlying C library only
/// ever returns paths encoded as utf-8 strings, if a path cannot be converted
/// to a valid utf-8 string, then an error is returned instead.
#[derive(Clone, PartialEq)]
pub enum Response {
    /// The user pressed okay, and a single path was selected
    Okay(PathBuf),
    /// The user pressed okay, and 1 or more paths were selected
    OkayMultiple(Vec<PathBuf>),
    /// User pressed cancel
    Cancel,
}

#[derive(Copy, Clone, PartialEq)]
pub enum DialogType {
    SingleFile,
    MultipleFiles,
    SaveFile,
    PickFolder,
}

pub struct DialogBuilder<'a> {
    filter: Option<&'a str>,
    default_path: Option<&'a Path>,
    dialog_type: DialogType,
}

impl<'a> DialogBuilder<'a> {
    pub fn new(dialog_type: DialogType) -> Self {
        DialogBuilder {
            filter: None,
            default_path: None,
            dialog_type,
        }
    }

    /// Creates a builder for selecting a single file
    pub fn single() -> Self {
        Self::new(DialogType::SingleFile)
    }

    /// Creates a builder for selecting multiple files
    pub fn multiple() -> Self {
        Self::new(DialogType::MultipleFiles)
    }

    /// Adds a filter to the dialog
    ///
    /// Separators
    /// - `;` Begin a new filter.
    /// - `,` Add a separate type to the filter.
    ///
    /// # Examples
    ///
    /// - `txt` The default filter is for text files. There is a wildcard option
    /// in a dropdown.
    /// - `png,jpg;psd` The default filter is for `png` and `jpg` files. A
    /// second filter is available for `psd` files. There is a wildcard option
    /// in a dropdown.
    /// - Not applying any filters means only the wildcard option is available.
    ///
    /// See the [documentation](https://github.com/mlabbe/nativefiledialog#file-filter-syntax)
    /// of the underlying C lib for more info.
    pub fn filter(&'a mut self, filter: &'a str) -> &mut DialogBuilder<'a> {
        self.filter = Some(filter);
        self
    }

    /// Specify the default directory to start the dialog in, otherwise the
    /// default directory will be dependent upon the host API
    pub fn default_path<P: AsRef<Path>>(&'a mut self, path: &'a P) -> &mut DialogBuilder<'a> {
        self.default_path = Some(path.as_ref());
        self
    }

    /// Opens the dialog and waits upon a response from the user
    pub fn open(&self) -> Result<Response> {
        open_dialog(self.filter, self.default_path, self.dialog_type)
    }
}

/// Helper
pub fn dialog<'a>() -> DialogBuilder<'a> {
    DialogBuilder::new(DialogType::SingleFile)
}

pub fn dialog_multiple<'a>() -> DialogBuilder<'a> {
    DialogBuilder::new(DialogType::MultipleFiles)
}

pub fn dialog_save<'a>() -> DialogBuilder<'a> {
    DialogBuilder::new(DialogType::SaveFile)
}

pub type Result<T> = std::result::Result<T, NFDError>;

/// Open single file dialog
pub fn open_file_dialog(
    filter_list: Option<&str>,
    default_path: Option<&Path>,
) -> Result<Response> {
    open_dialog(filter_list, default_path, DialogType::SingleFile)
}

/// Open mulitple file dialog
pub fn open_file_multiple_dialog(
    filter_list: Option<&str>,
    default_path: Option<&Path>,
) -> Result<Response> {
    open_dialog(filter_list, default_path, DialogType::MultipleFiles)
}

/// Open save dialog
pub fn open_save_dialog(
    filter_list: Option<&str>,
    default_path: Option<&Path>,
) -> Result<Response> {
    open_dialog(filter_list, default_path, DialogType::SaveFile)
}

/// Open save dialog
pub fn open_pick_folder(default_path: Option<&Path>) -> Result<Response> {
    open_dialog(None, default_path, DialogType::PickFolder)
}

pub fn open_dialog(
    filter_list: Option<&str>,
    default_path: Option<&Path>,
    dialog_type: DialogType,
) -> Result<Response> {
    let result;
    let filter_list_cstring;
    let default_path_cstring;

    let filter_list_ptr = match filter_list {
        Some(fl_str) => {
            filter_list_cstring = CString::new(fl_str)?;
            filter_list_cstring.as_ptr()
        }
        None => std::ptr::null(),
    };

    let default_path_ptr = match default_path {
        Some(dp_str) => {
            default_path_cstring = CString::new(dp_str.to_str().ok_or_else(|| {
                NFDError::Error("unable to convert default path to utf-8".to_owned())
            })?)?;
            default_path_cstring.as_ptr()
        }
        None => std::ptr::null(),
    };

    let mut out_path: *mut c_char = std::ptr::null_mut();
    let ptr_out_path = &mut out_path as *mut *mut c_char;

    let mut out_multiple = nfdpathset_t::default();
    let ptr_out_multiple = &mut out_multiple as *mut nfdpathset_t;

    unsafe {
        result = match dialog_type {
            DialogType::SingleFile => {
                NFD_OpenDialog(filter_list_ptr, default_path_ptr, ptr_out_path)
            }

            DialogType::MultipleFiles => {
                NFD_OpenDialogMultiple(filter_list_ptr, default_path_ptr, ptr_out_multiple)
            }

            DialogType::SaveFile => NFD_SaveDialog(filter_list_ptr, default_path_ptr, ptr_out_path),

            DialogType::PickFolder => NFD_PickFolder(default_path_ptr, ptr_out_path),
        };

        match result {
            nfdresult_t::NFD_OKAY => {
                if dialog_type == DialogType::MultipleFiles {
                    let count = NFD_PathSet_GetCount(&out_multiple);
                    let mut res = Vec::with_capacity(count);
                    for i in 0..count {
                        let path = CStr::from_ptr(NFD_PathSet_GetPath(&out_multiple, i))
                            .to_string_lossy()
                            .into_owned();
                        res.push(PathBuf::from(path));
                    }

                    NFD_PathSet_Free(ptr_out_multiple);

                    Ok(Response::OkayMultiple(res))
                } else {
                    Ok(Response::Okay(PathBuf::from(
                        CStr::from_ptr(out_path).to_string_lossy().into_owned(),
                    )))
                }
            }

            nfdresult_t::NFD_CANCEL => Ok(Response::Cancel),
            nfdresult_t::NFD_ERROR => Err(NFDError::Error(
                CStr::from_ptr(NFD_GetError())
                    .to_string_lossy()
                    .into_owned(),
            )),
        }
    }
}
