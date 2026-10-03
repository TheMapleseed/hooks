//! PayPal REST webhooks.

use crate::common::{Webhook, WebhookBuilder};
use crate::error::Result;
use crate::provider::Provider;

provider_events! {
    /// Common PayPal webhook event types.
    pub enum Event {
        /// `PAYMENT.CAPTURE.COMPLETED`
        PaymentCaptureCompleted => "PAYMENT.CAPTURE.COMPLETED",
        /// `PAYMENT.CAPTURE.DENIED`
        PaymentCaptureDenied => "PAYMENT.CAPTURE.DENIED",
        /// `PAYMENT.CAPTURE.REFUNDED`
        PaymentCaptureRefunded => "PAYMENT.CAPTURE.REFUNDED",
        /// `CHECKOUT.ORDER.APPROVED`
        CheckoutOrderApproved => "CHECKOUT.ORDER.APPROVED",
        /// `CHECKOUT.ORDER.COMPLETED`
        CheckoutOrderCompleted => "CHECKOUT.ORDER.COMPLETED",
        /// `BILLING.SUBSCRIPTION.CREATED`
        BillingSubscriptionCreated => "BILLING.SUBSCRIPTION.CREATED",
        /// `BILLING.SUBSCRIPTION.ACTIVATED`
        BillingSubscriptionActivated => "BILLING.SUBSCRIPTION.ACTIVATED",
        /// `BILLING.SUBSCRIPTION.CANCELLED`
        BillingSubscriptionCancelled => "BILLING.SUBSCRIPTION.CANCELLED",
        /// `BILLING.SUBSCRIPTION.SUSPENDED`
        BillingSubscriptionSuspended => "BILLING.SUBSCRIPTION.SUSPENDED",
        /// `BILLING.SUBSCRIPTION.PAYMENT.FAILED`
        BillingSubscriptionPaymentFailed => "BILLING.SUBSCRIPTION.PAYMENT.FAILED",
        /// `CUSTOMER.DISPUTE.CREATED`
        CustomerDisputeCreated => "CUSTOMER.DISPUTE.CREATED",
        /// `MERCHANT.ONBOARDING.COMPLETED`
        MerchantOnboardingCompleted => "MERCHANT.ONBOARDING.COMPLETED",
    }
}

/// Start a PayPal webhook endpoint builder.
#[must_use]
pub fn endpoint() -> WebhookBuilder {
    WebhookBuilder::new(Provider::PayPal)
}

/// Build a PayPal webhook from typed events.
pub fn webhook(
    url: impl Into<String>,
    events: impl IntoIterator<Item = Event>,
) -> Result<Webhook> {
    endpoint().url(url).events(events).build()
}
