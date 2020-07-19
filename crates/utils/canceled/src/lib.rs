#[derive(PartialEq, Eq)]
pub struct Canceled;

impl Canceled {
    pub fn throw() -> ! {
        // Don't print backtrace
        std::panic::resume_unwind(Box::new(Canceled))
    }

    pub fn cancel_if(db: &salsa::Runtime) {
        if db.is_current_revision_canceled() {
            Self::throw();
        }
    }
}
