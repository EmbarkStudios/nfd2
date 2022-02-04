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

// BEGIN - Embark standard lints v0.3
// do not change or add/remove here, but one can add exceptions after this section
// for more info see: <https://github.com/EmbarkStudios/rust-ecosystem/issues/59>
#![warn(
    clippy::all,
    clippy::await_holding_lock,
    clippy::dbg_macro,
    clippy::debug_assert_with_mut_call,
    clippy::doc_markdown,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::exit,
    clippy::explicit_into_iter_loop,
    clippy::filter_map_next,
    clippy::fn_params_excessive_bools,
    clippy::if_let_mutex,
    clippy::imprecise_flops,
    clippy::inefficient_to_string,
    clippy::large_types_passed_by_value,
    clippy::let_unit_value,
    clippy::linkedlist,
    clippy::lossy_float_literal,
    clippy::macro_use_imports,
    clippy::map_err_ignore,
    clippy::map_flatten,
    clippy::map_unwrap_or,
    clippy::match_on_vec_items,
    clippy::match_same_arms,
    clippy::match_wildcard_for_single_variants,
    clippy::mem_forget,
    clippy::mismatched_target_os,
    clippy::needless_borrow,
    clippy::needless_continue,
    clippy::option_option,
    clippy::pub_enum_variant_names,
    clippy::ref_option_ref,
    clippy::rest_pat_in_fully_bound_structs,
    clippy::string_add_assign,
    clippy::string_add,
    clippy::string_to_string,
    clippy::suboptimal_flops,
    clippy::todo,
    clippy::unimplemented,
    clippy::unnested_or_patterns,
    clippy::unused_self,
    clippy::verbose_file_reads,
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms
)]
// END - Embark standard lints v0.3

mod common;
pub mod error;

pub use common::FilterList;
use error::NfdError;
use std::path::{Path, PathBuf};

#[cfg_attr(
    not(any(
        target_os = "windows",
        target_os = "macos",
        target_os = "ios",
        feature = "zenity"
    )),
    path = "gtk.rs"
)]
#[cfg_attr(
    all(
        not(any(target_os = "windows", target_os = "macos", target_os = "ios")),
        feature = "zenity"
    ),
    path = "zenity.rs"
)]
mod imp;

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
    filter: Option<FilterList<'a>>,
    default_path: Option<&'a Path>,
    dialog_type: DialogType,
}

impl<'a> DialogBuilder<'a> {
    pub fn new(dialog_type: DialogType) -> Self {
        Self {
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
    pub fn filter(&mut self, filter: FilterList<'a>) -> &mut Self {
        self.filter = Some(filter);
        self
    }

    /// Specify the default directory to start the dialog in, otherwise the
    /// default directory will be dependent upon the host API
    pub fn default_path(&mut self, path: &'a impl AsRef<Path>) -> &mut Self {
        self.default_path = Some(path.as_ref());
        self
    }

    /// Opens the dialog and waits upon a response from the user
    pub fn open(&self) -> Result<Response> {
        open_dialog(self.filter, self.default_path, self.dialog_type)
    }
}

pub type Result<T> = std::result::Result<T, NfdError>;

/// Open single file dialog
#[inline]
pub fn open_file_dialog(
    filter_list: Option<FilterList<'_>>,
    default_path: Option<&Path>,
) -> Result<Response> {
    open_dialog(filter_list, default_path, DialogType::SingleFile)
}

/// Open mulitple file dialog
pub fn open_file_multiple_dialog(
    filter_list: Option<FilterList<'_>>,
    default_path: Option<&Path>,
) -> Result<Response> {
    open_dialog(filter_list, default_path, DialogType::MultipleFiles)
}

/// Open save dialog
pub fn open_save_dialog(
    filter_list: Option<FilterList<'_>>,
    default_path: Option<&Path>,
) -> Result<Response> {
    open_dialog(filter_list, default_path, DialogType::SaveFile)
}

/// Open folder selection dialog
pub fn open_pick_folder(default_path: Option<&Path>) -> Result<Response> {
    open_dialog(None, default_path, DialogType::PickFolder)
}

pub fn open_dialog(
    filter_list: Option<FilterList<'_>>,
    default_path: Option<&Path>,
    dialog_type: DialogType,
) -> Result<Response> {
    match dialog_type {
        DialogType::SingleFile => imp::open_dialog(filter_list, default_path),
        DialogType::MultipleFiles => imp::open_dialog_multi(filter_list, default_path),

        DialogType::SaveFile => imp::open_save_dialog(filter_list, default_path),
        DialogType::PickFolder => imp::pick_folder(default_path),
    }
}
