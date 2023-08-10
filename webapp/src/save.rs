use std::{
    cell::{Ref, RefCell, RefMut},
    rc::Rc,
};

use gloo::file::FileReadError;
use recordkeeper::{SaveData, SaveFile, SaveResult};
use yew::prelude::*;

use crate::dialog::{Dialog, DialogLayout, DialogQueue, Severity};

#[derive(Default)]
pub struct SaveManager {
    save_buffers: [Option<SaveFile>; 4],
    // discriminant for PartialEq
    change_id: usize,
}

pub enum EditAction {
    Load(Vec<u8>),
    Save,
    ClearError,
    Edit(Box<dyn FnOnce(&mut SaveData)>),
}

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
        self.save_buffers[0].as_ref().unwrap()
    }

    pub fn get_mut(&mut self) -> &mut SaveFile {
        self.save_buffers[0].as_mut().unwrap()
    }

    pub fn is_loaded(&self) -> bool {
        self.save_buffers[0].is_some()
    }

    fn load(&mut self, bytes: &[u8]) -> SaveResult<()> {
        let save = SaveFile::from_bytes(bytes)?;
        self.save_buffers.fill_with(|| None); // TODO replace with fill with Clone
        self.save_buffers[0] = Some(save);
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
        self.save_buffers[0].as_mut().unwrap().write()?;
        self.change_id = self.change_id.wrapping_add(1);
        Ok(())
    }
}

impl SaveContext {
    fn new(handle: UseReducerHandle<SaveReducer>) -> Self {
        Self { handle }
    }

    pub fn submit_action(&self, action: EditAction) {
        self.handle.dispatch(action);
    }

    pub fn get(&self) -> Ref<'_, SaveManager> {
        self.handle.manager.borrow()
    }

    pub fn on_file_upload(&self, data: Result<Vec<u8>, FileReadError>) {
        self.submit_action(EditAction::Load(data.unwrap()));
    }
}

impl Reducible for SaveReducer {
    type Action = EditAction;

    fn reduce(self: Rc<Self>, action: Self::Action) -> Rc<Self> {
        let mut handle = self.manager.borrow_mut();
        let res = match action {
            EditAction::Load(bytes) => handle.load(&bytes),
            EditAction::Save => handle.back_up_and_save(),
            EditAction::Edit(mut callback) => {
                callback(handle.get_mut().save_mut());
                Ok(())
            }
            EditAction::ClearError => Ok(()),
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
