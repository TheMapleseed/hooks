//! KDL emission for [`super::Webhook`] documents.

use kdl::{KdlDocument, KdlEntry, KdlNode};

use super::endpoint::Webhook;

/// Render a webhook endpoint as KDL.
pub fn webhook_to_kdl(webhook: &Webhook) -> String {
    let mut doc = KdlDocument::new();
    let mut root = KdlNode::new("webhook");
    root.push(KdlEntry::new_prop("provider", webhook.provider()));

    let mut children = KdlDocument::new();

    let mut url = KdlNode::new("url");
    url.push(KdlEntry::new(webhook.url()));
    children.nodes_mut().push(url);

    if let Some(description) = webhook.description() {
        let mut node = KdlNode::new("description");
        node.push(KdlEntry::new(description));
        children.nodes_mut().push(node);
    }

    if let Some(api_version) = webhook.api_version() {
        let mut node = KdlNode::new("api-version");
        node.push(KdlEntry::new(api_version));
        children.nodes_mut().push(node);
    }

    if let Some(secret_env) = webhook.secret_env() {
        let mut node = KdlNode::new("secret-env");
        node.push(KdlEntry::new(secret_env));
        children.nodes_mut().push(node);
    }

    let mut events = KdlNode::new("events");
    let mut event_children = KdlDocument::new();
    for event in webhook.events() {
        let mut event_node = KdlNode::new("event");
        event_node.push(KdlEntry::new(event.as_str()));
        event_children.nodes_mut().push(event_node);
    }
    events.set_children(event_children);
    children.nodes_mut().push(events);

    let mut signature = KdlNode::new("signature");
    signature.push(KdlEntry::new_prop("scheme", webhook.signature_scheme()));
    children.nodes_mut().push(signature);

    if !webhook.metadata().is_empty() {
        let mut meta = KdlNode::new("meta");
        for (key, value) in webhook.metadata() {
            meta.push(KdlEntry::new_prop(key.as_str(), value.as_str()));
        }
        children.nodes_mut().push(meta);
    }

    root.set_children(children);
    doc.nodes_mut().push(root);

    let mut rendered = doc.to_string();
    if !rendered.ends_with('\n') {
        rendered.push('\n');
    }
    rendered
}
