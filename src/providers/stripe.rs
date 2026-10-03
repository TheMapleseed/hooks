//! Stripe webhook endpoints — typed events, no payload templates.

use crate::common::{Webhook, WebhookBuilder};
use crate::error::Result;
use crate::provider::Provider;

provider_events! {
    /// Common Stripe webhook event types.
    pub enum Event {
        /// `charge.succeeded`
        ChargeSucceeded => "charge.succeeded",
        /// `charge.failed`
        ChargeFailed => "charge.failed",
        /// `charge.refunded`
        ChargeRefunded => "charge.refunded",
        /// `charge.dispute.created`
        ChargeDisputeCreated => "charge.dispute.created",
        /// `payment_intent.succeeded`
        PaymentIntentSucceeded => "payment_intent.succeeded",
        /// `payment_intent.payment_failed`
        PaymentIntentPaymentFailed => "payment_intent.payment_failed",
        /// `payment_intent.canceled`
        PaymentIntentCanceled => "payment_intent.canceled",
        /// `payment_method.attached`
        PaymentMethodAttached => "payment_method.attached",
        /// `checkout.session.completed`
        CheckoutSessionCompleted => "checkout.session.completed",
        /// `checkout.session.expired`
        CheckoutSessionExpired => "checkout.session.expired",
        /// `customer.created`
        CustomerCreated => "customer.created",
        /// `customer.updated`
        CustomerUpdated => "customer.updated",
        /// `customer.deleted`
        CustomerDeleted => "customer.deleted",
        /// `customer.subscription.created`
        CustomerSubscriptionCreated => "customer.subscription.created",
        /// `customer.subscription.updated`
        CustomerSubscriptionUpdated => "customer.subscription.updated",
        /// `customer.subscription.deleted`
        CustomerSubscriptionDeleted => "customer.subscription.deleted",
        /// `customer.subscription.trial_will_end`
        CustomerSubscriptionTrialWillEnd => "customer.subscription.trial_will_end",
        /// `invoice.paid`
        InvoicePaid => "invoice.paid",
        /// `invoice.payment_failed`
        InvoicePaymentFailed => "invoice.payment_failed",
        /// `invoice.finalized`
        InvoiceFinalized => "invoice.finalized",
        /// `invoice.upcoming`
        InvoiceUpcoming => "invoice.upcoming",
        /// `invoiceitem.created`
        InvoiceItemCreated => "invoiceitem.created",
        /// `payout.paid`
        PayoutPaid => "payout.paid",
        /// `payout.failed`
        PayoutFailed => "payout.failed",
        /// `setup_intent.succeeded`
        SetupIntentSucceeded => "setup_intent.succeeded",
        /// `setup_intent.setup_failed`
        SetupIntentSetupFailed => "setup_intent.setup_failed",
        /// `account.updated`
        AccountUpdated => "account.updated",
        /// `account.application.deauthorized`
        AccountApplicationDeauthorized => "account.application.deauthorized",
        /// `radar.early_fraud_warning.created`
        RadarEarlyFraudWarningCreated => "radar.early_fraud_warning.created",
        /// `billing_portal.session.created`
        BillingPortalSessionCreated => "billing_portal.session.created",
        /// `price.created`
        PriceCreated => "price.created",
        /// `product.created`
        ProductCreated => "product.created",
        /// `transfer.created`
        TransferCreated => "transfer.created",
    }
}

/// Start a Stripe webhook endpoint builder.
#[must_use]
pub fn endpoint() -> WebhookBuilder {
    WebhookBuilder::new(Provider::Stripe)
}

/// Build a Stripe webhook from typed events.
pub fn webhook(
    url: impl Into<String>,
    events: impl IntoIterator<Item = Event>,
) -> Result<Webhook> {
    endpoint().url(url).events(events).build()
}
