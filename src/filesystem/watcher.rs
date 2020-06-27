use notify::DebouncedEvent;
use std::path::PathBuf;

pub enum FileAction {
    Resize(PathBuf),
    Delete(PathBuf),
    Rename(PathBuf, PathBuf),
}

pub fn get_watcher_event_action(event: &DebouncedEvent) -> Option<FileAction> {
    match event {
        DebouncedEvent::Create(path) | DebouncedEvent::Write(path) => {
            Some(FileAction::Resize(path.to_path_buf()))
        }
        DebouncedEvent::Remove(path) => Some(FileAction::Delete(path.to_path_buf())),
        DebouncedEvent::Rename(path, dest) => {
            Some(FileAction::Rename(path.to_path_buf(), dest.to_path_buf()))
        }

        // Notice* occurs immediately, events above are debounced for same events
        DebouncedEvent::NoticeWrite(_)
        | DebouncedEvent::NoticeRemove(_)
        | DebouncedEvent::Chmod(_)
        | DebouncedEvent::Rescan
        | DebouncedEvent::Error(_, _) => None,
    }
}
