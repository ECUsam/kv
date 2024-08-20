pub trait CommandService{
    fn execut(self, stor: &impl Storage) -> CommandResponse;
}

