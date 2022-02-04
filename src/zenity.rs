use std::{
    ffi::OsString,
    os::unix::ffi::OsStringExt,
    path::{Path, PathBuf},
    process::Command,
};

fn exec(
    title: &str,
    filter_list: Option<crate::FilterList<'_>>,
    default_path: Option<&Path>,
    opt: Option<&str>,
) -> crate::Result<crate::Response> {
    let mut cmd = Command::new("zenity");
    cmd.args(&["--file-selection", "--title", title]);

    if let Some(dp) = default_path {
        cmd.arg("--filename");
        cmd.arg(dp);
    }

    if let Some(fl) = filter_list {
        let mut type_buf = String::with_capacity(128);
        let mut filter_name = String::with_capacity(128);

        for exts in fl {
            if exts.is_empty() {
                return Err("empty type list provided".into());
            }

            filter_name.clear();
            type_buf.clear();

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

                if !type_buf.is_empty() {
                    filter_name.push_str(" ");
                }

                type_buf.push_str("*.");
                type_buf.push_str(typ);
            }

            cmd.arg("--file-filter");
            cmd.arg(format!("{} | {}", filter_name, type_buf));
        }

        cmd.arg("--file-filter");
        cmd.arg("*.* | *.*");
    }

    let multiple = if let Some(opt) = opt {
        cmd.arg(opt);
        opt == "--multiple"
    } else {
        false
    };

    cmd.stdout(std::process::Stdio::piped());
    let output = cmd
        .output()
        .map_err(|_| crate::NfdError::from("zenity not installed"))?;

    if !output.status.success() {
        return Ok(crate::Response::Cancel);
    }

    if multiple {
        let mut out = output.stdout;
        // Pop of the trailing newline
        out.pop();

        let mut indices = Vec::with_capacity(2);
        for (i, b) in out.iter().enumerate() {
            if *b == b'|' {
                indices.push(i + 1);
            }
        }

        indices.reverse();

        let mut paths = Vec::new();
        for ind in indices {
            paths.push(PathBuf::from(OsString::from_vec(out.split_off(ind))));
            out.pop();
        }

        paths.push(PathBuf::from(OsString::from_vec(out)));

        Ok(crate::Response::OkayMultiple(paths))
    } else {
        Ok(crate::Response::Okay(PathBuf::from(OsString::from_vec(
            output.stdout,
        ))))
    }
}

pub(crate) fn open_dialog(
    filter_list: Option<crate::FilterList<'_>>,
    default_path: Option<&Path>,
) -> crate::Result<crate::Response> {
    exec("Open File", filter_list, default_path, None)
}

pub(crate) fn open_dialog_multi(
    filter_list: Option<crate::FilterList<'_>>,
    default_path: Option<&Path>,
) -> crate::Result<crate::Response> {
    exec("Open Files", filter_list, default_path, Some("--multiple"))
}

pub(crate) fn open_save_dialog(
    filter_list: Option<crate::FilterList<'_>>,
    default_path: Option<&Path>,
) -> crate::Result<crate::Response> {
    exec("Save File", filter_list, default_path, Some("--save"))
}

pub(crate) fn pick_folder(default_path: Option<&Path>) -> crate::Result<crate::Response> {
    exec("Select Folder", None, default_path, Some("--directory"))
}
