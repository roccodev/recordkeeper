use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

use anyhow::{anyhow, Result};
use gloo::file::{Blob, FileReadError, ObjectUrl};
use log::info;
use recordkeeper::{SaveData, SaveFile, SaveResult};
use wasm_bindgen::JsCast;
use web_sys::HtmlAnchorElement;
use yew::prelude::*;

use crate::dialog::{Dialog, DialogLayout, DialogQueue, Severity};

#[derive(Default)]
pub struct SaveManager {
    save_buffers: [Option<SaveEntry>; 4],
    // discriminant for PartialEq
    change_id: usize,
}

struct SaveEntry {
    file: SaveFile,
    name: String,
}

pub enum EditAction {
    Load(Vec<u8>, String),
    Save,
    ClearError,
    Edit(EditFn),
    TryEdit(TryEditFn),
    Download,
}

pub type EditFn = Box<dyn FnOnce(&mut SaveData)>;
pub type TryEditFn = Box<dyn FnOnce(&mut SaveData) -> Result<()>>;

#[derive(Properties, PartialEq)]
pub struct SaveProviderProps {
    #[prop_or_default]
    pub children: Children,
}

#[derive(PartialEq, Clone)]
pub struct SaveContext {
    handle: UseReducerHandle<SaveReducer>,
}

#[derive(Default, PartialEq)]
struct SaveReducer {
    manager: Rc<RefCell<SaveManager>>,
    change_id: usize,
    error_dialog: Option<Dialog>,
}

#[function_component]
pub fn SaveProvider(props: &SaveProviderProps) -> Html {
    let manager = use_reducer(SaveReducer::default);
    let dialog = use_context::<DialogQueue>().unwrap();

    if let Some(error) = &manager.error_dialog {
        manager.dispatch(EditAction::ClearError);
        dialog.dispatch(Some(error.clone()));
    }

    html! {
        <ContextProvider<SaveContext> context={SaveContext::new(manager)}>
            { for props.children.iter() }
        </ContextProvider<SaveContext>>
    }
}

impl SaveManager {
    pub fn get(&self) -> &SaveFile {
        self.save_buffers[0].as_ref().map(|s| &s.file).unwrap()
    }

    pub fn get_mut(&mut self) -> &mut SaveFile {
        self.save_buffers[0].as_mut().map(|s| &mut s.file).unwrap()
    }

    pub fn is_loaded(&self) -> bool {
        self.save_buffers[0].is_some()
    }

    fn load(&mut self, bytes: &[u8], file_name: String) -> SaveResult<()> {
        let save = SaveFile::from_bytes(bytes)?;

        info!(
            "Loaded save file, last saved {} {}",
            save.save().timestamp.to_iso_date(),
            save.save().timestamp.to_iso_time()
        );

        self.save_buffers.fill_with(|| None); // TODO replace with fill with Clone
        self.save_buffers[0] = Some(SaveEntry {
            file: save,
            name: file_name,
        });
        self.change_id = if self.change_id == 0 { 1 } else { 0 };
        Ok(())
    }

    fn back_up_and_save(&mut self) -> SaveResult<()> {
        assert!(2 <= self.save_buffers.len());
        for i in (2..self.save_buffers.len()).rev() {
            self.save_buffers.swap(i, i - 1);
        }
        // Least recently used buffer is now in position 1,
        // replace it with the new backup
        // self.save_buffers[1] = self.save_buffers[0].clone(); // TODO
        self.save_buffers[0].as_mut().unwrap().file.write()?;
        self.mark_change();
        Ok(())
    }

    fn download(&self) -> Result<()> {
        let slot = self.save_buffers[0].as_ref().unwrap();
        let bytes = Blob::new(slot.file.bytes());
        let url = ObjectUrl::from(bytes);
        let link = web_sys::window()
            .expect("no window")
            .document()
            .expect("no document")
            .create_element("a")
            .map_err(|v| anyhow!("Download DOM error: {v:?}"))?
            .dyn_into::<HtmlAnchorElement>()
            .unwrap();
        link.set_href(&url);
        link.set_download(&slot.name);
        link.click();
        Ok(())
    }

    fn mark_change(&mut self) {
        self.change_id = self.change_id.wrapping_add(1);
    }
}

impl SaveContext {
    fn new(handle: UseReducerHandle<SaveReducer>) -> Self {
        Self { handle }
    }

    pub fn submit_action(&self, action: EditAction) {
        self.handle.dispatch(action);
    }

    pub fn edit(&self, editor: impl FnOnce(&mut SaveData) + 'static) {
        self.submit_action(EditAction::Edit(Box::new(editor)))
    }

    pub fn try_edit(&self, editor: impl FnOnce(&mut SaveData) -> Result<()> + 'static) {
        self.submit_action(EditAction::TryEdit(Box::new(editor)))
    }

    pub fn get(&self) -> Ref<'_, SaveManager> {
        self.handle.manager.borrow()
    }

    pub fn on_file_upload(&self, name: String, data: Result<Vec<u8>, FileReadError>) {
        self.submit_action(EditAction::Load(data.unwrap(), name));
    }
}

impl Reducible for SaveReducer {
    type Action = EditAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut handle = self.manager.borrow_mut();
        let res: Result<()> = match action {
            EditAction::Load(bytes, name) => handle.load(&bytes, name).map_err(Into::into),
            EditAction::Save => handle.back_up_and_save().map_err(Into::into),
            EditAction::Edit(callback) => {
                callback(handle.get_mut().save_mut());
                handle.mark_change();
                Ok(())
            }
            EditAction::TryEdit(callback) => {
                callback(handle.get_mut().save_mut()).map(|_| handle.mark_change())
            }
            EditAction::ClearError => Ok(()),
            EditAction::Download => handle.download(),
        };
        Rc::new(Self {
            manager: Rc::clone(&self.manager),
            change_id: handle.change_id,
            error_dialog: res.err().map(|e| {
                DialogLayout::Ok {
                    title: None,
                    message: e.to_string().into(),
                    severity: Severity::Error,
                }
                .into()
            }),
        })
    }
}

impl PartialEq for SaveManager {
    fn eq(&self, other: &Self) -> bool {
        self.change_id.eq(&other.change_id)
    }
}
