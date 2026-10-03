//! Shopify Admin webhook topics.

use crate::common::{Webhook, WebhookBuilder};
use crate::error::Result;
use crate::provider::Provider;

provider_events! {
    /// Common Shopify webhook topics (`X-Shopify-Topic`).
    pub enum Event {
        /// `orders/create`
        OrdersCreate => "orders/create",
        /// `orders/paid`
        OrdersPaid => "orders/paid",
        /// `orders/cancelled`
        OrdersCancelled => "orders/cancelled",
        /// `orders/fulfilled`
        OrdersFulfilled => "orders/fulfilled",
        /// `orders/updated`
        OrdersUpdated => "orders/updated",
        /// `orders/partially_fulfilled`
        OrdersPartiallyFulfilled => "orders/partially_fulfilled",
        /// `products/create`
        ProductsCreate => "products/create",
        /// `products/update`
        ProductsUpdate => "products/update",
        /// `products/delete`
        ProductsDelete => "products/delete",
        /// `customers/create`
        CustomersCreate => "customers/create",
        /// `customers/update`
        CustomersUpdate => "customers/update",
        /// `customers/delete`
        CustomersDelete => "customers/delete",
        /// `checkouts/create`
        CheckoutsCreate => "checkouts/create",
        /// `checkouts/update`
        CheckoutsUpdate => "checkouts/update",
        /// `refunds/create`
        RefundsCreate => "refunds/create",
        /// `app/uninstalled`
        AppUninstalled => "app/uninstalled",
        /// `carts/create`
        CartsCreate => "carts/create",
        /// `carts/update`
        CartsUpdate => "carts/update",
        /// `inventory_levels/update`
        InventoryLevelsUpdate => "inventory_levels/update",
        /// `fulfillments/create`
        FulfillmentsCreate => "fulfillments/create",
        /// `fulfillments/update`
        FulfillmentsUpdate => "fulfillments/update",
        /// `shop/update`
        ShopUpdate => "shop/update",
        /// `themes/publish`
        ThemesPublish => "themes/publish",
    }
}

/// Start a Shopify webhook endpoint builder.
#[must_use]
pub fn endpoint() -> WebhookBuilder {
    WebhookBuilder::new(Provider::Shopify)
}

/// Build a Shopify webhook from typed topics.
pub fn webhook(
    url: impl Into<String>,
    events: impl IntoIterator<Item = Event>,
) -> Result<Webhook> {
    endpoint().url(url).events(events).build()
}
