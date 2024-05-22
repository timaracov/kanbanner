pub type UnixTimestamp = i8;


pub struct Tag {
    name: String,
    hex_color: String,
}


pub struct Task {
    title: String,
    created_at: UnixTimestamp,
    updated_at: UnixTimestamp,
    complete_to_date: UnixTimestamp,
    tags: Vec<Tag>
}


pub struct Board {
    name: String,
    tasks: Vec<Task>
}

pub struct Participant {
    id: i8,
    name: String,
}

pub struct Project {
    boards: Vec<Board>
}
