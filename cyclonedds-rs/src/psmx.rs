//! # Publish Subscribe Message Exchange
//! ## Overview
//!
//! The Publish Subscribe Message Exchange (PSMX) interface provides support for off-loading data communication from the network stack of Cyclone to arbitrary pub-sub transport implementations.[^1] This section provides an overview of the structure and introduces some terminology, details on specific operations are provided at the definitions of those operations.
//!
//! A Cyclone DDS Domain consists of a plurality of DDS Domain Entities, which are the representations of the DDS Domain in a specific process.
//!
//! A PSMX Plugin provides an implementation of the PSMX interface, allowing the instantiation of a PSMX Instance to establish a connection between a DDS Domain Entity and a PSMX Domain. The PSMX Plugin is specified as a library and a PSMX Instance Name. The library is loaded in the process and a constructor function provided by the library is invoked to create and initialize a PSMX Instance given the PSMX Instance Name (and a configuration string). In principle a specific library may be configured multiple times in a single DDS Domain.
//!
//! The PSMX Instance Name is assumed to uniquely identify the PSMX Domain in the DDS Domain. From the PSMX Instance Name, a numeric PSMX Instance ID is derived that uniquely identifies the PSMX Domain within the DDS Domain Entity and is assumed to uniquely identify the PSMX Domain in the DDS Domain.[^2]
//!
//! Each PSMX Instance chooses a 16-byte PSMX Locator[^3] such that any pair of instances with the same PSMX Locator communicate, and any pair with different locators do not communicate.[^4]
//!
//! DDS Topics, DDS Readers and DDS Writers are mapped to corresponding objects in PSMX Instances. For DDS Readers and DDS Writers, the application can restrict the set of PSMX Instances for which the mapping is created using the “PSMX Instances” QoS setting, and the PSMX Instances can refuse mapping based on type and QoS information.
//!
//! DDS Topic Entities are representations of the topics in the DDS Domain, such that two identical definitions of a topic in a DDS Domain Entity give rise to two application-level DDS Topic Entities, but only to a single topic in the DDS Domain Entity and thus also only one PSMX Topic object per PSMX Instance.
//!
//! Each DDS Reader/Writer is mapped to a set of PSMX Reader/Writer Endpoints, one for each PSMX Instance in the “PSMX Instances” QoS that accepts the type and reader/writer QoS. An associated set of PSMX Domains consisting of the PSMX Domains for which PSMX Reader/Writer Endpoints have been created is assumed to exist.
//!
//! The PSMX Domain is assumed to deliver data published by the PSMX Writer associated with DDS Writer X to all PSMX Readers associated with the DDS Readers Ys that match X[^5], optionally excluding DDS Readers in the same Domain Entity as X. It is assumed to not deliver data to other DDS Readers in the DDS Domain. It is assumed to do this with a quality of service compatible with the DDS QoS.
//!
//! Readers associated with DDS Readers in the same DDS Domain Entity.
//!
//! If the intersection of the sets of PSMX Domains of a DDS Reader and a DDS Writer in a DDS Domain:
//!
//! - is empty, off-loading data transfer to PSMX (for this pair) is not possible;
//!
//! - contains one instance, that PSMX Domain is eligible for off-loading data transfer;
//!
//! - contains multiple instances, the configuration is invalid.
//!
//! If an eligible PSMX Domain exists and the PSMX Locators for the corresponding two PSMX Instances are the same, then PSMX is used to transfer data.
//!
//! The PSMX objects are represented in the interface as pointers to “dds_psmx”, “dds_psmx_topic”, “dds_psmx_endpoint”. The PSMX Plugin is responsible for allocating and freeing these. It is expected that the PSMX Plugin internally uses an extended version of these types to store any additional data it needs. E.g., a hypothetical “weed” PSMX Plugin could do:
//!```c
//! struct psmx_weed {
//!   struct dds_psmx c;
//!   weed_root *x;
//! };
//! ```
//! The creator function mentioned above is required to be called NAME_create_psmx, where NAME is the value of the “name” attribute of the PubSubMessageExchange interface configuration element. It must have the following signature:
//!```c
//! dds_return_t NAME_create_psmx (
//!   struct dds_psmx **psmx_instance,
//!   dds_psmx_instance_id_t identifier,
//!   const char *config)
//! ```
//! Where
//! *psmx_instance must be set point to a new PSMX Instance on success and may be left undefined on error identifier contains the numeric PSMX Instance ID config the PSMX configuration from the “config” attribute of the PubSubMessageExchange interface configuration element.
//!
//! The “config” argument is a contiguous sequence of characters terminated by the first double-\0. Each \0-terminated character sequence is a string that consists of KEY=VALUE pairs, where each K-V pair is terminated by a semicolon.
//!
//! If the configuration string as set in Cyclone DDS configuration contains a “INSTANCE_NAME” key, its value is used as the PSMX Instance Name. If the key is not included, the value of the “name” attribute of the corresponding PubSubMessageExchange element in configuration is used as the PSMX Instance Name. In all cases, looking up the “INSTANCE_NAME” key in the configuration string using dds_psmx_get_config_option_value will return the PSMX Instance Name as its value.
//!
//! The behaviour of the constructor function is dependent on the interface version it implements:
//!
//! - For version 0, it is responsible for setting:
//!
//!     - ops to the addresses of the various functions implementing the operations
//!
//!     - instance_name to a “dds_alloc” allocated string
//!
//!     - instance_id to the “identifier” argument
//!
//! and for zero-initializing the other fields. At some point after this initialization, and once it is prepared to handle the “get_node_id” operation, it must invoke the “dds_psmx_init_generic” to complete the initialization.
//!
//! - For version 1, it is responsible for setting:
//!
//!     - ops
//!
//! All other fields will be initialized by the Cyclone DDS after succesful return and the “get_node_id” operation also will be invoked after the constructor returned.
//!
//! Whether the plugin implements version 0 or version 1 of the interface is controlled by the function pointers in “dds_psmx_ops_t”. If “create_topic” and “deinit” are non-null, it is version 0; if both are null it is version 1. Neither “create_topic_type” nor “delete_psmx” is touched by Cyclone DDS if the interface is version 0, allowing for binary backwards compatibility.
//!
//! &#8212; Footnotes: &#8212;
//!
//! [^1]: In particular including shared-memory based mechanisms.
//!
//! [^2]: Internally, the name is not used for anything other than the generation of the numeric id.
//!
//! [^3]: Confusingly named “node identifier” in the interface, even though it has nothing to do with the numeric PSMX Domain identifier.
//!
//! [^4]: This typically matches a machine when the transport is shared memory.
//!
//! [^5]: That is, the matching rules between Readers and Writers defined in the DDS specification.
//!
