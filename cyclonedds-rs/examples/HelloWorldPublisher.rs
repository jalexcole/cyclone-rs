use cyclonedds_rs::domain::DomainParticipant;

fn main() {
    let mut participant: DomainParticipant = DomainParticipant::new(0).unwrap();
    let mut publisher = participant.publisher().unwrap();
    println!("=== [Publisher]  Waiting for a reader to be discovered ...\n");
    let mut writer = publisher.create_datawriter().unwrap();
    let mut msg = HelloWorldData::Msg {
        userID: 1,
        message: "Hello World".to_string(),
    };

    println!("=== [Publisher]  Writing : ");

    writer.write(&msg).unwrap();

    drop(participant)
   
}

pub mod HelloWorldData {
    use cyclonedds_rs::topic::TopicType;
    use serde::Serialize;

    /// ```idl
    /// module HelloWorldData {
    ///   struct Msg {
    ///     @key
    ///    long userID;
    ///     string message;
    ///   };
    /// };
    /// ```
    #[derive(Serialize, Debug, Clone)]
    pub struct Msg {
        pub userID: i64,
        pub message: String,
    }

    impl TopicType for Msg {
    }
}
