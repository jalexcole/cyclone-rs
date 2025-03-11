# Cyclone DDS Rust Binding

This is a wrapper of the [CycloneDDS](https://github.com/eclipse-cyclonedds)
library. The intent is to have a Rust Idiomatic version of the library while not
straying from the c library.

| SubProject | Status          | Notes                                                            |
|------------|-----------------|------------------------------------------------------------------|
| Domain     | Almost Wrapped  | Requires topic to be completed                                   |
| Subscriber | Almost Wrapped  | Requires topic to be completed                                   |
| Publisher  | Almost Wrapped  | Requires topic to be completed                                   |
| Topic      | Incomplete      | Requires a way to implement a Topic Descriptor for the TopicType |
| DataReader | Incomplete      | Pending on Topic                                                 |
| DataWriter | Incomplete      | Pending on Topic                                                 |
| Dynamic    | Incomplete      |                                                                  |
| xtypes     | Not Started     |                                                                  |
| statistics | Wrapped         | Requires Testing & Documentation                                 |
| psmx       | Not Started     |                                                                  |
| qos        | Setters Wrapped | Requires Testing & Documentation                                 |
| logging    | Not Started     | The plan is to be able to pass logging to tracing                |
