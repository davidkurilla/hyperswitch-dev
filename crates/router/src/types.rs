// FIXME: Why were these data types grouped this way?
//
// Folder `types` is strange for Rust ecosystem, nevertheless it might be okay.
// But folder `enum` is even more strange I unlikely okay. Why should not we introduce folders `type`, `structs` and `traits`? :)
// Is it better to split data types according to business logic instead.
// For example, customers/address/dispute/mandate is "models".
// Separation of concerns instead of separation of forms.

pub mod api;
pub mod authentication;
pub mod domain;
#[cfg(feature = "frm")]
pub mod fraud_check;
pub mod pm_auth;

pub mod storage;
pub mod transformers;
use std::marker::PhantomData;

pub use api_models::{enums::Connector, mandates};
#[cfg(feature = "payouts")]
pub use api_models::{enums::PayoutConnectors, payouts as payout_types};
pub use common_utils::{pii, pii::Email, request::RequestContent, types::MinorUnit};
#[cfg(feature = "frm")]
pub use hyperswitch_domain_models::router_data_v2::FrmFlowData;
pub use hyperswitch_domain_models::{
    payment_address::PaymentAddress,
    router_data::{
        AccessToken, AdditionalPaymentMethodConnectorResponse, ApplePayCryptogramData,
        ApplePayPredecryptData, ConnectorAuthType, ConnectorResponseData, ErrorResponse,
        PaymentMethodBalance, PaymentMethodToken, RecurringMandatePaymentData, RouterData,
    },
    router_data_v2::{
        AccessTokenFlowData, DisputesFlowData, ExternalAuthenticationFlowData, FilesFlowData,
        MandateRevokeFlowData, PaymentFlowData, RefundFlowData, RouterDataV2,
        WebhookSourceVerifyData,
    },
    router_request_types::{
        AcceptDisputeRequestData, AccessTokenRequestData, AuthorizeSessionTokenData,
        BrowserInformation, ChargeRefunds, ChargeRefundsOptions, CompleteAuthorizeData,
        CompleteAuthorizeRedirectResponse, ConnectorCustomerData, DefendDisputeRequestData,
        DestinationChargeRefund, DirectChargeRefund, MandateRevokeRequestData,
        MultipleCaptureRequestData, PaymentMethodTokenizationData, PaymentsApproveData,
        PaymentsAuthorizeData, PaymentsCancelData, PaymentsCaptureData,
        PaymentsIncrementalAuthorizationData, PaymentsPreProcessingData, PaymentsRejectData,
        PaymentsSessionData, PaymentsSyncData, RefundsData, ResponseId, RetrieveFileRequestData,
        SetupMandateRequestData, SubmitEvidenceRequestData, SyncRequestType, UploadFileRequestData,
        VerifyWebhookSourceRequestData,
    },
    router_response_types::{
        AcceptDisputeResponse, CaptureSyncResponse, DefendDisputeResponse, MandateReference,
        MandateRevokeResponseData, PaymentsResponseData, PreprocessingResponseId,
        RefundsResponseData, RetrieveFileResponse, SubmitEvidenceResponse, UploadFileResponse,
        VerifyWebhookSourceResponseData, VerifyWebhookStatus,
    },
};
#[cfg(feature = "payouts")]
pub use hyperswitch_domain_models::{
    router_data_v2::PayoutFlowData, router_request_types::PayoutsData,
    router_response_types::PayoutsResponseData,
};
pub use hyperswitch_interfaces::types::Response;

pub use crate::core::payments::CustomerDetails;
#[cfg(feature = "payouts")]
use crate::{
    connector::utils::missing_field_err,
    core::utils::IRRELEVANT_CONNECTOR_REQUEST_REFERENCE_ID_IN_PAYOUTS_FLOW,
};
use crate::{
    consts,
    core::{
        errors::{self},
        payments::PaymentData,
    },
    services,
    types::transformers::{ForeignFrom, ForeignTryFrom},
};
pub type PaymentsAuthorizeRouterData =
    RouterData<api::Authorize, PaymentsAuthorizeData, PaymentsResponseData>;
pub type PaymentsPreProcessingRouterData =
    RouterData<api::PreProcessing, PaymentsPreProcessingData, PaymentsResponseData>;
pub type PaymentsAuthorizeSessionTokenRouterData =
    RouterData<api::AuthorizeSessionToken, AuthorizeSessionTokenData, PaymentsResponseData>;
pub type PaymentsCompleteAuthorizeRouterData =
    RouterData<api::CompleteAuthorize, CompleteAuthorizeData, PaymentsResponseData>;
pub type PaymentsInitRouterData =
    RouterData<api::InitPayment, PaymentsAuthorizeData, PaymentsResponseData>;
pub type PaymentsBalanceRouterData =
    RouterData<api::Balance, PaymentsAuthorizeData, PaymentsResponseData>;
pub type PaymentsSyncRouterData = RouterData<api::PSync, PaymentsSyncData, PaymentsResponseData>;
pub type PaymentsCaptureRouterData =
    RouterData<api::Capture, PaymentsCaptureData, PaymentsResponseData>;
pub type PaymentsIncrementalAuthorizationRouterData = RouterData<
    api::IncrementalAuthorization,
    PaymentsIncrementalAuthorizationData,
    PaymentsResponseData,
>;
pub type PaymentsCancelRouterData = RouterData<api::Void, PaymentsCancelData, PaymentsResponseData>;
pub type PaymentsRejectRouterData =
    RouterData<api::Reject, PaymentsRejectData, PaymentsResponseData>;
pub type PaymentsApproveRouterData =
    RouterData<api::Approve, PaymentsApproveData, PaymentsResponseData>;
pub type PaymentsSessionRouterData =
    RouterData<api::Session, PaymentsSessionData, PaymentsResponseData>;
pub type RefundsRouterData<F> = RouterData<F, RefundsData, RefundsResponseData>;
pub type RefundExecuteRouterData = RouterData<api::Execute, RefundsData, RefundsResponseData>;
pub type RefundSyncRouterData = RouterData<api::RSync, RefundsData, RefundsResponseData>;
pub type TokenizationRouterData =
    RouterData<api::PaymentMethodToken, PaymentMethodTokenizationData, PaymentsResponseData>;
pub type ConnectorCustomerRouterData =
    RouterData<api::CreateConnectorCustomer, ConnectorCustomerData, PaymentsResponseData>;

pub type RefreshTokenRouterData =
    RouterData<api::AccessTokenAuth, AccessTokenRequestData, AccessToken>;

pub type PaymentsResponseRouterData<R> =
    ResponseRouterData<api::Authorize, R, PaymentsAuthorizeData, PaymentsResponseData>;
pub type PaymentsCancelResponseRouterData<R> =
    ResponseRouterData<api::Void, R, PaymentsCancelData, PaymentsResponseData>;
pub type PaymentsBalanceResponseRouterData<R> =
    ResponseRouterData<api::Balance, R, PaymentsAuthorizeData, PaymentsResponseData>;
pub type PaymentsSyncResponseRouterData<R> =
    ResponseRouterData<api::PSync, R, PaymentsSyncData, PaymentsResponseData>;
pub type PaymentsSessionResponseRouterData<R> =
    ResponseRouterData<api::Session, R, PaymentsSessionData, PaymentsResponseData>;
pub type PaymentsInitResponseRouterData<R> =
    ResponseRouterData<api::InitPayment, R, PaymentsAuthorizeData, PaymentsResponseData>;
pub type PaymentsCaptureResponseRouterData<R> =
    ResponseRouterData<api::Capture, R, PaymentsCaptureData, PaymentsResponseData>;
pub type PaymentsPreprocessingResponseRouterData<R> =
    ResponseRouterData<api::PreProcessing, R, PaymentsPreProcessingData, PaymentsResponseData>;
pub type TokenizationResponseRouterData<R> = ResponseRouterData<
    api::PaymentMethodToken,
    R,
    PaymentMethodTokenizationData,
    PaymentsResponseData,
>;
pub type ConnectorCustomerResponseRouterData<R> = ResponseRouterData<
    api::CreateConnectorCustomer,
    R,
    ConnectorCustomerData,
    PaymentsResponseData,
>;

pub type RefundsResponseRouterData<F, R> =
    ResponseRouterData<F, R, RefundsData, RefundsResponseData>;

pub type PaymentsAuthorizeType =
    dyn services::ConnectorIntegration<api::Authorize, PaymentsAuthorizeData, PaymentsResponseData>;
pub type SetupMandateType = dyn services::ConnectorIntegration<
    api::SetupMandate,
    SetupMandateRequestData,
    PaymentsResponseData,
>;
pub type MandateRevokeType = dyn services::ConnectorIntegration<
    api::MandateRevoke,
    MandateRevokeRequestData,
    MandateRevokeResponseData,
>;
pub type PaymentsPreProcessingType = dyn services::ConnectorIntegration<
    api::PreProcessing,
    PaymentsPreProcessingData,
    PaymentsResponseData,
>;
pub type PaymentsCompleteAuthorizeType = dyn services::ConnectorIntegration<
    api::CompleteAuthorize,
    CompleteAuthorizeData,
    PaymentsResponseData,
>;
pub type PaymentsPreAuthorizeType = dyn services::ConnectorIntegration<
    api::AuthorizeSessionToken,
    AuthorizeSessionTokenData,
    PaymentsResponseData,
>;
pub type PaymentsInitType = dyn services::ConnectorIntegration<
    api::InitPayment,
    PaymentsAuthorizeData,
    PaymentsResponseData,
>;
pub type PaymentsBalanceType =
    dyn services::ConnectorIntegration<api::Balance, PaymentsAuthorizeData, PaymentsResponseData>;
pub type PaymentsSyncType =
    dyn services::ConnectorIntegration<api::PSync, PaymentsSyncData, PaymentsResponseData>;
pub type PaymentsCaptureType =
    dyn services::ConnectorIntegration<api::Capture, PaymentsCaptureData, PaymentsResponseData>;
pub type PaymentsSessionType =
    dyn services::ConnectorIntegration<api::Session, PaymentsSessionData, PaymentsResponseData>;
pub type PaymentsVoidType =
    dyn services::ConnectorIntegration<api::Void, PaymentsCancelData, PaymentsResponseData>;
pub type TokenizationType = dyn services::ConnectorIntegration<
    api::PaymentMethodToken,
    PaymentMethodTokenizationData,
    PaymentsResponseData,
>;
pub type IncrementalAuthorizationType = dyn services::ConnectorIntegration<
    api::IncrementalAuthorization,
    PaymentsIncrementalAuthorizationData,
    PaymentsResponseData,
>;

pub type ConnectorCustomerType = dyn services::ConnectorIntegration<
    api::CreateConnectorCustomer,
    ConnectorCustomerData,
    PaymentsResponseData,
>;

pub type RefundExecuteType =
    dyn services::ConnectorIntegration<api::Execute, RefundsData, RefundsResponseData>;
pub type RefundSyncType =
    dyn services::ConnectorIntegration<api::RSync, RefundsData, RefundsResponseData>;

#[cfg(feature = "payouts")]
pub type PayoutCancelType =
    dyn services::ConnectorIntegration<api::PoCancel, PayoutsData, PayoutsResponseData>;
#[cfg(feature = "payouts")]
pub type PayoutCreateType =
    dyn services::ConnectorIntegration<api::PoCreate, PayoutsData, PayoutsResponseData>;
#[cfg(feature = "payouts")]
pub type PayoutEligibilityType =
    dyn services::ConnectorIntegration<api::PoEligibility, PayoutsData, PayoutsResponseData>;
#[cfg(feature = "payouts")]
pub type PayoutFulfillType =
    dyn services::ConnectorIntegration<api::PoFulfill, PayoutsData, PayoutsResponseData>;
#[cfg(feature = "payouts")]
pub type PayoutRecipientType =
    dyn services::ConnectorIntegration<api::PoRecipient, PayoutsData, PayoutsResponseData>;
#[cfg(feature = "payouts")]
pub type PayoutRecipientAccountType =
    dyn services::ConnectorIntegration<api::PoRecipientAccount, PayoutsData, PayoutsResponseData>;
#[cfg(feature = "payouts")]
pub type PayoutQuoteType =
    dyn services::ConnectorIntegration<api::PoQuote, PayoutsData, PayoutsResponseData>;

pub type RefreshTokenType =
    dyn services::ConnectorIntegration<api::AccessTokenAuth, AccessTokenRequestData, AccessToken>;

pub type AcceptDisputeType = dyn services::ConnectorIntegration<
    api::Accept,
    AcceptDisputeRequestData,
    AcceptDisputeResponse,
>;
pub type VerifyWebhookSourceType = dyn services::ConnectorIntegration<
    api::VerifyWebhookSource,
    VerifyWebhookSourceRequestData,
    VerifyWebhookSourceResponseData,
>;

pub type SubmitEvidenceType = dyn services::ConnectorIntegration<
    api::Evidence,
    SubmitEvidenceRequestData,
    SubmitEvidenceResponse,
>;

pub type UploadFileType =
    dyn services::ConnectorIntegration<api::Upload, UploadFileRequestData, UploadFileResponse>;

pub type RetrieveFileType = dyn services::ConnectorIntegration<
    api::Retrieve,
    RetrieveFileRequestData,
    RetrieveFileResponse,
>;

pub type DefendDisputeType = dyn services::ConnectorIntegration<
    api::Defend,
    DefendDisputeRequestData,
    DefendDisputeResponse,
>;

pub type SetupMandateRouterData =
    RouterData<api::SetupMandate, SetupMandateRequestData, PaymentsResponseData>;

pub type AcceptDisputeRouterData =
    RouterData<api::Accept, AcceptDisputeRequestData, AcceptDisputeResponse>;

pub type VerifyWebhookSourceRouterData = RouterData<
    api::VerifyWebhookSource,
    VerifyWebhookSourceRequestData,
    VerifyWebhookSourceResponseData,
>;

pub type SubmitEvidenceRouterData =
    RouterData<api::Evidence, SubmitEvidenceRequestData, SubmitEvidenceResponse>;

pub type UploadFileRouterData = RouterData<api::Upload, UploadFileRequestData, UploadFileResponse>;

pub type RetrieveFileRouterData =
    RouterData<api::Retrieve, RetrieveFileRequestData, RetrieveFileResponse>;

pub type DefendDisputeRouterData =
    RouterData<api::Defend, DefendDisputeRequestData, DefendDisputeResponse>;

pub type MandateRevokeRouterData =
    RouterData<api::MandateRevoke, MandateRevokeRequestData, MandateRevokeResponseData>;

#[cfg(feature = "payouts")]
pub type PayoutsRouterData<F> = RouterData<F, PayoutsData, PayoutsResponseData>;

#[cfg(feature = "payouts")]
pub type PayoutsResponseRouterData<F, R> =
    ResponseRouterData<F, R, PayoutsData, PayoutsResponseData>;

#[cfg(feature = "payouts")]
pub trait PayoutIndividualDetailsExt {
    type Error;
    fn get_external_account_account_holder_type(&self) -> Result<String, Self::Error>;
}

#[cfg(feature = "payouts")]
impl PayoutIndividualDetailsExt for api_models::payouts::PayoutIndividualDetails {
    type Error = error_stack::Report<errors::ConnectorError>;
    fn get_external_account_account_holder_type(&self) -> Result<String, Self::Error> {
        self.external_account_account_holder_type
            .clone()
            .ok_or_else(missing_field_err("external_account_account_holder_type"))
    }
}

pub trait Capturable {
    fn get_captured_amount<F>(&self, _payment_data: &PaymentData<F>) -> Option<i64>
    where
        F: Clone,
    {
        None
    }
    fn get_amount_capturable<F>(
        &self,
        _payment_data: &PaymentData<F>,
        _attempt_status: common_enums::AttemptStatus,
    ) -> Option<i64>
    where
        F: Clone,
    {
        None
    }
}

impl Capturable for PaymentsAuthorizeData {
    fn get_captured_amount<F>(&self, _payment_data: &PaymentData<F>) -> Option<i64>
    where
        F: Clone,
    {
        let final_amount = self
            .surcharge_details
            .as_ref()
            .map(|surcharge_details| surcharge_details.final_amount.get_amount_as_i64());
        final_amount.or(Some(self.amount))
    }

    fn get_amount_capturable<F>(
        &self,
        payment_data: &PaymentData<F>,
        attempt_status: common_enums::AttemptStatus,
    ) -> Option<i64>
    where
        F: Clone,
    {
        match payment_data
            .payment_attempt
            .capture_method
            .unwrap_or_default()
        {
            common_enums::CaptureMethod::Automatic => {
                let intent_status = common_enums::IntentStatus::foreign_from(attempt_status);
                match intent_status {
                    common_enums::IntentStatus::Succeeded
                    | common_enums::IntentStatus::Failed
                    | common_enums::IntentStatus::Processing => Some(0),
                    common_enums::IntentStatus::Cancelled
                    | common_enums::IntentStatus::PartiallyCaptured
                    | common_enums::IntentStatus::RequiresCustomerAction
                    | common_enums::IntentStatus::RequiresMerchantAction
                    | common_enums::IntentStatus::RequiresPaymentMethod
                    | common_enums::IntentStatus::RequiresConfirmation
                    | common_enums::IntentStatus::RequiresCapture
                    | common_enums::IntentStatus::PartiallyCapturedAndCapturable => None,
                }
            },
            common_enums::CaptureMethod::Manual => Some(payment_data.payment_attempt.get_total_amount().get_amount_as_i64()),
            // In case of manual multiple, amount capturable must be inferred from all captures.
            common_enums::CaptureMethod::ManualMultiple |
            // Scheduled capture is not supported as of now
            common_enums::CaptureMethod::Scheduled => None,
        }
    }
}

impl Capturable for PaymentsCaptureData {
    fn get_captured_amount<F>(&self, _payment_data: &PaymentData<F>) -> Option<i64>
    where
        F: Clone,
    {
        Some(self.amount_to_capture)
    }
    fn get_amount_capturable<F>(
        &self,
        _payment_data: &PaymentData<F>,
        attempt_status: common_enums::AttemptStatus,
    ) -> Option<i64>
    where
        F: Clone,
    {
        let intent_status = common_enums::IntentStatus::foreign_from(attempt_status);
        match intent_status {
            common_enums::IntentStatus::Succeeded
            | common_enums::IntentStatus::PartiallyCaptured => Some(0),
            common_enums::IntentStatus::Processing
            | common_enums::IntentStatus::Cancelled
            | common_enums::IntentStatus::Failed
            | common_enums::IntentStatus::RequiresCustomerAction
            | common_enums::IntentStatus::RequiresMerchantAction
            | common_enums::IntentStatus::RequiresPaymentMethod
            | common_enums::IntentStatus::RequiresConfirmation
            | common_enums::IntentStatus::RequiresCapture
            | common_enums::IntentStatus::PartiallyCapturedAndCapturable => None,
        }
    }
}

impl Capturable for CompleteAuthorizeData {
    fn get_captured_amount<F>(&self, _payment_data: &PaymentData<F>) -> Option<i64>
    where
        F: Clone,
    {
        Some(self.amount)
    }
    fn get_amount_capturable<F>(
        &self,
        payment_data: &PaymentData<F>,
        attempt_status: common_enums::AttemptStatus,
    ) -> Option<i64>
    where
        F: Clone,
    {
        match payment_data
            .payment_attempt
            .capture_method
            .unwrap_or_default()
        {
            common_enums::CaptureMethod::Automatic => {
                let intent_status = common_enums::IntentStatus::foreign_from(attempt_status);
                match intent_status {
                    common_enums::IntentStatus::Succeeded|
                    common_enums::IntentStatus::Failed|
                    common_enums::IntentStatus::Processing => Some(0),
                    common_enums::IntentStatus::Cancelled
                    | common_enums::IntentStatus::PartiallyCaptured
                    | common_enums::IntentStatus::RequiresCustomerAction
                    | common_enums::IntentStatus::RequiresMerchantAction
                    | common_enums::IntentStatus::RequiresPaymentMethod
                    | common_enums::IntentStatus::RequiresConfirmation
                    | common_enums::IntentStatus::RequiresCapture
                    | common_enums::IntentStatus::PartiallyCapturedAndCapturable => None,
                }
            },
            common_enums::CaptureMethod::Manual => Some(payment_data.payment_attempt.get_total_amount().get_amount_as_i64()),
            // In case of manual multiple, amount capturable must be inferred from all captures.
            common_enums::CaptureMethod::ManualMultiple |
            // Scheduled capture is not supported as of now
            common_enums::CaptureMethod::Scheduled => None,
        }
    }
}
impl Capturable for SetupMandateRequestData {}
impl Capturable for PaymentsCancelData {
    fn get_captured_amount<F>(&self, payment_data: &PaymentData<F>) -> Option<i64>
    where
        F: Clone,
    {
        // return previously captured amount
        payment_data
            .payment_intent
            .amount_captured
            .map(|amt| amt.get_amount_as_i64())
    }
    fn get_amount_capturable<F>(
        &self,
        _payment_data: &PaymentData<F>,
        attempt_status: common_enums::AttemptStatus,
    ) -> Option<i64>
    where
        F: Clone,
    {
        let intent_status = common_enums::IntentStatus::foreign_from(attempt_status);
        match intent_status {
            common_enums::IntentStatus::Cancelled
            | common_enums::IntentStatus::Processing
            | common_enums::IntentStatus::PartiallyCaptured => Some(0),
            common_enums::IntentStatus::Succeeded
            | common_enums::IntentStatus::Failed
            | common_enums::IntentStatus::RequiresCustomerAction
            | common_enums::IntentStatus::RequiresMerchantAction
            | common_enums::IntentStatus::RequiresPaymentMethod
            | common_enums::IntentStatus::RequiresConfirmation
            | common_enums::IntentStatus::RequiresCapture
            | common_enums::IntentStatus::PartiallyCapturedAndCapturable => None,
        }
    }
}
impl Capturable for PaymentsApproveData {}
impl Capturable for PaymentsRejectData {}
impl Capturable for PaymentsSessionData {}
impl Capturable for PaymentsIncrementalAuthorizationData {
    fn get_amount_capturable<F>(
        &self,
        _payment_data: &PaymentData<F>,
        _attempt_status: common_enums::AttemptStatus,
    ) -> Option<i64>
    where
        F: Clone,
    {
        Some(self.total_amount)
    }
}
impl Capturable for PaymentsSyncData {
    fn get_captured_amount<F>(&self, payment_data: &PaymentData<F>) -> Option<i64>
    where
        F: Clone,
    {
        payment_data
            .payment_attempt
            .amount_to_capture
            .or_else(|| Some(payment_data.payment_attempt.get_total_amount()))
            .map(|amt| amt.get_amount_as_i64())
    }
    fn get_amount_capturable<F>(
        &self,
        _payment_data: &PaymentData<F>,
        attempt_status: common_enums::AttemptStatus,
    ) -> Option<i64>
    where
        F: Clone,
    {
        if attempt_status.is_terminal_status() {
            Some(0)
        } else {
            None
        }
    }
}

pub struct AddAccessTokenResult {
    pub access_token_result: Result<Option<AccessToken>, ErrorResponse>,
    pub connector_supports_access_token: bool,
}

pub struct PaymentMethodTokenResult {
    pub payment_method_token_result: Result<Option<String>, ErrorResponse>,
    pub is_payment_method_tokenization_performed: bool,
}

#[derive(Debug, Clone, Copy)]
pub enum Redirection {
    Redirect,
    NoRedirect,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct PollConfig {
    pub delay_in_secs: i8,
    pub frequency: i8,
}

impl PollConfig {
    pub fn get_poll_config_key(connector: String) -> String {
        format!("poll_config_external_three_ds_{connector}")
    }
}

impl Default for PollConfig {
    fn default() -> Self {
        Self {
            delay_in_secs: consts::DEFAULT_POLL_DELAY_IN_SECS,
            frequency: consts::DEFAULT_POLL_FREQUENCY,
        }
    }
}

#[derive(Clone, Debug)]
pub struct RedirectPaymentFlowResponse {
    pub payments_response: api_models::payments::PaymentsResponse,
    pub business_profile: diesel_models::business_profile::BusinessProfile,
}

#[derive(Clone, Debug)]
pub struct AuthenticatePaymentFlowResponse {
    pub payments_response: api_models::payments::PaymentsResponse,
    pub poll_config: PollConfig,
    pub business_profile: diesel_models::business_profile::BusinessProfile,
}

#[derive(Debug, Clone, Default, serde::Deserialize, serde::Serialize)]
pub struct ConnectorResponse {
    pub merchant_id: String,
    pub connector: String,
    pub payment_id: String,
    pub amount: i64,
    pub connector_transaction_id: String,
    pub return_url: Option<String>,
    pub three_ds_form: Option<services::RedirectForm>,
}

pub struct ResponseRouterData<Flow, R, Request, Response> {
    pub response: R,
    pub data: RouterData<Flow, Request, Response>,
    pub http_code: u16,
}

impl ForeignFrom<api_models::admin::ConnectorAuthType> for ConnectorAuthType {
    fn foreign_from(value: api_models::admin::ConnectorAuthType) -> Self {
        match value {
            api_models::admin::ConnectorAuthType::TemporaryAuth => Self::TemporaryAuth,
            api_models::admin::ConnectorAuthType::HeaderKey { api_key } => {
                Self::HeaderKey { api_key }
            }
            api_models::admin::ConnectorAuthType::BodyKey { api_key, key1 } => {
                Self::BodyKey { api_key, key1 }
            }
            api_models::admin::ConnectorAuthType::SignatureKey {
                api_key,
                key1,
                api_secret,
            } => Self::SignatureKey {
                api_key,
                key1,
                api_secret,
            },
            api_models::admin::ConnectorAuthType::MultiAuthKey {
                api_key,
                key1,
                api_secret,
                key2,
            } => Self::MultiAuthKey {
                api_key,
                key1,
                api_secret,
                key2,
            },
            api_models::admin::ConnectorAuthType::CurrencyAuthKey { auth_key_map } => {
                Self::CurrencyAuthKey { auth_key_map }
            }
            api_models::admin::ConnectorAuthType::NoKey => Self::NoKey,
            api_models::admin::ConnectorAuthType::CertificateAuth {
                certificate,
                private_key,
            } => Self::CertificateAuth {
                certificate,
                private_key,
            },
        }
    }
}

impl ForeignFrom<ConnectorAuthType> for api_models::admin::ConnectorAuthType {
    fn foreign_from(from: ConnectorAuthType) -> Self {
        match from {
            ConnectorAuthType::TemporaryAuth => Self::TemporaryAuth,
            ConnectorAuthType::HeaderKey { api_key } => Self::HeaderKey { api_key },
            ConnectorAuthType::BodyKey { api_key, key1 } => Self::BodyKey { api_key, key1 },
            ConnectorAuthType::SignatureKey {
                api_key,
                key1,
                api_secret,
            } => Self::SignatureKey {
                api_key,
                key1,
                api_secret,
            },
            ConnectorAuthType::MultiAuthKey {
                api_key,
                key1,
                api_secret,
                key2,
            } => Self::MultiAuthKey {
                api_key,
                key1,
                api_secret,
                key2,
            },
            ConnectorAuthType::CurrencyAuthKey { auth_key_map } => {
                Self::CurrencyAuthKey { auth_key_map }
            }
            ConnectorAuthType::NoKey => Self::NoKey,
            ConnectorAuthType::CertificateAuth {
                certificate,
                private_key,
            } => Self::CertificateAuth {
                certificate,
                private_key,
            },
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConnectorsList {
    pub connectors: Vec<String>,
}

impl ForeignTryFrom<ConnectorAuthType> for AccessTokenRequestData {
    type Error = errors::ApiErrorResponse;
    fn foreign_try_from(connector_auth: ConnectorAuthType) -> Result<Self, Self::Error> {
        match connector_auth {
            ConnectorAuthType::HeaderKey { api_key } => Ok(Self {
                app_id: api_key,
                id: None,
            }),
            ConnectorAuthType::BodyKey { api_key, key1 } => Ok(Self {
                app_id: api_key,
                id: Some(key1),
            }),
            ConnectorAuthType::SignatureKey { api_key, key1, .. } => Ok(Self {
                app_id: api_key,
                id: Some(key1),
            }),
            ConnectorAuthType::MultiAuthKey { api_key, key1, .. } => Ok(Self {
                app_id: api_key,
                id: Some(key1),
            }),

            _ => Err(errors::ApiErrorResponse::InvalidDataValue {
                field_name: "connector_account_details",
            }),
        }
    }
}

impl ForeignFrom<&PaymentsAuthorizeRouterData> for AuthorizeSessionTokenData {
    fn foreign_from(data: &PaymentsAuthorizeRouterData) -> Self {
        Self {
            amount_to_capture: data.amount_captured,
            currency: data.request.currency,
            connector_transaction_id: data.payment_id.clone(),
            amount: Some(data.request.amount),
        }
    }
}

pub trait Tokenizable {
    fn set_session_token(&mut self, token: Option<String>);
}

impl Tokenizable for SetupMandateRequestData {
    fn set_session_token(&mut self, _token: Option<String>) {}
}

impl Tokenizable for PaymentsAuthorizeData {
    fn set_session_token(&mut self, token: Option<String>) {
        self.session_token = token;
    }
}

impl Tokenizable for CompleteAuthorizeData {
    fn set_session_token(&mut self, _token: Option<String>) {}
}

impl ForeignFrom<&SetupMandateRouterData> for PaymentsAuthorizeData {
    fn foreign_from(data: &SetupMandateRouterData) -> Self {
        Self {
            currency: data.request.currency,
            payment_method_data: data.request.payment_method_data.clone(),
            confirm: data.request.confirm,
            statement_descriptor_suffix: data.request.statement_descriptor_suffix.clone(),
            mandate_id: data.request.mandate_id.clone(),
            setup_future_usage: data.request.setup_future_usage,
            off_session: data.request.off_session,
            setup_mandate_details: data.request.setup_mandate_details.clone(),
            router_return_url: data.request.router_return_url.clone(),
            email: data.request.email.clone(),
            customer_name: data.request.customer_name.clone(),
            amount: 0,
            minor_amount: MinorUnit::new(0),
            statement_descriptor: None,
            capture_method: None,
            webhook_url: None,
            complete_authorize_url: None,
            browser_info: data.request.browser_info.clone(),
            order_details: None,
            order_category: None,
            session_token: None,
            enrolled_for_3ds: true,
            related_transaction_id: None,
            payment_experience: None,
            payment_method_type: None,
            customer_id: None,
            surcharge_details: None,
            request_incremental_authorization: data.request.request_incremental_authorization,
            metadata: None,
            authentication_data: None,
            customer_acceptance: data.request.customer_acceptance.clone(),
            charges: None, // TODO: allow charges on mandates?
            merchant_order_reference_id: None,
            integrity_object: None,
        }
    }
}

impl<F1, F2, T1, T2> ForeignFrom<(&RouterData<F1, T1, PaymentsResponseData>, T2)>
    for RouterData<F2, T2, PaymentsResponseData>
{
    fn foreign_from(item: (&RouterData<F1, T1, PaymentsResponseData>, T2)) -> Self {
        let data = item.0;
        let request = item.1;
        Self {
            flow: PhantomData,
            request,
            merchant_id: data.merchant_id.clone(),
            connector: data.connector.clone(),
            attempt_id: data.attempt_id.clone(),
            status: data.status,
            payment_method: data.payment_method,
            connector_auth_type: data.connector_auth_type.clone(),
            description: data.description.clone(),
            return_url: data.return_url.clone(),
            address: data.address.clone(),
            auth_type: data.auth_type,
            connector_meta_data: data.connector_meta_data.clone(),
            connector_wallets_details: data.connector_wallets_details.clone(),
            amount_captured: data.amount_captured,
            minor_amount_captured: data.minor_amount_captured,
            access_token: data.access_token.clone(),
            response: data.response.clone(),
            payment_id: data.payment_id.clone(),
            session_token: data.session_token.clone(),
            reference_id: data.reference_id.clone(),
            customer_id: data.customer_id.clone(),
            payment_method_token: None,
            preprocessing_id: None,
            connector_customer: data.connector_customer.clone(),
            recurring_mandate_payment_data: data.recurring_mandate_payment_data.clone(),
            connector_request_reference_id: data.connector_request_reference_id.clone(),
            #[cfg(feature = "payouts")]
            payout_method_data: data.payout_method_data.clone(),
            #[cfg(feature = "payouts")]
            quote_id: data.quote_id.clone(),
            test_mode: data.test_mode,
            payment_method_status: None,
            payment_method_balance: data.payment_method_balance.clone(),
            connector_api_version: data.connector_api_version.clone(),
            connector_http_status_code: data.connector_http_status_code,
            external_latency: data.external_latency,
            apple_pay_flow: data.apple_pay_flow.clone(),
            frm_metadata: data.frm_metadata.clone(),
            dispute_id: data.dispute_id.clone(),
            refund_id: data.refund_id.clone(),
            connector_response: data.connector_response.clone(),
            integrity_check: Ok(()),
        }
    }
}

#[cfg(feature = "payouts")]
impl<F1, F2>
    ForeignFrom<(
        &RouterData<F1, PayoutsData, PayoutsResponseData>,
        PayoutsData,
    )> for RouterData<F2, PayoutsData, PayoutsResponseData>
{
    fn foreign_from(
        item: (
            &RouterData<F1, PayoutsData, PayoutsResponseData>,
            PayoutsData,
        ),
    ) -> Self {
        let data = item.0;
        let request = item.1;
        Self {
            flow: PhantomData,
            request,
            merchant_id: data.merchant_id.clone(),
            connector: data.connector.clone(),
            attempt_id: data.attempt_id.clone(),
            status: data.status,
            payment_method: data.payment_method,
            connector_auth_type: data.connector_auth_type.clone(),
            description: data.description.clone(),
            return_url: data.return_url.clone(),
            address: data.address.clone(),
            auth_type: data.auth_type,
            connector_meta_data: data.connector_meta_data.clone(),
            connector_wallets_details: data.connector_wallets_details.clone(),
            amount_captured: data.amount_captured,
            minor_amount_captured: data.minor_amount_captured,
            access_token: data.access_token.clone(),
            response: data.response.clone(),
            payment_id: data.payment_id.clone(),
            session_token: data.session_token.clone(),
            reference_id: data.reference_id.clone(),
            customer_id: data.customer_id.clone(),
            payment_method_token: None,
            recurring_mandate_payment_data: None,
            preprocessing_id: None,
            connector_customer: data.connector_customer.clone(),
            connector_request_reference_id:
                IRRELEVANT_CONNECTOR_REQUEST_REFERENCE_ID_IN_PAYOUTS_FLOW.to_string(),
            payout_method_data: data.payout_method_data.clone(),
            quote_id: data.quote_id.clone(),
            test_mode: data.test_mode,
            payment_method_balance: None,
            payment_method_status: None,
            connector_api_version: None,
            connector_http_status_code: data.connector_http_status_code,
            external_latency: data.external_latency,
            apple_pay_flow: None,
            frm_metadata: None,
            refund_id: None,
            dispute_id: None,
            connector_response: data.connector_response.clone(),
            integrity_check: Ok(()),
        }
    }
}
