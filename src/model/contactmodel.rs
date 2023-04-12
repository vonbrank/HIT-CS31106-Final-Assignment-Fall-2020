struct ContactState {
    contact_list_state: Vec<ContactList>,
}

struct ContactList {
    name: String,
    data: Vec<Contact>,
}

#[derive(Clone)]
pub struct Contact {
    pub name: String,
    pub telephone: String,
}
