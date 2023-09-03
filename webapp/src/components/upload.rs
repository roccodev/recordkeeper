//! File upload utility

use gloo::file::{callbacks::FileReader, File, FileReadError};
use web_sys::{FileList, HtmlInputElement};
use yew::prelude::*;

use crate::save::SaveContext;

#[derive(Properties, PartialEq)]
pub struct UploadButtonProps {
    pub children: Children,
}

#[function_component]
pub fn UploadButton(props: &UploadButtonProps) -> Html {
    let save = use_context::<SaveContext>().unwrap();
    let reader_state = use_state(|| None::<FileReader>);

    let on_upload = Callback::from(move |e: Event| {
        let input: HtmlInputElement = e.target_unchecked_into();
        let save = save.clone();
        // Persist reader as the read is cancelled when the handle is dropped
        let reader = read_file(input.files(), move |name, data| {
            save.on_file_upload(name, data)
        });
        reader_state.set(reader);
    });

    html! {
        <>
            <label for="file-upload" class="button">{for props.children.iter() }</label>
            <input
                id="file-upload"
                type="file"
                onchange={on_upload}
                class="button"
                style="position: fixed; top: -100em" accept=".sav"
            />
        </>
    }
}

fn read_file(
    files: Option<FileList>,
    callback: impl Fn(String, Result<Vec<u8>, FileReadError>) + 'static,
) -> Option<FileReader> {
    let file = match files
        .and_then(|files| js_sys::try_iter(&files).unwrap())
        .and_then(|mut i| i.next())
    {
        Some(file) => file,
        _ => return None,
    }
    .unwrap();
    let file = File::from(web_sys::File::from(file));
    let name = file.name();
    gloo::file::callbacks::read_as_bytes(&file, move |res| callback(name, res)).into()
}
