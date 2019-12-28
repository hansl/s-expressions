pub enum Error {
    IndexOutOfBoundError(usize),
    EssentialContentCannotBeAppended,
    EssentialContentCannotBePrepended,
    ContentShouldNotBeRemoved,
}
