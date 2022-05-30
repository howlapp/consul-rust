use std::collections::HashMap;

use async_trait::async_trait;

use crate::{
    agent::AgentService,
    errors::Result,
    payload::{
        CatalogDeregistrationPayload, CatalogRegistrationPayload, QueryMeta, QueryOptions,
        WriteMeta, WriteOptions,
    },
    request::{get, put},
    sealed::Sealed,
    Client,
};

#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct ServiceWeights {
    passing: u32,
    warning: u32,
}

/// A node within the cluster gossip pool.
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct Node {
    /// The ID of the node.
    #[serde(rename = "ID")]
    id: String,
    /// The name of the node.
    #[serde(rename = "Node")]
    node: String,
    /// The address of the node.
    #[serde(rename = "Address")]
    address: String,
    /// The datacenter of the node.
    #[serde(rename = "Datacenter")]
    datacenter: String,
    /// The tags of the node.
    #[serde(rename = "TaggedAddresses")]
    tagged_addresses: HashMap<String, String>,
    /// The meta data of the node.
    #[serde(rename = "Meta")]
    meta: HashMap<String, String>,
    create_index: u64,
    modify_index: u64,
}

/// A service defined within the Agent catalog.
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct CatalogService {
    /// The ID of the service.
    #[serde(rename = "ID")]
    id: String,
    /// The node the service is associated with.
    #[serde(rename = "Node")]
    node: String,
    /// The address of the node.
    #[serde(rename = "Address")]
    address: String,
    /// The datacenter of the node running the service.
    #[serde(rename = "Datacenter")]
    datacenter: String,
    /// A map of addresses tagged to the node hosting the service.
    #[serde(rename = "TaggedAddresses")]
    tagged_addresses: HashMap<String, String>,
    /// Metadata attached to the node this service is hosted on.
    #[serde(rename = "NodeMeta")]
    node_meta: HashMap<String, String>,
    /// The ID of the service.
    #[serde(rename = "ServiceID")]
    service_id: String,
    /// The name of the service.
    #[serde(rename = "ServiceName")]
    service_name: String,
    /// The address of the service.
    #[serde(rename = "ServiceAddress")]
    service_address: String,
    /// Tags assigned to the service.
    #[serde(rename = "ServiceTags")]
    service_tags: Vec<String>,
    /// Metadata assigned to the service.
    #[serde(rename = "ServiceMeta")]
    service_meta: HashMap<String, String>,
    /// The port of the service.
    #[serde(rename = "ServicePort")]
    service_port: u32,
    #[serde(rename = "ServiceWeights")]
    service_weights: ServiceWeights,
    #[serde(rename = "ServiceEnableTagOverride")]
    service_enable_tag_override: bool,
    #[serde(rename = "CreateIndex")]
    create_index: u64,
    #[serde(rename = "ModifyIndex")]
    modify_index: u64,
}

/// A response datatype containing a [Node] and its services.
#[derive(Eq, Default, PartialEq, Serialize, Deserialize, Debug)]
#[serde(default)]
pub struct CatalogNode {
    /// The node stored in the catalog.
    #[serde(rename = "Node")]
    node: Option<Node>,
    /// The services associated with the node.
    #[serde(rename = "Services")]
    services: HashMap<String, AgentService>,
}

/// This trait provides methods for interacting with the Agent catalogue.
#[async_trait]
pub trait Catalog: Sealed {
    /// This method is a low-level mechanism for registering or updating
    /// entries in the catalog. It is usually preferable to instead use methods
    /// defined in the `Agent` trait for registration as they are simpler and
    /// perform anti-entropy.
    ///
    /// For more information, consult https://www.consul.io/api-docs/catalog#register-entity.
    async fn register(
        &self,
        reg: &CatalogRegistrationPayload,
        q: Option<&WriteOptions>,
    ) -> Result<((), WriteMeta)>;

    /// This method is a low-level mechanism for directly removing entries from
    /// the Catalog. It is usually preferable to instead use methods defined
    /// in the `Agent` trait for deregistration as they are simpler and
    /// perform anti-entropy.
    ///
    /// For more information, consult https://www.consul.io/api/catalog.html#deregister-entity.
    async fn deregister(
        &self,
        dereg: &CatalogDeregistrationPayload,
        q: Option<&WriteOptions>,
    ) -> Result<((), WriteMeta)>;

    /// This method returns the list of all known datacenters. The datacenters
    /// will be sorted in ascending order based on the estimated median round
    /// trip time from the server to the servers in that datacenter.
    ///
    /// For more information, consult https://www.consul.io/api/catalog.html#list-datacenters
    async fn list_datacenters(&self) -> Result<(Vec<String>, QueryMeta)>;

    /// This endpoint and returns the nodes registered in a given datacenter.
    ///
    /// For more information, consult https://www.consul.io/api/catalog.html#list-nodes.
    async fn list_datacenter_nodes(
        &self,
        q: Option<&QueryOptions>,
    ) -> Result<(Vec<Node>, QueryMeta)>;

    /// This endpoint returns the services registered in a given datacenter.
    ///
    /// For more information, consult https://www.consul.io/api-docs/catalog#list-services.
    async fn list_datacenter_services(
        &self,
        q: Option<&QueryOptions>,
    ) -> Result<(HashMap<String, Vec<String>>, QueryMeta)>;
}

#[async_trait]
impl Catalog for Client {
    async fn register(
        &self,
        reg: &CatalogRegistrationPayload,
        q: Option<&WriteOptions>,
    ) -> Result<((), WriteMeta)> {
        put("/v1/session/create", Some(reg), &self.config, HashMap::new(), q).await
    }

    async fn deregister(
        &self,
        dereg: &CatalogDeregistrationPayload,
        q: Option<&WriteOptions>,
    ) -> Result<((), WriteMeta)> {
        put("/v1/catalog/deregister", Some(dereg), &self.config, HashMap::new(), q).await
    }

    async fn list_datacenters(&self) -> Result<(Vec<String>, QueryMeta)> {
        get("/v1/catalog/datacenters", &self.config, HashMap::new(), None).await
    }

    async fn list_datacenter_nodes(
        &self,
        q: Option<&QueryOptions>,
    ) -> Result<(Vec<Node>, QueryMeta)> {
        get("/v1/catalog/nodes", &self.config, HashMap::new(), q).await
    }

    async fn list_datacenter_services(
        &self,
        q: Option<&QueryOptions>,
    ) -> Result<(HashMap<String, Vec<String>>, QueryMeta)> {
        get("/v1/catalog/services", &self.config, HashMap::new(), q).await
    }
}
