use cyclonedds_rs::domain::DomainParticipant;

fn main() {
    let mut participant: DomainParticipant = DomainParticipant::new(0).unwrap();
    let publisher = participant.publisher();

    let msg = HelloWorldData::Msg {
        userID: 123,
        message: "Hello Cyclone DDS".to_string(),
    };

    
    // topic = Topic::new(&participant, "HelloWorldData_Msg");
    // writer = DataWriter::new(&participant, &topic);
}

pub mod HelloWorldData {
    /// ```idl
    /// module HelloWorldData {
    ///   struct Msg {
    ///     @key
    ///    long userID;
    ///     string message;
    ///   };
    /// };
    /// ```
    pub struct Msg {
        pub userID: i64,
        pub message: String,
    }
}
