use vector_core::internal_event::InternalEvent;

#[derive(Debug)]
pub struct AddTagsTagOverwritten<'a> {
    pub(crate) tag: &'a str,
}

impl<'a> InternalEvent for AddTagsTagOverwritten<'a> {
    fn emit_logs(&self) {
        debug!(message = "Tag overwritten.", tag = %self.tag, internal_log_rate_secs = 30);
    }
}

#[derive(Debug)]
pub struct AddTagsTagNotOverwritten<'a> {
    pub(crate) tag: &'a str,
}

impl<'a> InternalEvent for AddTagsTagNotOverwritten<'a> {
    fn emit_logs(&self) {
        debug!(message = "Tag not overwritten.", tag = %self.tag, internal_log_rate_secs = 30);
    }
}
