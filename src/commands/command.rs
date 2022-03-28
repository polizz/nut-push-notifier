use color_eyre::Report;
use crate::args::{ListArgs, NotifyArgs};

pub trait Command<T> {
    fn execute(args: impl CommandArgs<T>) -> Result<(), Report>;
}

pub trait CommandArgs<T> {
    fn get_args(self) -> T;
}

impl CommandArgs<NotifyArgs> for NotifyArgs {
    fn get_args(self) -> NotifyArgs {
        self
    }
}

impl CommandArgs<ListArgs> for ListArgs {
    fn get_args(self) -> ListArgs {
        self
    }
}
