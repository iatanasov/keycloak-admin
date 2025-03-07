#[allow(unused_imports)]
use progenitor_client::{encode_path, RequestBuilderExt};
#[allow(unused_imports)]
pub use progenitor_client::{ByteStream, Error, ResponseValue};
#[allow(unused_imports)]
use reqwest::header::{HeaderMap, HeaderValue};
use reqwest::StatusCode;
/// Types used as operation parameters and responses.
#[allow(clippy::all)]
pub mod types {
    use serde::{Deserialize, Serialize};
    #[allow(unused_imports)]
    use std::convert::TryFrom;
    /// Error types.
    pub mod error {
        /// Error from a TryFrom or FromStr implementation.
        pub struct ConversionError(std::borrow::Cow<'static, str>);
        impl std::error::Error for ConversionError {}
        impl std::fmt::Display for ConversionError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                std::fmt::Display::fmt(&self.0, f)
            }
        }

        impl std::fmt::Debug for ConversionError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                std::fmt::Debug::fmt(&self.0, f)
            }
        }

        impl From<&'static str> for ConversionError {
            fn from(value: &'static str) -> Self {
                Self(value.into())
            }
        }

        impl From<String> for ConversionError {
            fn from(value: String) -> Self {
                Self(value.into())
            }
        }
    }

    ///AbstractPolicyRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "decisionStrategy": {
    ///      "$ref": "#/components/schemas/DecisionStrategy"
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "logic": {
    ///      "$ref": "#/components/schemas/Logic"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "owner": {
    ///      "type": "string"
    ///    },
    ///    "policies": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      },
    ///      "uniqueItems": true
    ///    },
    ///    "resources": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      },
    ///      "uniqueItems": true
    ///    },
    ///    "resourcesData": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ResourceRepresentation"
    ///      },
    ///      "uniqueItems": true
    ///    },
    ///    "scopes": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      },
    ///      "uniqueItems": true
    ///    },
    ///    "scopesData": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ScopeRepresentation"
    ///      },
    ///      "uniqueItems": true
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AbstractPolicyRepresentation {
        #[serde(
            rename = "decisionStrategy",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub decision_strategy: Option<DecisionStrategy>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub logic: Option<Logic>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub owner: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub policies: Option<Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resources: Option<Vec<String>>,
        #[serde(
            rename = "resourcesData",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub resources_data: Option<Vec<ResourceRepresentation>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub scopes: Option<Vec<String>>,
        #[serde(
            rename = "scopesData",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub scopes_data: Option<Vec<ScopeRepresentation>>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    impl From<&AbstractPolicyRepresentation> for AbstractPolicyRepresentation {
        fn from(value: &AbstractPolicyRepresentation) -> Self {
            value.clone()
        }
    }

    ///Access
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "roles": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      },
    ///      "uniqueItems": true
    ///    },
    ///    "verify_caller": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Access {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub roles: Option<Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub verify_caller: Option<bool>,
    }

    impl From<&Access> for Access {
        fn from(value: &Access) -> Self {
            value.clone()
        }
    }

    ///AccessToken
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "acr": {
    ///      "type": "string"
    ///    },
    ///    "address": {
    ///      "$ref": "#/components/schemas/AddressClaimSet"
    ///    },
    ///    "allowed-origins": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      },
    ///      "uniqueItems": true
    ///    },
    ///    "at_hash": {
    ///      "type": "string"
    ///    },
    ///    "auth_time": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "authorization": {
    ///      "$ref": "#/components/schemas/Authorization"
    ///    },
    ///    "azp": {
    ///      "type": "string"
    ///    },
    ///    "birthdate": {
    ///      "type": "string"
    ///    },
    ///    "c_hash": {
    ///      "type": "string"
    ///    },
    ///    "claims_locales": {
    ///      "type": "string"
    ///    },
    ///    "cnf": {
    ///      "$ref": "#/components/schemas/Confirmation"
    ///    },
    ///    "email": {
    ///      "type": "string"
    ///    },
    ///    "email_verified": {
    ///      "type": "boolean"
    ///    },
    ///    "exp": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "family_name": {
    ///      "type": "string"
    ///    },
    ///    "gender": {
    ///      "type": "string"
    ///    },
    ///    "given_name": {
    ///      "type": "string"
    ///    },
    ///    "iat": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "iss": {
    ///      "type": "string"
    ///    },
    ///    "jti": {
    ///      "type": "string"
    ///    },
    ///    "locale": {
    ///      "type": "string"
    ///    },
    ///    "middle_name": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "nbf": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "nickname": {
    ///      "type": "string"
    ///    },
    ///    "nonce": {
    ///      "type": "string"
    ///    },
    ///    "otherClaims": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "phone_number": {
    ///      "type": "string"
    ///    },
    ///    "phone_number_verified": {
    ///      "type": "boolean"
    ///    },
    ///    "picture": {
    ///      "type": "string"
    ///    },
    ///    "preferred_username": {
    ///      "type": "string"
    ///    },
    ///    "profile": {
    ///      "type": "string"
    ///    },
    ///    "realm_access": {
    ///      "$ref": "#/components/schemas/Access"
    ///    },
    ///    "resource_access": {
    ///      "writeOnly": true,
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "$ref": "#/components/schemas/Access"
    ///      }
    ///    },
    ///    "s_hash": {
    ///      "type": "string"
    ///    },
    ///    "scope": {
    ///      "type": "string"
    ///    },
    ///    "session_state": {
    ///      "type": "string"
    ///    },
    ///    "sid": {
    ///      "type": "string"
    ///    },
    ///    "sub": {
    ///      "type": "string"
    ///    },
    ///    "trusted-certs": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      },
    ///      "uniqueItems": true
    ///    },
    ///    "typ": {
    ///      "type": "string"
    ///    },
    ///    "updated_at": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "website": {
    ///      "type": "string"
    ///    },
    ///    "zoneinfo": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AccessToken {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub acr: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub address: Option<AddressClaimSet>,
        #[serde(
            rename = "allowed-origins",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub allowed_origins: Option<Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub at_hash: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub auth_time: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub authorization: Option<Authorization>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub azp: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub birthdate: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub c_hash: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub claims_locales: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub cnf: Option<Confirmation>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub email_verified: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub exp: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub family_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub gender: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub given_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub iat: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub iss: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub jti: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub locale: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub middle_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub nbf: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub nickname: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub nonce: Option<String>,
        #[serde(
            rename = "otherClaims",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub other_claims: serde_json::Map<String, serde_json::Value>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub phone_number: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub phone_number_verified: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub picture: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub preferred_username: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub profile: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub realm_access: Option<Access>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub resource_access: std::collections::HashMap<String, Access>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub s_hash: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub scope: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub session_state: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sid: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sub: Option<String>,
        #[serde(
            rename = "trusted-certs",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub trusted_certs: Option<Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub typ: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub website: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub zoneinfo: Option<String>,
    }

    impl From<&AccessToken> for AccessToken {
        fn from(value: &AccessToken) -> Self {
            value.clone()
        }
    }

    ///AddressClaimSet
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "country": {
    ///      "type": "string"
    ///    },
    ///    "formatted": {
    ///      "type": "string"
    ///    },
    ///    "locality": {
    ///      "type": "string"
    ///    },
    ///    "postal_code": {
    ///      "type": "string"
    ///    },
    ///    "region": {
    ///      "type": "string"
    ///    },
    ///    "street_address": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AddressClaimSet {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub country: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub formatted: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub locality: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub postal_code: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub region: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub street_address: Option<String>,
    }

    impl From<&AddressClaimSet> for AddressClaimSet {
        fn from(value: &AddressClaimSet) -> Self {
            value.clone()
        }
    }

    ///AdminEventRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "authDetails": {
    ///      "$ref": "#/components/schemas/AuthDetailsRepresentation"
    ///    },
    ///    "error": {
    ///      "type": "string"
    ///    },
    ///    "operationType": {
    ///      "type": "string"
    ///    },
    ///    "realmId": {
    ///      "type": "string"
    ///    },
    ///    "representation": {
    ///      "type": "string"
    ///    },
    ///    "resourcePath": {
    ///      "type": "string"
    ///    },
    ///    "resourceType": {
    ///      "type": "string"
    ///    },
    ///    "time": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AdminEventRepresentation {
        #[serde(
            rename = "authDetails",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub auth_details: Option<AuthDetailsRepresentation>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub error: Option<String>,
        #[serde(
            rename = "operationType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub operation_type: Option<String>,
        #[serde(rename = "realmId", default, skip_serializing_if = "Option::is_none")]
        pub realm_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub representation: Option<String>,
        #[serde(
            rename = "resourcePath",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub resource_path: Option<String>,
        #[serde(
            rename = "resourceType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub resource_type: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub time: Option<i64>,
    }

    impl From<&AdminEventRepresentation> for AdminEventRepresentation {
        fn from(value: &AdminEventRepresentation) -> Self {
            value.clone()
        }
    }

    ///ApplicationRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "deprecated": true,
    ///  "type": "object",
    ///  "properties": {
    ///    "access": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "boolean"
    ///      }
    ///    },
    ///    "adminUrl": {
    ///      "type": "string"
    ///    },
    ///    "alwaysDisplayInConsole": {
    ///      "type": "boolean"
    ///    },
    ///    "attributes": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "authenticationFlowBindingOverrides": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "authorizationServicesEnabled": {
    ///      "type": "boolean"
    ///    },
    ///    "authorizationSettings": {
    ///      "$ref": "#/components/schemas/ResourceServerRepresentation"
    ///    },
    ///    "baseUrl": {
    ///      "type": "string"
    ///    },
    ///    "bearerOnly": {
    ///      "type": "boolean"
    ///    },
    ///    "claims": {
    ///      "deprecated": true,
    ///      "type": "object",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/ClaimRepresentation"
    ///        }
    ///      ]
    ///    },
    ///    "clientAuthenticatorType": {
    ///      "type": "string"
    ///    },
    ///    "clientId": {
    ///      "type": "string"
    ///    },
    ///    "clientTemplate": {
    ///      "deprecated": true,
    ///      "type": "string"
    ///    },
    ///    "consentRequired": {
    ///      "type": "boolean"
    ///    },
    ///    "defaultClientScopes": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "defaultRoles": {
    ///      "deprecated": true,
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "directAccessGrantsEnabled": {
    ///      "type": "boolean"
    ///    },
    ///    "directGrantsOnly": {
    ///      "deprecated": true,
    ///      "type": "boolean"
    ///    },
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "frontchannelLogout": {
    ///      "type": "boolean"
    ///    },
    ///    "fullScopeAllowed": {
    ///      "type": "boolean"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "implicitFlowEnabled": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "nodeReRegistrationTimeout": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "notBefore": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "optionalClientScopes": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "origin": {
    ///      "type": "string"
    ///    },
    ///    "protocol": {
    ///      "type": "string"
    ///    },
    ///    "protocolMappers": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ProtocolMapperRepresentation"
    ///      }
    ///    },
    ///    "publicClient": {
    ///      "type": "boolean"
    ///    },
    ///    "redirectUris": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "registeredNodes": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "integer",
    ///        "format": "int32"
    ///      }
    ///    },
    ///    "registrationAccessToken": {
    ///      "type": "string"
    ///    },
    ///    "rootUrl": {
    ///      "type": "string"
    ///    },
    ///    "secret": {
    ///      "type": "string"
    ///    },
    ///    "serviceAccountsEnabled": {
    ///      "type": "boolean"
    ///    },
    ///    "standardFlowEnabled": {
    ///      "type": "boolean"
    ///    },
    ///    "surrogateAuthRequired": {
    ///      "type": "boolean"
    ///    },
    ///    "useTemplateConfig": {
    ///      "deprecated": true,
    ///      "type": "boolean"
    ///    },
    ///    "useTemplateMappers": {
    ///      "deprecated": true,
    ///      "type": "boolean"
    ///    },
    ///    "useTemplateScope": {
    ///      "deprecated": true,
    ///      "type": "boolean"
    ///    },
    ///    "webOrigins": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ApplicationRepresentation {
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub access: std::collections::HashMap<String, bool>,
        #[serde(rename = "adminUrl", default, skip_serializing_if = "Option::is_none")]
        pub admin_url: Option<String>,
        #[serde(
            rename = "alwaysDisplayInConsole",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub always_display_in_console: Option<bool>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub attributes: std::collections::HashMap<String, String>,
        #[serde(
            rename = "authenticationFlowBindingOverrides",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub authentication_flow_binding_overrides: std::collections::HashMap<String, String>,
        #[serde(
            rename = "authorizationServicesEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub authorization_services_enabled: Option<bool>,
        #[serde(
            rename = "authorizationSettings",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub authorization_settings: Option<ResourceServerRepresentation>,
        #[serde(rename = "baseUrl", default, skip_serializing_if = "Option::is_none")]
        pub base_url: Option<String>,
        #[serde(
            rename = "bearerOnly",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub bearer_only: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub claims: Option<ClaimRepresentation>,
        #[serde(
            rename = "clientAuthenticatorType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub client_authenticator_type: Option<String>,
        #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
        pub client_id: Option<String>,
        #[serde(
            rename = "clientTemplate",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub client_template: Option<String>,
        #[serde(
            rename = "consentRequired",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub consent_required: Option<bool>,
        #[serde(
            rename = "defaultClientScopes",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub default_client_scopes: Vec<String>,
        #[serde(
            rename = "defaultRoles",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub default_roles: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(
            rename = "directAccessGrantsEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub direct_access_grants_enabled: Option<bool>,
        #[serde(
            rename = "directGrantsOnly",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub direct_grants_only: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[serde(
            rename = "frontchannelLogout",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub frontchannel_logout: Option<bool>,
        #[serde(
            rename = "fullScopeAllowed",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub full_scope_allowed: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(
            rename = "implicitFlowEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub implicit_flow_enabled: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(
            rename = "nodeReRegistrationTimeout",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub node_re_registration_timeout: Option<i32>,
        #[serde(rename = "notBefore", default, skip_serializing_if = "Option::is_none")]
        pub not_before: Option<i32>,
        #[serde(
            rename = "optionalClientScopes",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub optional_client_scopes: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub origin: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub protocol: Option<String>,
        #[serde(
            rename = "protocolMappers",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub protocol_mappers: Vec<ProtocolMapperRepresentation>,
        #[serde(
            rename = "publicClient",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub public_client: Option<bool>,
        #[serde(
            rename = "redirectUris",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub redirect_uris: Vec<String>,
        #[serde(
            rename = "registeredNodes",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub registered_nodes: std::collections::HashMap<String, i32>,
        #[serde(
            rename = "registrationAccessToken",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub registration_access_token: Option<String>,
        #[serde(rename = "rootUrl", default, skip_serializing_if = "Option::is_none")]
        pub root_url: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub secret: Option<String>,
        #[serde(
            rename = "serviceAccountsEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub service_accounts_enabled: Option<bool>,
        #[serde(
            rename = "standardFlowEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub standard_flow_enabled: Option<bool>,
        #[serde(
            rename = "surrogateAuthRequired",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub surrogate_auth_required: Option<bool>,
        #[serde(
            rename = "useTemplateConfig",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub use_template_config: Option<bool>,
        #[serde(
            rename = "useTemplateMappers",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub use_template_mappers: Option<bool>,
        #[serde(
            rename = "useTemplateScope",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub use_template_scope: Option<bool>,
        #[serde(rename = "webOrigins", default, skip_serializing_if = "Vec::is_empty")]
        pub web_origins: Vec<String>,
    }

    impl From<&ApplicationRepresentation> for ApplicationRepresentation {
        fn from(value: &ApplicationRepresentation) -> Self {
            value.clone()
        }
    }

    ///AuthDetailsRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "clientId": {
    ///      "type": "string"
    ///    },
    ///    "ipAddress": {
    ///      "type": "string"
    ///    },
    ///    "realmId": {
    ///      "type": "string"
    ///    },
    ///    "userId": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AuthDetailsRepresentation {
        #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
        pub client_id: Option<String>,
        #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
        pub ip_address: Option<String>,
        #[serde(rename = "realmId", default, skip_serializing_if = "Option::is_none")]
        pub realm_id: Option<String>,
        #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
        pub user_id: Option<String>,
    }

    impl From<&AuthDetailsRepresentation> for AuthDetailsRepresentation {
        fn from(value: &AuthDetailsRepresentation) -> Self {
            value.clone()
        }
    }

    ///AuthenticationExecutionExportRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "authenticator": {
    ///      "type": "string"
    ///    },
    ///    "authenticatorConfig": {
    ///      "type": "string"
    ///    },
    ///    "authenticatorFlow": {
    ///      "type": "boolean"
    ///    },
    ///    "autheticatorFlow": {
    ///      "deprecated": true,
    ///      "type": "boolean"
    ///    },
    ///    "flowAlias": {
    ///      "type": "string"
    ///    },
    ///    "priority": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "requirement": {
    ///      "type": "string"
    ///    },
    ///    "userSetupAllowed": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AuthenticationExecutionExportRepresentation {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub authenticator: Option<String>,
        #[serde(
            rename = "authenticatorConfig",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub authenticator_config: Option<String>,
        #[serde(
            rename = "authenticatorFlow",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub authenticator_flow: Option<bool>,
        #[serde(
            rename = "autheticatorFlow",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub autheticator_flow: Option<bool>,
        #[serde(rename = "flowAlias", default, skip_serializing_if = "Option::is_none")]
        pub flow_alias: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub priority: Option<i32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub requirement: Option<String>,
        #[serde(
            rename = "userSetupAllowed",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub user_setup_allowed: Option<bool>,
    }

    impl From<&AuthenticationExecutionExportRepresentation>
        for AuthenticationExecutionExportRepresentation
    {
        fn from(value: &AuthenticationExecutionExportRepresentation) -> Self {
            value.clone()
        }
    }

    ///AuthenticationExecutionInfoRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "alias": {
    ///      "type": "string"
    ///    },
    ///    "authenticationConfig": {
    ///      "type": "string"
    ///    },
    ///    "authenticationFlow": {
    ///      "type": "boolean"
    ///    },
    ///    "configurable": {
    ///      "type": "boolean"
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "displayName": {
    ///      "type": "string"
    ///    },
    ///    "flowId": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "index": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "level": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "providerId": {
    ///      "type": "string"
    ///    },
    ///    "requirement": {
    ///      "type": "string"
    ///    },
    ///    "requirementChoices": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AuthenticationExecutionInfoRepresentation {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub alias: Option<String>,
        #[serde(
            rename = "authenticationConfig",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub authentication_config: Option<String>,
        #[serde(
            rename = "authenticationFlow",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub authentication_flow: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub configurable: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub display_name: Option<String>,
        #[serde(rename = "flowId", default, skip_serializing_if = "Option::is_none")]
        pub flow_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub index: Option<i32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub level: Option<i32>,
        #[serde(
            rename = "providerId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub provider_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub requirement: Option<String>,
        #[serde(
            rename = "requirementChoices",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub requirement_choices: Vec<String>,
    }

    impl From<&AuthenticationExecutionInfoRepresentation>
        for AuthenticationExecutionInfoRepresentation
    {
        fn from(value: &AuthenticationExecutionInfoRepresentation) -> Self {
            value.clone()
        }
    }

    ///AuthenticationExecutionRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "authenticator": {
    ///      "type": "string"
    ///    },
    ///    "authenticatorConfig": {
    ///      "type": "string"
    ///    },
    ///    "authenticatorFlow": {
    ///      "type": "boolean"
    ///    },
    ///    "autheticatorFlow": {
    ///      "deprecated": true,
    ///      "type": "boolean"
    ///    },
    ///    "flowId": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "parentFlow": {
    ///      "type": "string"
    ///    },
    ///    "priority": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "requirement": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AuthenticationExecutionRepresentation {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub authenticator: Option<String>,
        #[serde(
            rename = "authenticatorConfig",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub authenticator_config: Option<String>,
        #[serde(
            rename = "authenticatorFlow",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub authenticator_flow: Option<bool>,
        #[serde(
            rename = "autheticatorFlow",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub autheticator_flow: Option<bool>,
        #[serde(rename = "flowId", default, skip_serializing_if = "Option::is_none")]
        pub flow_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(
            rename = "parentFlow",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub parent_flow: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub priority: Option<i32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub requirement: Option<String>,
    }

    impl From<&AuthenticationExecutionRepresentation> for AuthenticationExecutionRepresentation {
        fn from(value: &AuthenticationExecutionRepresentation) -> Self {
            value.clone()
        }
    }

    ///AuthenticationFlowRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "alias": {
    ///      "type": "string"
    ///    },
    ///    "authenticationExecutions": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref":
    /// "#/components/schemas/AuthenticationExecutionExportRepresentation"
    ///      }
    ///    },
    ///    "builtIn": {
    ///      "type": "boolean"
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "providerId": {
    ///      "type": "string"
    ///    },
    ///    "topLevel": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AuthenticationFlowRepresentation {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub alias: Option<String>,
        #[serde(
            rename = "authenticationExecutions",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub authentication_executions: Vec<AuthenticationExecutionExportRepresentation>,
        #[serde(rename = "builtIn", default, skip_serializing_if = "Option::is_none")]
        pub built_in: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(
            rename = "providerId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub provider_id: Option<String>,
        #[serde(rename = "topLevel", default, skip_serializing_if = "Option::is_none")]
        pub top_level: Option<bool>,
    }

    impl From<&AuthenticationFlowRepresentation> for AuthenticationFlowRepresentation {
        fn from(value: &AuthenticationFlowRepresentation) -> Self {
            value.clone()
        }
    }

    ///AuthenticatorConfigInfoRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "helpText": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "properties": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ConfigPropertyRepresentation"
    ///      }
    ///    },
    ///    "providerId": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AuthenticatorConfigInfoRepresentation {
        #[serde(rename = "helpText", default, skip_serializing_if = "Option::is_none")]
        pub help_text: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub properties: Vec<ConfigPropertyRepresentation>,
        #[serde(
            rename = "providerId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub provider_id: Option<String>,
    }

    impl From<&AuthenticatorConfigInfoRepresentation> for AuthenticatorConfigInfoRepresentation {
        fn from(value: &AuthenticatorConfigInfoRepresentation) -> Self {
            value.clone()
        }
    }

    ///AuthenticatorConfigRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "alias": {
    ///      "type": "string"
    ///    },
    ///    "config": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct AuthenticatorConfigRepresentation {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub alias: Option<String>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub config: std::collections::HashMap<String, String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
    }

    impl From<&AuthenticatorConfigRepresentation> for AuthenticatorConfigRepresentation {
        fn from(value: &AuthenticatorConfigRepresentation) -> Self {
            value.clone()
        }
    }

    ///Authorization
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "permissions": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/Permission"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Authorization {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub permissions: Vec<Permission>,
    }

    impl From<&Authorization> for Authorization {
        fn from(value: &Authorization) -> Self {
            value.clone()
        }
    }

    ///CertificateRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "certificate": {
    ///      "type": "string"
    ///    },
    ///    "kid": {
    ///      "type": "string"
    ///    },
    ///    "privateKey": {
    ///      "type": "string"
    ///    },
    ///    "publicKey": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CertificateRepresentation {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub certificate: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub kid: Option<String>,
        #[serde(
            rename = "privateKey",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub private_key: Option<String>,
        #[serde(rename = "publicKey", default, skip_serializing_if = "Option::is_none")]
        pub public_key: Option<String>,
    }

    impl From<&CertificateRepresentation> for CertificateRepresentation {
        fn from(value: &CertificateRepresentation) -> Self {
            value.clone()
        }
    }

    ///ClaimRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "address": {
    ///      "type": "boolean"
    ///    },
    ///    "email": {
    ///      "type": "boolean"
    ///    },
    ///    "gender": {
    ///      "type": "boolean"
    ///    },
    ///    "locale": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "type": "boolean"
    ///    },
    ///    "phone": {
    ///      "type": "boolean"
    ///    },
    ///    "picture": {
    ///      "type": "boolean"
    ///    },
    ///    "profile": {
    ///      "type": "boolean"
    ///    },
    ///    "username": {
    ///      "type": "boolean"
    ///    },
    ///    "website": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ClaimRepresentation {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub address: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub email: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub gender: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub locale: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub phone: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub picture: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub profile: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub username: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub website: Option<bool>,
    }

    impl From<&ClaimRepresentation> for ClaimRepresentation {
        fn from(value: &ClaimRepresentation) -> Self {
            value.clone()
        }
    }

    ///ClientInitialAccessCreatePresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "count": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "expiration": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ClientInitialAccessCreatePresentation {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub count: Option<i32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub expiration: Option<i32>,
    }

    impl From<&ClientInitialAccessCreatePresentation> for ClientInitialAccessCreatePresentation {
        fn from(value: &ClientInitialAccessCreatePresentation) -> Self {
            value.clone()
        }
    }

    ///ClientInitialAccessPresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "count": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "expiration": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "remainingCount": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "timestamp": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "token": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ClientInitialAccessPresentation {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub count: Option<i32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub expiration: Option<i32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(
            rename = "remainingCount",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub remaining_count: Option<i32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub timestamp: Option<i32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub token: Option<String>,
    }

    impl From<&ClientInitialAccessPresentation> for ClientInitialAccessPresentation {
        fn from(value: &ClientInitialAccessPresentation) -> Self {
            value.clone()
        }
    }

    ///ClientMappingsRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "client": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "mappings": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/RoleRepresentation"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ClientMappingsRepresentation {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub client: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub mappings: Vec<RoleRepresentation>,
    }

    impl From<&ClientMappingsRepresentation> for ClientMappingsRepresentation {
        fn from(value: &ClientMappingsRepresentation) -> Self {
            value.clone()
        }
    }

    ///ClientPoliciesRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "policies": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ClientPolicyRepresentation"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ClientPoliciesRepresentation {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub policies: Vec<ClientPolicyRepresentation>,
    }

    impl From<&ClientPoliciesRepresentation> for ClientPoliciesRepresentation {
        fn from(value: &ClientPoliciesRepresentation) -> Self {
            value.clone()
        }
    }

    ///ClientPolicyConditionRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "condition": {
    ///      "type": "string"
    ///    },
    ///    "configuration": {
    ///      "type": "array"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ClientPolicyConditionRepresentation {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub condition: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub configuration: Vec<serde_json::Value>,
    }

    impl From<&ClientPolicyConditionRepresentation> for ClientPolicyConditionRepresentation {
        fn from(value: &ClientPolicyConditionRepresentation) -> Self {
            value.clone()
        }
    }

    ///ClientPolicyExecutorRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "configuration": {
    ///      "type": "array"
    ///    },
    ///    "executor": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ClientPolicyExecutorRepresentation {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub configuration: Vec<serde_json::Value>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub executor: Option<String>,
    }

    impl From<&ClientPolicyExecutorRepresentation> for ClientPolicyExecutorRepresentation {
        fn from(value: &ClientPolicyExecutorRepresentation) -> Self {
            value.clone()
        }
    }

    ///ClientPolicyRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "conditions": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref":
    /// "#/components/schemas/ClientPolicyConditionRepresentation"
    ///      }
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "profiles": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ClientPolicyRepresentation {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub conditions: Vec<ClientPolicyConditionRepresentation>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub profiles: Vec<String>,
    }

    impl From<&ClientPolicyRepresentation> for ClientPolicyRepresentation {
        fn from(value: &ClientPolicyRepresentation) -> Self {
            value.clone()
        }
    }

    ///ClientProfileRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "executors": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ClientPolicyExecutorRepresentation"
    ///      }
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ClientProfileRepresentation {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub executors: Vec<ClientPolicyExecutorRepresentation>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    impl From<&ClientProfileRepresentation> for ClientProfileRepresentation {
        fn from(value: &ClientProfileRepresentation) -> Self {
            value.clone()
        }
    }

    ///ClientProfilesRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "globalProfiles": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ClientProfileRepresentation"
    ///      }
    ///    },
    ///    "profiles": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ClientProfileRepresentation"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ClientProfilesRepresentation {
        #[serde(
            rename = "globalProfiles",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub global_profiles: Vec<ClientProfileRepresentation>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub profiles: Vec<ClientProfileRepresentation>,
    }

    impl From<&ClientProfilesRepresentation> for ClientProfilesRepresentation {
        fn from(value: &ClientProfilesRepresentation) -> Self {
            value.clone()
        }
    }

    ///ClientRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "access": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "boolean"
    ///      }
    ///    },
    ///    "adminUrl": {
    ///      "type": "string"
    ///    },
    ///    "alwaysDisplayInConsole": {
    ///      "type": "boolean"
    ///    },
    ///    "attributes": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "authenticationFlowBindingOverrides": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "authorizationServicesEnabled": {
    ///      "type": "boolean"
    ///    },
    ///    "authorizationSettings": {
    ///      "$ref": "#/components/schemas/ResourceServerRepresentation"
    ///    },
    ///    "baseUrl": {
    ///      "type": "string"
    ///    },
    ///    "bearerOnly": {
    ///      "type": "boolean"
    ///    },
    ///    "clientAuthenticatorType": {
    ///      "type": "string"
    ///    },
    ///    "clientId": {
    ///      "type": "string"
    ///    },
    ///    "clientTemplate": {
    ///      "deprecated": true,
    ///      "type": "string"
    ///    },
    ///    "consentRequired": {
    ///      "type": "boolean"
    ///    },
    ///    "defaultClientScopes": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "defaultRoles": {
    ///      "deprecated": true,
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "directAccessGrantsEnabled": {
    ///      "type": "boolean"
    ///    },
    ///    "directGrantsOnly": {
    ///      "deprecated": true,
    ///      "type": "boolean"
    ///    },
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "frontchannelLogout": {
    ///      "type": "boolean"
    ///    },
    ///    "fullScopeAllowed": {
    ///      "type": "boolean"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "implicitFlowEnabled": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "nodeReRegistrationTimeout": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "notBefore": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "optionalClientScopes": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "origin": {
    ///      "type": "string"
    ///    },
    ///    "protocol": {
    ///      "type": "string"
    ///    },
    ///    "protocolMappers": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ProtocolMapperRepresentation"
    ///      }
    ///    },
    ///    "publicClient": {
    ///      "type": "boolean"
    ///    },
    ///    "redirectUris": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "registeredNodes": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "integer",
    ///        "format": "int32"
    ///      }
    ///    },
    ///    "registrationAccessToken": {
    ///      "type": "string"
    ///    },
    ///    "rootUrl": {
    ///      "type": "string"
    ///    },
    ///    "secret": {
    ///      "type": "string"
    ///    },
    ///    "serviceAccountsEnabled": {
    ///      "type": "boolean"
    ///    },
    ///    "standardFlowEnabled": {
    ///      "type": "boolean"
    ///    },
    ///    "surrogateAuthRequired": {
    ///      "type": "boolean"
    ///    },
    ///    "useTemplateConfig": {
    ///      "deprecated": true,
    ///      "type": "boolean"
    ///    },
    ///    "useTemplateMappers": {
    ///      "deprecated": true,
    ///      "type": "boolean"
    ///    },
    ///    "useTemplateScope": {
    ///      "deprecated": true,
    ///      "type": "boolean"
    ///    },
    ///    "webOrigins": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ClientRepresentation {
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub access: std::collections::HashMap<String, bool>,
        #[serde(rename = "adminUrl", default, skip_serializing_if = "Option::is_none")]
        pub admin_url: Option<String>,
        #[serde(
            rename = "alwaysDisplayInConsole",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub always_display_in_console: Option<bool>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub attributes: std::collections::HashMap<String, String>,
        #[serde(
            rename = "authenticationFlowBindingOverrides",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub authentication_flow_binding_overrides: std::collections::HashMap<String, String>,
        #[serde(
            rename = "authorizationServicesEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub authorization_services_enabled: Option<bool>,
        #[serde(
            rename = "authorizationSettings",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub authorization_settings: Option<ResourceServerRepresentation>,
        #[serde(rename = "baseUrl", default, skip_serializing_if = "Option::is_none")]
        pub base_url: Option<String>,
        #[serde(
            rename = "bearerOnly",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub bearer_only: Option<bool>,
        #[serde(
            rename = "clientAuthenticatorType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub client_authenticator_type: Option<String>,
        #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
        pub client_id: Option<String>,
        #[serde(
            rename = "clientTemplate",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub client_template: Option<String>,
        #[serde(
            rename = "consentRequired",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub consent_required: Option<bool>,
        #[serde(
            rename = "defaultClientScopes",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub default_client_scopes: Vec<String>,
        #[serde(
            rename = "defaultRoles",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub default_roles: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(
            rename = "directAccessGrantsEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub direct_access_grants_enabled: Option<bool>,
        #[serde(
            rename = "directGrantsOnly",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub direct_grants_only: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[serde(
            rename = "frontchannelLogout",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub frontchannel_logout: Option<bool>,
        #[serde(
            rename = "fullScopeAllowed",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub full_scope_allowed: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(
            rename = "implicitFlowEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub implicit_flow_enabled: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(
            rename = "nodeReRegistrationTimeout",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub node_re_registration_timeout: Option<i32>,
        #[serde(rename = "notBefore", default, skip_serializing_if = "Option::is_none")]
        pub not_before: Option<i32>,
        #[serde(
            rename = "optionalClientScopes",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub optional_client_scopes: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub origin: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub protocol: Option<String>,
        #[serde(
            rename = "protocolMappers",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub protocol_mappers: Vec<ProtocolMapperRepresentation>,
        #[serde(
            rename = "publicClient",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub public_client: Option<bool>,
        #[serde(
            rename = "redirectUris",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub redirect_uris: Vec<String>,
        #[serde(
            rename = "registeredNodes",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub registered_nodes: std::collections::HashMap<String, i32>,
        #[serde(
            rename = "registrationAccessToken",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub registration_access_token: Option<String>,
        #[serde(rename = "rootUrl", default, skip_serializing_if = "Option::is_none")]
        pub root_url: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub secret: Option<String>,
        #[serde(
            rename = "serviceAccountsEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub service_accounts_enabled: Option<bool>,
        #[serde(
            rename = "standardFlowEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub standard_flow_enabled: Option<bool>,
        #[serde(
            rename = "surrogateAuthRequired",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub surrogate_auth_required: Option<bool>,
        #[serde(
            rename = "useTemplateConfig",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub use_template_config: Option<bool>,
        #[serde(
            rename = "useTemplateMappers",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub use_template_mappers: Option<bool>,
        #[serde(
            rename = "useTemplateScope",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub use_template_scope: Option<bool>,
        #[serde(rename = "webOrigins", default, skip_serializing_if = "Vec::is_empty")]
        pub web_origins: Vec<String>,
    }

    impl From<&ClientRepresentation> for ClientRepresentation {
        fn from(value: &ClientRepresentation) -> Self {
            value.clone()
        }
    }

    ///ClientScopeRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "attributes": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "protocol": {
    ///      "type": "string"
    ///    },
    ///    "protocolMappers": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ProtocolMapperRepresentation"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ClientScopeRepresentation {
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub attributes: std::collections::HashMap<String, String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub protocol: Option<String>,
        #[serde(
            rename = "protocolMappers",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub protocol_mappers: Vec<ProtocolMapperRepresentation>,
    }

    impl From<&ClientScopeRepresentation> for ClientScopeRepresentation {
        fn from(value: &ClientScopeRepresentation) -> Self {
            value.clone()
        }
    }

    ///ClientTemplateRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "deprecated": true,
    ///  "type": "object",
    ///  "properties": {
    ///    "attributes": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "bearerOnly": {
    ///      "type": "boolean"
    ///    },
    ///    "consentRequired": {
    ///      "type": "boolean"
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "directAccessGrantsEnabled": {
    ///      "type": "boolean"
    ///    },
    ///    "frontchannelLogout": {
    ///      "type": "boolean"
    ///    },
    ///    "fullScopeAllowed": {
    ///      "type": "boolean"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "implicitFlowEnabled": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "protocol": {
    ///      "type": "string"
    ///    },
    ///    "protocolMappers": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ProtocolMapperRepresentation"
    ///      }
    ///    },
    ///    "publicClient": {
    ///      "type": "boolean"
    ///    },
    ///    "serviceAccountsEnabled": {
    ///      "type": "boolean"
    ///    },
    ///    "standardFlowEnabled": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ClientTemplateRepresentation {
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub attributes: std::collections::HashMap<String, String>,
        #[serde(
            rename = "bearerOnly",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub bearer_only: Option<bool>,
        #[serde(
            rename = "consentRequired",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub consent_required: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(
            rename = "directAccessGrantsEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub direct_access_grants_enabled: Option<bool>,
        #[serde(
            rename = "frontchannelLogout",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub frontchannel_logout: Option<bool>,
        #[serde(
            rename = "fullScopeAllowed",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub full_scope_allowed: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(
            rename = "implicitFlowEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub implicit_flow_enabled: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub protocol: Option<String>,
        #[serde(
            rename = "protocolMappers",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub protocol_mappers: Vec<ProtocolMapperRepresentation>,
        #[serde(
            rename = "publicClient",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub public_client: Option<bool>,
        #[serde(
            rename = "serviceAccountsEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub service_accounts_enabled: Option<bool>,
        #[serde(
            rename = "standardFlowEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub standard_flow_enabled: Option<bool>,
    }

    impl From<&ClientTemplateRepresentation> for ClientTemplateRepresentation {
        fn from(value: &ClientTemplateRepresentation) -> Self {
            value.clone()
        }
    }

    ///ComponentExportRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "config": {
    ///      "$ref": "#/components/schemas/MultivaluedHashMapStringString"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "providerId": {
    ///      "type": "string"
    ///    },
    ///    "subComponents": {
    ///      "$ref":
    /// "#/components/schemas/
    /// MultivaluedHashMapStringComponentExportRepresentation"
    ///    },
    ///    "subType": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ComponentExportRepresentation {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub config: Option<MultivaluedHashMapStringString>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(
            rename = "providerId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub provider_id: Option<String>,
        #[serde(
            rename = "subComponents",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sub_components: Option<MultivaluedHashMapStringComponentExportRepresentation>,
        #[serde(rename = "subType", default, skip_serializing_if = "Option::is_none")]
        pub sub_type: Option<String>,
    }

    impl From<&ComponentExportRepresentation> for ComponentExportRepresentation {
        fn from(value: &ComponentExportRepresentation) -> Self {
            value.clone()
        }
    }

    ///ComponentRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "config": {
    ///      "$ref": "#/components/schemas/MultivaluedHashMapStringString"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "parentId": {
    ///      "type": "string"
    ///    },
    ///    "providerId": {
    ///      "type": "string"
    ///    },
    ///    "providerType": {
    ///      "type": "string"
    ///    },
    ///    "subType": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ComponentRepresentation {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub config: Option<MultivaluedHashMapStringString>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "parentId", default, skip_serializing_if = "Option::is_none")]
        pub parent_id: Option<String>,
        #[serde(
            rename = "providerId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub provider_id: Option<String>,
        #[serde(
            rename = "providerType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub provider_type: Option<String>,
        #[serde(rename = "subType", default, skip_serializing_if = "Option::is_none")]
        pub sub_type: Option<String>,
    }

    impl From<&ComponentRepresentation> for ComponentRepresentation {
        fn from(value: &ComponentRepresentation) -> Self {
            value.clone()
        }
    }

    ///ComponentTypeRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "helpText": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "metadata": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "properties": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ConfigPropertyRepresentation"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ComponentTypeRepresentation {
        #[serde(rename = "helpText", default, skip_serializing_if = "Option::is_none")]
        pub help_text: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
        pub metadata: serde_json::Map<String, serde_json::Value>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub properties: Vec<ConfigPropertyRepresentation>,
    }

    impl From<&ComponentTypeRepresentation> for ComponentTypeRepresentation {
        fn from(value: &ComponentTypeRepresentation) -> Self {
            value.clone()
        }
    }

    ///Composites
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "application": {
    ///      "deprecated": true,
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "array",
    ///        "items": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "client": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "array",
    ///        "items": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "realm": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      },
    ///      "uniqueItems": true
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Composites {
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub application: std::collections::HashMap<String, Vec<String>>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub client: std::collections::HashMap<String, Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub realm: Option<Vec<String>>,
    }

    impl From<&Composites> for Composites {
        fn from(value: &Composites) -> Self {
            value.clone()
        }
    }

    ///ConfigPropertyRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "defaultValue": {},
    ///    "helpText": {
    ///      "type": "string"
    ///    },
    ///    "label": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "options": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "readOnly": {
    ///      "type": "boolean"
    ///    },
    ///    "required": {
    ///      "type": "boolean"
    ///    },
    ///    "secret": {
    ///      "type": "boolean"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ConfigPropertyRepresentation {
        #[serde(
            rename = "defaultValue",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub default_value: Option<serde_json::Value>,
        #[serde(rename = "helpText", default, skip_serializing_if = "Option::is_none")]
        pub help_text: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub label: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub options: Vec<String>,
        #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
        pub read_only: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub required: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub secret: Option<bool>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    impl From<&ConfigPropertyRepresentation> for ConfigPropertyRepresentation {
        fn from(value: &ConfigPropertyRepresentation) -> Self {
            value.clone()
        }
    }

    ///Confirmation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "jkt": {
    ///      "type": "string"
    ///    },
    ///    "x5t#S256": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Confirmation {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub jkt: Option<String>,
        #[serde(rename = "x5t#S256", default, skip_serializing_if = "Option::is_none")]
        pub x5t_s256: Option<String>,
    }

    impl From<&Confirmation> for Confirmation {
        fn from(value: &Confirmation) -> Self {
            value.clone()
        }
    }

    ///CredentialRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "algorithm": {
    ///      "deprecated": true,
    ///      "type": "string"
    ///    },
    ///    "config": {
    ///      "deprecated": true,
    ///      "type": "object",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/MultivaluedHashMapStringString"
    ///        }
    ///      ]
    ///    },
    ///    "counter": {
    ///      "deprecated": true,
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "createdDate": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "credentialData": {
    ///      "type": "string"
    ///    },
    ///    "device": {
    ///      "deprecated": true,
    ///      "type": "string"
    ///    },
    ///    "digits": {
    ///      "deprecated": true,
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "hashIterations": {
    ///      "deprecated": true,
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "hashedSaltedValue": {
    ///      "deprecated": true,
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "period": {
    ///      "deprecated": true,
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "priority": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "salt": {
    ///      "deprecated": true,
    ///      "type": "string"
    ///    },
    ///    "secretData": {
    ///      "type": "string"
    ///    },
    ///    "temporary": {
    ///      "type": "boolean"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    },
    ///    "userLabel": {
    ///      "type": "string"
    ///    },
    ///    "value": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct CredentialRepresentation {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub algorithm: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub config: Option<MultivaluedHashMapStringString>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub counter: Option<i32>,
        #[serde(
            rename = "createdDate",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub created_date: Option<i64>,
        #[serde(
            rename = "credentialData",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub credential_data: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub device: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub digits: Option<i32>,
        #[serde(
            rename = "hashIterations",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub hash_iterations: Option<i32>,
        #[serde(
            rename = "hashedSaltedValue",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub hashed_salted_value: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub period: Option<i32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub priority: Option<i32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub salt: Option<String>,
        #[serde(
            rename = "secretData",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub secret_data: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub temporary: Option<bool>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        #[serde(rename = "userLabel", default, skip_serializing_if = "Option::is_none")]
        pub user_label: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub value: Option<String>,
    }

    impl From<&CredentialRepresentation> for CredentialRepresentation {
        fn from(value: &CredentialRepresentation) -> Self {
            value.clone()
        }
    }

    ///DecisionEffect
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "PERMIT",
    ///    "DENY"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum DecisionEffect {
        #[serde(rename = "PERMIT")]
        Permit,
        #[serde(rename = "DENY")]
        Deny,
    }

    impl From<&DecisionEffect> for DecisionEffect {
        fn from(value: &DecisionEffect) -> Self {
            value.clone()
        }
    }

    impl ToString for DecisionEffect {
        fn to_string(&self) -> String {
            match *self {
                Self::Permit => "PERMIT".to_string(),
                Self::Deny => "DENY".to_string(),
            }
        }
    }

    impl std::str::FromStr for DecisionEffect {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "PERMIT" => Ok(Self::Permit),
                "DENY" => Ok(Self::Deny),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for DecisionEffect {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for DecisionEffect {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for DecisionEffect {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///DecisionStrategy
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "AFFIRMATIVE",
    ///    "UNANIMOUS",
    ///    "CONSENSUS"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum DecisionStrategy {
        #[serde(rename = "AFFIRMATIVE")]
        Affirmative,
        #[serde(rename = "UNANIMOUS")]
        Unanimous,
        #[serde(rename = "CONSENSUS")]
        Consensus,
    }

    impl From<&DecisionStrategy> for DecisionStrategy {
        fn from(value: &DecisionStrategy) -> Self {
            value.clone()
        }
    }

    impl ToString for DecisionStrategy {
        fn to_string(&self) -> String {
            match *self {
                Self::Affirmative => "AFFIRMATIVE".to_string(),
                Self::Unanimous => "UNANIMOUS".to_string(),
                Self::Consensus => "CONSENSUS".to_string(),
            }
        }
    }

    impl std::str::FromStr for DecisionStrategy {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "AFFIRMATIVE" => Ok(Self::Affirmative),
                "UNANIMOUS" => Ok(Self::Unanimous),
                "CONSENSUS" => Ok(Self::Consensus),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for DecisionStrategy {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for DecisionStrategy {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for DecisionStrategy {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///EnforcementMode
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "PERMISSIVE",
    ///    "ENFORCING",
    ///    "DISABLED"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum EnforcementMode {
        #[serde(rename = "PERMISSIVE")]
        Permissive,
        #[serde(rename = "ENFORCING")]
        Enforcing,
        #[serde(rename = "DISABLED")]
        Disabled,
    }

    impl From<&EnforcementMode> for EnforcementMode {
        fn from(value: &EnforcementMode) -> Self {
            value.clone()
        }
    }

    impl ToString for EnforcementMode {
        fn to_string(&self) -> String {
            match *self {
                Self::Permissive => "PERMISSIVE".to_string(),
                Self::Enforcing => "ENFORCING".to_string(),
                Self::Disabled => "DISABLED".to_string(),
            }
        }
    }

    impl std::str::FromStr for EnforcementMode {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "PERMISSIVE" => Ok(Self::Permissive),
                "ENFORCING" => Ok(Self::Enforcing),
                "DISABLED" => Ok(Self::Disabled),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for EnforcementMode {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for EnforcementMode {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for EnforcementMode {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///EvaluationResultRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "allowedScopes": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ScopeRepresentation"
    ///      }
    ///    },
    ///    "policies": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/PolicyResultRepresentation"
    ///      }
    ///    },
    ///    "resource": {
    ///      "$ref": "#/components/schemas/ResourceRepresentation"
    ///    },
    ///    "scopes": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ScopeRepresentation"
    ///      }
    ///    },
    ///    "status": {
    ///      "$ref": "#/components/schemas/DecisionEffect"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EvaluationResultRepresentation {
        #[serde(
            rename = "allowedScopes",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub allowed_scopes: Vec<ScopeRepresentation>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub policies: Vec<PolicyResultRepresentation>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<ResourceRepresentation>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub scopes: Vec<ScopeRepresentation>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<DecisionEffect>,
    }

    impl From<&EvaluationResultRepresentation> for EvaluationResultRepresentation {
        fn from(value: &EvaluationResultRepresentation) -> Self {
            value.clone()
        }
    }

    ///EventRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "clientId": {
    ///      "type": "string"
    ///    },
    ///    "details": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "error": {
    ///      "type": "string"
    ///    },
    ///    "ipAddress": {
    ///      "type": "string"
    ///    },
    ///    "realmId": {
    ///      "type": "string"
    ///    },
    ///    "sessionId": {
    ///      "type": "string"
    ///    },
    ///    "time": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    },
    ///    "userId": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct EventRepresentation {
        #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
        pub client_id: Option<String>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub details: std::collections::HashMap<String, String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub error: Option<String>,
        #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
        pub ip_address: Option<String>,
        #[serde(rename = "realmId", default, skip_serializing_if = "Option::is_none")]
        pub realm_id: Option<String>,
        #[serde(rename = "sessionId", default, skip_serializing_if = "Option::is_none")]
        pub session_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub time: Option<i64>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
        pub user_id: Option<String>,
    }

    impl From<&EventRepresentation> for EventRepresentation {
        fn from(value: &EventRepresentation) -> Self {
            value.clone()
        }
    }

    ///FederatedIdentityRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "identityProvider": {
    ///      "type": "string"
    ///    },
    ///    "userId": {
    ///      "type": "string"
    ///    },
    ///    "userName": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct FederatedIdentityRepresentation {
        #[serde(
            rename = "identityProvider",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub identity_provider: Option<String>,
        #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
        pub user_id: Option<String>,
        #[serde(rename = "userName", default, skip_serializing_if = "Option::is_none")]
        pub user_name: Option<String>,
    }

    impl From<&FederatedIdentityRepresentation> for FederatedIdentityRepresentation {
        fn from(value: &FederatedIdentityRepresentation) -> Self {
            value.clone()
        }
    }

    ///GlobalRequestResult
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "failedRequests": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "successRequests": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GlobalRequestResult {
        #[serde(
            rename = "failedRequests",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub failed_requests: Vec<String>,
        #[serde(
            rename = "successRequests",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub success_requests: Vec<String>,
    }

    impl From<&GlobalRequestResult> for GlobalRequestResult {
        fn from(value: &GlobalRequestResult) -> Self {
            value.clone()
        }
    }

    ///GroupRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "access": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "boolean"
    ///      }
    ///    },
    ///    "attributes": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "array",
    ///        "items": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "clientRoles": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "array",
    ///        "items": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "parentId": {
    ///      "type": "string"
    ///    },
    ///    "path": {
    ///      "type": "string"
    ///    },
    ///    "realmRoles": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "subGroupCount": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "subGroups": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/GroupRepresentation"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct GroupRepresentation {
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub access: std::collections::HashMap<String, bool>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub attributes: std::collections::HashMap<String, Vec<String>>,
        #[serde(
            rename = "clientRoles",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub client_roles: std::collections::HashMap<String, Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "parentId", default, skip_serializing_if = "Option::is_none")]
        pub parent_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub path: Option<String>,
        #[serde(rename = "realmRoles", default, skip_serializing_if = "Vec::is_empty")]
        pub realm_roles: Vec<String>,
        #[serde(
            rename = "subGroupCount",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sub_group_count: Option<i64>,
        #[serde(rename = "subGroups", default, skip_serializing_if = "Vec::is_empty")]
        pub sub_groups: Vec<GroupRepresentation>,
    }

    impl From<&GroupRepresentation> for GroupRepresentation {
        fn from(value: &GroupRepresentation) -> Self {
            value.clone()
        }
    }

    ///IdToken
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "acr": {
    ///      "type": "string"
    ///    },
    ///    "address": {
    ///      "$ref": "#/components/schemas/AddressClaimSet"
    ///    },
    ///    "at_hash": {
    ///      "type": "string"
    ///    },
    ///    "auth_time": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "azp": {
    ///      "type": "string"
    ///    },
    ///    "birthdate": {
    ///      "type": "string"
    ///    },
    ///    "c_hash": {
    ///      "type": "string"
    ///    },
    ///    "claims_locales": {
    ///      "type": "string"
    ///    },
    ///    "email": {
    ///      "type": "string"
    ///    },
    ///    "email_verified": {
    ///      "type": "boolean"
    ///    },
    ///    "exp": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "family_name": {
    ///      "type": "string"
    ///    },
    ///    "gender": {
    ///      "type": "string"
    ///    },
    ///    "given_name": {
    ///      "type": "string"
    ///    },
    ///    "iat": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "iss": {
    ///      "type": "string"
    ///    },
    ///    "jti": {
    ///      "type": "string"
    ///    },
    ///    "locale": {
    ///      "type": "string"
    ///    },
    ///    "middle_name": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "nbf": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "nickname": {
    ///      "type": "string"
    ///    },
    ///    "nonce": {
    ///      "type": "string"
    ///    },
    ///    "otherClaims": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "phone_number": {
    ///      "type": "string"
    ///    },
    ///    "phone_number_verified": {
    ///      "type": "boolean"
    ///    },
    ///    "picture": {
    ///      "type": "string"
    ///    },
    ///    "preferred_username": {
    ///      "type": "string"
    ///    },
    ///    "profile": {
    ///      "type": "string"
    ///    },
    ///    "s_hash": {
    ///      "type": "string"
    ///    },
    ///    "session_state": {
    ///      "type": "string"
    ///    },
    ///    "sid": {
    ///      "type": "string"
    ///    },
    ///    "sub": {
    ///      "type": "string"
    ///    },
    ///    "typ": {
    ///      "type": "string"
    ///    },
    ///    "updated_at": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "website": {
    ///      "type": "string"
    ///    },
    ///    "zoneinfo": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct IdToken {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub acr: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub address: Option<AddressClaimSet>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub at_hash: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub auth_time: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub azp: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub birthdate: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub c_hash: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub claims_locales: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub email_verified: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub exp: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub family_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub gender: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub given_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub iat: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub iss: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub jti: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub locale: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub middle_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub nbf: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub nickname: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub nonce: Option<String>,
        #[serde(
            rename = "otherClaims",
            default,
            skip_serializing_if = "serde_json::Map::is_empty"
        )]
        pub other_claims: serde_json::Map<String, serde_json::Value>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub phone_number: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub phone_number_verified: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub picture: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub preferred_username: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub profile: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub s_hash: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub session_state: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sid: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub sub: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub typ: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub updated_at: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub website: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub zoneinfo: Option<String>,
    }

    impl From<&IdToken> for IdToken {
        fn from(value: &IdToken) -> Self {
            value.clone()
        }
    }

    ///IdentityProviderMapperRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "config": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "identityProviderAlias": {
    ///      "type": "string"
    ///    },
    ///    "identityProviderMapper": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct IdentityProviderMapperRepresentation {
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub config: std::collections::HashMap<String, String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(
            rename = "identityProviderAlias",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub identity_provider_alias: Option<String>,
        #[serde(
            rename = "identityProviderMapper",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub identity_provider_mapper: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    impl From<&IdentityProviderMapperRepresentation> for IdentityProviderMapperRepresentation {
        fn from(value: &IdentityProviderMapperRepresentation) -> Self {
            value.clone()
        }
    }

    ///IdentityProviderMapperTypeRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "category": {
    ///      "type": "string"
    ///    },
    ///    "helpText": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "properties": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ConfigPropertyRepresentation"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct IdentityProviderMapperTypeRepresentation {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub category: Option<String>,
        #[serde(rename = "helpText", default, skip_serializing_if = "Option::is_none")]
        pub help_text: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub properties: Vec<ConfigPropertyRepresentation>,
    }

    impl From<&IdentityProviderMapperTypeRepresentation> for IdentityProviderMapperTypeRepresentation {
        fn from(value: &IdentityProviderMapperTypeRepresentation) -> Self {
            value.clone()
        }
    }

    ///IdentityProviderRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "addReadTokenRoleOnCreate": {
    ///      "type": "boolean"
    ///    },
    ///    "alias": {
    ///      "type": "string"
    ///    },
    ///    "authenticateByDefault": {
    ///      "type": "boolean"
    ///    },
    ///    "config": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "displayName": {
    ///      "type": "string"
    ///    },
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "firstBrokerLoginFlowAlias": {
    ///      "type": "string"
    ///    },
    ///    "internalId": {
    ///      "type": "string"
    ///    },
    ///    "linkOnly": {
    ///      "type": "boolean"
    ///    },
    ///    "postBrokerLoginFlowAlias": {
    ///      "type": "string"
    ///    },
    ///    "providerId": {
    ///      "type": "string"
    ///    },
    ///    "storeToken": {
    ///      "type": "boolean"
    ///    },
    ///    "trustEmail": {
    ///      "type": "boolean"
    ///    },
    ///    "updateProfileFirstLogin": {
    ///      "deprecated": true,
    ///      "type": "boolean"
    ///    },
    ///    "updateProfileFirstLoginMode": {
    ///      "deprecated": true,
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct IdentityProviderRepresentation {
        #[serde(
            rename = "addReadTokenRoleOnCreate",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub add_read_token_role_on_create: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub alias: Option<String>,
        #[serde(
            rename = "authenticateByDefault",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub authenticate_by_default: Option<bool>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub config: std::collections::HashMap<String, String>,
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub display_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[serde(
            rename = "firstBrokerLoginFlowAlias",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub first_broker_login_flow_alias: Option<String>,
        #[serde(
            rename = "internalId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub internal_id: Option<String>,
        #[serde(rename = "linkOnly", default, skip_serializing_if = "Option::is_none")]
        pub link_only: Option<bool>,
        #[serde(
            rename = "postBrokerLoginFlowAlias",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub post_broker_login_flow_alias: Option<String>,
        #[serde(
            rename = "providerId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub provider_id: Option<String>,
        #[serde(
            rename = "storeToken",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub store_token: Option<bool>,
        #[serde(
            rename = "trustEmail",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub trust_email: Option<bool>,
        #[serde(
            rename = "updateProfileFirstLogin",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub update_profile_first_login: Option<bool>,
        #[serde(
            rename = "updateProfileFirstLoginMode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub update_profile_first_login_mode: Option<String>,
    }

    impl From<&IdentityProviderRepresentation> for IdentityProviderRepresentation {
        fn from(value: &IdentityProviderRepresentation) -> Self {
            value.clone()
        }
    }

    ///InstallationAdapterConfig
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "auth-server-url": {
    ///      "type": "string"
    ///    },
    ///    "bearer-only": {
    ///      "type": "boolean"
    ///    },
    ///    "confidential-port": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "credentials": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "policy-enforcer": {
    ///      "$ref": "#/components/schemas/PolicyEnforcerConfig"
    ///    },
    ///    "public-client": {
    ///      "type": "boolean"
    ///    },
    ///    "realm": {
    ///      "type": "string"
    ///    },
    ///    "realm-public-key": {
    ///      "type": "string"
    ///    },
    ///    "resource": {
    ///      "type": "string"
    ///    },
    ///    "ssl-required": {
    ///      "type": "string"
    ///    },
    ///    "use-resource-role-mappings": {
    ///      "type": "boolean"
    ///    },
    ///    "verify-token-audience": {
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct InstallationAdapterConfig {
        #[serde(
            rename = "auth-server-url",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub auth_server_url: Option<String>,
        #[serde(
            rename = "bearer-only",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub bearer_only: Option<bool>,
        #[serde(
            rename = "confidential-port",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub confidential_port: Option<i32>,
        #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
        pub credentials: serde_json::Map<String, serde_json::Value>,
        #[serde(
            rename = "policy-enforcer",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub policy_enforcer: Option<PolicyEnforcerConfig>,
        #[serde(
            rename = "public-client",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub public_client: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub realm: Option<String>,
        #[serde(
            rename = "realm-public-key",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub realm_public_key: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(
            rename = "ssl-required",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub ssl_required: Option<String>,
        #[serde(
            rename = "use-resource-role-mappings",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub use_resource_role_mappings: Option<bool>,
        #[serde(
            rename = "verify-token-audience",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub verify_token_audience: Option<bool>,
    }

    impl From<&InstallationAdapterConfig> for InstallationAdapterConfig {
        fn from(value: &InstallationAdapterConfig) -> Self {
            value.clone()
        }
    }

    ///KeyMetadataRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "algorithm": {
    ///      "type": "string"
    ///    },
    ///    "certificate": {
    ///      "type": "string"
    ///    },
    ///    "kid": {
    ///      "type": "string"
    ///    },
    ///    "providerId": {
    ///      "type": "string"
    ///    },
    ///    "providerPriority": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "publicKey": {
    ///      "type": "string"
    ///    },
    ///    "status": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    },
    ///    "use": {
    ///      "$ref": "#/components/schemas/KeyUse"
    ///    },
    ///    "validTo": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct KeyMetadataRepresentation {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub algorithm: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub certificate: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub kid: Option<String>,
        #[serde(
            rename = "providerId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub provider_id: Option<String>,
        #[serde(
            rename = "providerPriority",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub provider_priority: Option<i64>,
        #[serde(rename = "publicKey", default, skip_serializing_if = "Option::is_none")]
        pub public_key: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<String>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        #[serde(rename = "use", default, skip_serializing_if = "Option::is_none")]
        pub use_: Option<KeyUse>,
        #[serde(rename = "validTo", default, skip_serializing_if = "Option::is_none")]
        pub valid_to: Option<i64>,
    }

    impl From<&KeyMetadataRepresentation> for KeyMetadataRepresentation {
        fn from(value: &KeyMetadataRepresentation) -> Self {
            value.clone()
        }
    }

    ///KeyStoreConfig
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "format": {
    ///      "type": "string"
    ///    },
    ///    "keyAlias": {
    ///      "type": "string"
    ///    },
    ///    "keyPassword": {
    ///      "type": "string"
    ///    },
    ///    "realmAlias": {
    ///      "type": "string"
    ///    },
    ///    "realmCertificate": {
    ///      "type": "boolean"
    ///    },
    ///    "storePassword": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct KeyStoreConfig {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub format: Option<String>,
        #[serde(rename = "keyAlias", default, skip_serializing_if = "Option::is_none")]
        pub key_alias: Option<String>,
        #[serde(
            rename = "keyPassword",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub key_password: Option<String>,
        #[serde(
            rename = "realmAlias",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub realm_alias: Option<String>,
        #[serde(
            rename = "realmCertificate",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub realm_certificate: Option<bool>,
        #[serde(
            rename = "storePassword",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub store_password: Option<String>,
    }

    impl From<&KeyStoreConfig> for KeyStoreConfig {
        fn from(value: &KeyStoreConfig) -> Self {
            value.clone()
        }
    }

    ///KeyUse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "SIG",
    ///    "ENC"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum KeyUse {
        #[serde(rename = "SIG")]
        Sig,
        #[serde(rename = "ENC")]
        Enc,
    }

    impl From<&KeyUse> for KeyUse {
        fn from(value: &KeyUse) -> Self {
            value.clone()
        }
    }

    impl ToString for KeyUse {
        fn to_string(&self) -> String {
            match *self {
                Self::Sig => "SIG".to_string(),
                Self::Enc => "ENC".to_string(),
            }
        }
    }

    impl std::str::FromStr for KeyUse {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "SIG" => Ok(Self::Sig),
                "ENC" => Ok(Self::Enc),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for KeyUse {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for KeyUse {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for KeyUse {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///KeysMetadataRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "active": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "keys": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/KeyMetadataRepresentation"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct KeysMetadataRepresentation {
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub active: std::collections::HashMap<String, String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub keys: Vec<KeyMetadataRepresentation>,
    }

    impl From<&KeysMetadataRepresentation> for KeysMetadataRepresentation {
        fn from(value: &KeysMetadataRepresentation) -> Self {
            value.clone()
        }
    }

    ///Logic
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "POSITIVE",
    ///    "NEGATIVE"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum Logic {
        #[serde(rename = "POSITIVE")]
        Positive,
        #[serde(rename = "NEGATIVE")]
        Negative,
    }

    impl From<&Logic> for Logic {
        fn from(value: &Logic) -> Self {
            value.clone()
        }
    }

    impl ToString for Logic {
        fn to_string(&self) -> String {
            match *self {
                Self::Positive => "POSITIVE".to_string(),
                Self::Negative => "NEGATIVE".to_string(),
            }
        }
    }

    impl std::str::FromStr for Logic {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "POSITIVE" => Ok(Self::Positive),
                "NEGATIVE" => Ok(Self::Negative),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for Logic {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for Logic {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for Logic {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///ManagementPermissionReference
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "resource": {
    ///      "type": "string"
    ///    },
    ///    "scopePermissions": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ManagementPermissionReference {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(
            rename = "scopePermissions",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub scope_permissions: std::collections::HashMap<String, String>,
    }

    impl From<&ManagementPermissionReference> for ManagementPermissionReference {
        fn from(value: &ManagementPermissionReference) -> Self {
            value.clone()
        }
    }

    ///MappingsRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "clientMappings": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "$ref": "#/components/schemas/ClientMappingsRepresentation"
    ///      }
    ///    },
    ///    "realmMappings": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/RoleRepresentation"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct MappingsRepresentation {
        #[serde(
            rename = "clientMappings",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub client_mappings: std::collections::HashMap<String, ClientMappingsRepresentation>,
        #[serde(
            rename = "realmMappings",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub realm_mappings: Vec<RoleRepresentation>,
    }

    impl From<&MappingsRepresentation> for MappingsRepresentation {
        fn from(value: &MappingsRepresentation) -> Self {
            value.clone()
        }
    }

    ///MethodConfig
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "method": {
    ///      "type": "string"
    ///    },
    ///    "scopes": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "scopes-enforcement-mode": {
    ///      "$ref": "#/components/schemas/ScopeEnforcementMode"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct MethodConfig {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub method: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub scopes: Vec<String>,
        #[serde(
            rename = "scopes-enforcement-mode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub scopes_enforcement_mode: Option<ScopeEnforcementMode>,
    }

    impl From<&MethodConfig> for MethodConfig {
        fn from(value: &MethodConfig) -> Self {
            value.clone()
        }
    }

    ///MultivaluedHashMapStringComponentExportRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "additionalProperties": {
    ///    "type": "array",
    ///    "items": {
    ///      "$ref": "#/components/schemas/ComponentExportRepresentation"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct MultivaluedHashMapStringComponentExportRepresentation(
        pub std::collections::HashMap<String, Vec<ComponentExportRepresentation>>,
    );
    impl std::ops::Deref for MultivaluedHashMapStringComponentExportRepresentation {
        type Target = std::collections::HashMap<String, Vec<ComponentExportRepresentation>>;
        fn deref(&self) -> &std::collections::HashMap<String, Vec<ComponentExportRepresentation>> {
            &self.0
        }
    }

    impl From<MultivaluedHashMapStringComponentExportRepresentation>
        for std::collections::HashMap<String, Vec<ComponentExportRepresentation>>
    {
        fn from(value: MultivaluedHashMapStringComponentExportRepresentation) -> Self {
            value.0
        }
    }

    impl From<&MultivaluedHashMapStringComponentExportRepresentation>
        for MultivaluedHashMapStringComponentExportRepresentation
    {
        fn from(value: &MultivaluedHashMapStringComponentExportRepresentation) -> Self {
            value.clone()
        }
    }

    impl From<std::collections::HashMap<String, Vec<ComponentExportRepresentation>>>
        for MultivaluedHashMapStringComponentExportRepresentation
    {
        fn from(
            value: std::collections::HashMap<String, Vec<ComponentExportRepresentation>>,
        ) -> Self {
            Self(value)
        }
    }

    ///MultivaluedHashMapStringString
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "additionalProperties": {
    ///    "type": "array",
    ///    "items": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct MultivaluedHashMapStringString(pub std::collections::HashMap<String, Vec<String>>);
    impl std::ops::Deref for MultivaluedHashMapStringString {
        type Target = std::collections::HashMap<String, Vec<String>>;
        fn deref(&self) -> &std::collections::HashMap<String, Vec<String>> {
            &self.0
        }
    }

    impl From<MultivaluedHashMapStringString> for std::collections::HashMap<String, Vec<String>> {
        fn from(value: MultivaluedHashMapStringString) -> Self {
            value.0
        }
    }

    impl From<&MultivaluedHashMapStringString> for MultivaluedHashMapStringString {
        fn from(value: &MultivaluedHashMapStringString) -> Self {
            value.clone()
        }
    }

    impl From<std::collections::HashMap<String, Vec<String>>> for MultivaluedHashMapStringString {
        fn from(value: std::collections::HashMap<String, Vec<String>>) -> Self {
            Self(value)
        }
    }

    ///MultivaluedMapStringString
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "additionalProperties": {
    ///    "type": "array",
    ///    "items": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct MultivaluedMapStringString(pub std::collections::HashMap<String, Vec<String>>);
    impl std::ops::Deref for MultivaluedMapStringString {
        type Target = std::collections::HashMap<String, Vec<String>>;
        fn deref(&self) -> &std::collections::HashMap<String, Vec<String>> {
            &self.0
        }
    }

    impl From<MultivaluedMapStringString> for std::collections::HashMap<String, Vec<String>> {
        fn from(value: MultivaluedMapStringString) -> Self {
            value.0
        }
    }

    impl From<&MultivaluedMapStringString> for MultivaluedMapStringString {
        fn from(value: &MultivaluedMapStringString) -> Self {
            value.clone()
        }
    }

    impl From<std::collections::HashMap<String, Vec<String>>> for MultivaluedMapStringString {
        fn from(value: std::collections::HashMap<String, Vec<String>>) -> Self {
            Self(value)
        }
    }

    ///OAuthClientRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "deprecated": true,
    ///  "type": "object",
    ///  "properties": {
    ///    "access": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "boolean"
    ///      }
    ///    },
    ///    "adminUrl": {
    ///      "type": "string"
    ///    },
    ///    "alwaysDisplayInConsole": {
    ///      "type": "boolean"
    ///    },
    ///    "attributes": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "authenticationFlowBindingOverrides": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "authorizationServicesEnabled": {
    ///      "type": "boolean"
    ///    },
    ///    "authorizationSettings": {
    ///      "$ref": "#/components/schemas/ResourceServerRepresentation"
    ///    },
    ///    "baseUrl": {
    ///      "type": "string"
    ///    },
    ///    "bearerOnly": {
    ///      "type": "boolean"
    ///    },
    ///    "claims": {
    ///      "deprecated": true,
    ///      "type": "object",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/ClaimRepresentation"
    ///        }
    ///      ]
    ///    },
    ///    "clientAuthenticatorType": {
    ///      "type": "string"
    ///    },
    ///    "clientId": {
    ///      "type": "string"
    ///    },
    ///    "clientTemplate": {
    ///      "deprecated": true,
    ///      "type": "string"
    ///    },
    ///    "consentRequired": {
    ///      "type": "boolean"
    ///    },
    ///    "defaultClientScopes": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "defaultRoles": {
    ///      "deprecated": true,
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "directAccessGrantsEnabled": {
    ///      "type": "boolean"
    ///    },
    ///    "directGrantsOnly": {
    ///      "deprecated": true,
    ///      "type": "boolean"
    ///    },
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "frontchannelLogout": {
    ///      "type": "boolean"
    ///    },
    ///    "fullScopeAllowed": {
    ///      "type": "boolean"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "implicitFlowEnabled": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "nodeReRegistrationTimeout": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "notBefore": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "optionalClientScopes": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "origin": {
    ///      "type": "string"
    ///    },
    ///    "protocol": {
    ///      "type": "string"
    ///    },
    ///    "protocolMappers": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ProtocolMapperRepresentation"
    ///      }
    ///    },
    ///    "publicClient": {
    ///      "type": "boolean"
    ///    },
    ///    "redirectUris": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "registeredNodes": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "integer",
    ///        "format": "int32"
    ///      }
    ///    },
    ///    "registrationAccessToken": {
    ///      "type": "string"
    ///    },
    ///    "rootUrl": {
    ///      "type": "string"
    ///    },
    ///    "secret": {
    ///      "type": "string"
    ///    },
    ///    "serviceAccountsEnabled": {
    ///      "type": "boolean"
    ///    },
    ///    "standardFlowEnabled": {
    ///      "type": "boolean"
    ///    },
    ///    "surrogateAuthRequired": {
    ///      "type": "boolean"
    ///    },
    ///    "useTemplateConfig": {
    ///      "deprecated": true,
    ///      "type": "boolean"
    ///    },
    ///    "useTemplateMappers": {
    ///      "deprecated": true,
    ///      "type": "boolean"
    ///    },
    ///    "useTemplateScope": {
    ///      "deprecated": true,
    ///      "type": "boolean"
    ///    },
    ///    "webOrigins": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct OAuthClientRepresentation {
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub access: std::collections::HashMap<String, bool>,
        #[serde(rename = "adminUrl", default, skip_serializing_if = "Option::is_none")]
        pub admin_url: Option<String>,
        #[serde(
            rename = "alwaysDisplayInConsole",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub always_display_in_console: Option<bool>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub attributes: std::collections::HashMap<String, String>,
        #[serde(
            rename = "authenticationFlowBindingOverrides",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub authentication_flow_binding_overrides: std::collections::HashMap<String, String>,
        #[serde(
            rename = "authorizationServicesEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub authorization_services_enabled: Option<bool>,
        #[serde(
            rename = "authorizationSettings",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub authorization_settings: Option<ResourceServerRepresentation>,
        #[serde(rename = "baseUrl", default, skip_serializing_if = "Option::is_none")]
        pub base_url: Option<String>,
        #[serde(
            rename = "bearerOnly",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub bearer_only: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub claims: Option<ClaimRepresentation>,
        #[serde(
            rename = "clientAuthenticatorType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub client_authenticator_type: Option<String>,
        #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
        pub client_id: Option<String>,
        #[serde(
            rename = "clientTemplate",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub client_template: Option<String>,
        #[serde(
            rename = "consentRequired",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub consent_required: Option<bool>,
        #[serde(
            rename = "defaultClientScopes",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub default_client_scopes: Vec<String>,
        #[serde(
            rename = "defaultRoles",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub default_roles: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(
            rename = "directAccessGrantsEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub direct_access_grants_enabled: Option<bool>,
        #[serde(
            rename = "directGrantsOnly",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub direct_grants_only: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[serde(
            rename = "frontchannelLogout",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub frontchannel_logout: Option<bool>,
        #[serde(
            rename = "fullScopeAllowed",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub full_scope_allowed: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(
            rename = "implicitFlowEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub implicit_flow_enabled: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(
            rename = "nodeReRegistrationTimeout",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub node_re_registration_timeout: Option<i32>,
        #[serde(rename = "notBefore", default, skip_serializing_if = "Option::is_none")]
        pub not_before: Option<i32>,
        #[serde(
            rename = "optionalClientScopes",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub optional_client_scopes: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub origin: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub protocol: Option<String>,
        #[serde(
            rename = "protocolMappers",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub protocol_mappers: Vec<ProtocolMapperRepresentation>,
        #[serde(
            rename = "publicClient",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub public_client: Option<bool>,
        #[serde(
            rename = "redirectUris",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub redirect_uris: Vec<String>,
        #[serde(
            rename = "registeredNodes",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub registered_nodes: std::collections::HashMap<String, i32>,
        #[serde(
            rename = "registrationAccessToken",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub registration_access_token: Option<String>,
        #[serde(rename = "rootUrl", default, skip_serializing_if = "Option::is_none")]
        pub root_url: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub secret: Option<String>,
        #[serde(
            rename = "serviceAccountsEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub service_accounts_enabled: Option<bool>,
        #[serde(
            rename = "standardFlowEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub standard_flow_enabled: Option<bool>,
        #[serde(
            rename = "surrogateAuthRequired",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub surrogate_auth_required: Option<bool>,
        #[serde(
            rename = "useTemplateConfig",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub use_template_config: Option<bool>,
        #[serde(
            rename = "useTemplateMappers",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub use_template_mappers: Option<bool>,
        #[serde(
            rename = "useTemplateScope",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub use_template_scope: Option<bool>,
        #[serde(rename = "webOrigins", default, skip_serializing_if = "Vec::is_empty")]
        pub web_origins: Vec<String>,
    }

    impl From<&OAuthClientRepresentation> for OAuthClientRepresentation {
        fn from(value: &OAuthClientRepresentation) -> Self {
            value.clone()
        }
    }

    ///PathCacheConfig
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "lifespan": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "max-entries": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PathCacheConfig {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub lifespan: Option<i64>,
        #[serde(
            rename = "max-entries",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub max_entries: Option<i32>,
    }

    impl From<&PathCacheConfig> for PathCacheConfig {
        fn from(value: &PathCacheConfig) -> Self {
            value.clone()
        }
    }

    ///PathConfig
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "claim-information-point": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "object",
    ///        "additionalProperties": {}
    ///      }
    ///    },
    ///    "enforcement-mode": {
    ///      "$ref": "#/components/schemas/EnforcementMode"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "invalidated": {
    ///      "type": "boolean"
    ///    },
    ///    "methods": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/MethodConfig"
    ///      }
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "path": {
    ///      "type": "string"
    ///    },
    ///    "scopes": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "static": {
    ///      "type": "boolean"
    ///    },
    ///    "staticPath": {
    ///      "type": "boolean"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PathConfig {
        #[serde(
            rename = "claim-information-point",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub claim_information_point:
            std::collections::HashMap<String, serde_json::Map<String, serde_json::Value>>,
        #[serde(
            rename = "enforcement-mode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub enforcement_mode: Option<EnforcementMode>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub invalidated: Option<bool>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub methods: Vec<MethodConfig>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub path: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub scopes: Vec<String>,
        #[serde(rename = "static", default, skip_serializing_if = "Option::is_none")]
        pub static_: Option<bool>,
        #[serde(
            rename = "staticPath",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub static_path: Option<bool>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    impl From<&PathConfig> for PathConfig {
        fn from(value: &PathConfig) -> Self {
            value.clone()
        }
    }

    ///PathSegment
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "matrixParameters": {
    ///      "$ref": "#/components/schemas/MultivaluedMapStringString"
    ///    },
    ///    "path": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PathSegment {
        #[serde(
            rename = "matrixParameters",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub matrix_parameters: Option<MultivaluedMapStringString>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub path: Option<String>,
    }

    impl From<&PathSegment> for PathSegment {
        fn from(value: &PathSegment) -> Self {
            value.clone()
        }
    }

    ///Permission
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "claims": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "array",
    ///        "items": {
    ///          "type": "string"
    ///        },
    ///        "uniqueItems": true
    ///      }
    ///    },
    ///    "rsid": {
    ///      "type": "string"
    ///    },
    ///    "rsname": {
    ///      "type": "string"
    ///    },
    ///    "scopes": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      },
    ///      "uniqueItems": true
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct Permission {
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub claims: std::collections::HashMap<String, Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub rsid: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub rsname: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub scopes: Option<Vec<String>>,
    }

    impl From<&Permission> for Permission {
        fn from(value: &Permission) -> Self {
            value.clone()
        }
    }

    ///PolicyEnforcementMode
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "ENFORCING",
    ///    "PERMISSIVE",
    ///    "DISABLED"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum PolicyEnforcementMode {
        #[serde(rename = "ENFORCING")]
        Enforcing,
        #[serde(rename = "PERMISSIVE")]
        Permissive,
        #[serde(rename = "DISABLED")]
        Disabled,
    }

    impl From<&PolicyEnforcementMode> for PolicyEnforcementMode {
        fn from(value: &PolicyEnforcementMode) -> Self {
            value.clone()
        }
    }

    impl ToString for PolicyEnforcementMode {
        fn to_string(&self) -> String {
            match *self {
                Self::Enforcing => "ENFORCING".to_string(),
                Self::Permissive => "PERMISSIVE".to_string(),
                Self::Disabled => "DISABLED".to_string(),
            }
        }
    }

    impl std::str::FromStr for PolicyEnforcementMode {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "ENFORCING" => Ok(Self::Enforcing),
                "PERMISSIVE" => Ok(Self::Permissive),
                "DISABLED" => Ok(Self::Disabled),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for PolicyEnforcementMode {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for PolicyEnforcementMode {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for PolicyEnforcementMode {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///PolicyEnforcerConfig
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "auth-server-url": {
    ///      "type": "string"
    ///    },
    ///    "claim-information-point": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "object",
    ///        "additionalProperties": {}
    ///      }
    ///    },
    ///    "credentials": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "enforcement-mode": {
    ///      "$ref": "#/components/schemas/EnforcementMode"
    ///    },
    ///    "http-method-as-scope": {
    ///      "type": "boolean"
    ///    },
    ///    "lazy-load-paths": {
    ///      "type": "boolean"
    ///    },
    ///    "on-deny-redirect-to": {
    ///      "type": "string"
    ///    },
    ///    "path-cache": {
    ///      "$ref": "#/components/schemas/PathCacheConfig"
    ///    },
    ///    "paths": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/PathConfig"
    ///      }
    ///    },
    ///    "realm": {
    ///      "type": "string"
    ///    },
    ///    "resource": {
    ///      "type": "string"
    ///    },
    ///    "user-managed-access": {
    ///      "$ref": "#/components/schemas/UserManagedAccessConfig"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PolicyEnforcerConfig {
        #[serde(
            rename = "auth-server-url",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub auth_server_url: Option<String>,
        #[serde(
            rename = "claim-information-point",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub claim_information_point:
            std::collections::HashMap<String, serde_json::Map<String, serde_json::Value>>,
        #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
        pub credentials: serde_json::Map<String, serde_json::Value>,
        #[serde(
            rename = "enforcement-mode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub enforcement_mode: Option<EnforcementMode>,
        #[serde(
            rename = "http-method-as-scope",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub http_method_as_scope: Option<bool>,
        #[serde(
            rename = "lazy-load-paths",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub lazy_load_paths: Option<bool>,
        #[serde(
            rename = "on-deny-redirect-to",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub on_deny_redirect_to: Option<String>,
        #[serde(
            rename = "path-cache",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub path_cache: Option<PathCacheConfig>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub paths: Vec<PathConfig>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub realm: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resource: Option<String>,
        #[serde(
            rename = "user-managed-access",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub user_managed_access: Option<UserManagedAccessConfig>,
    }

    impl From<&PolicyEnforcerConfig> for PolicyEnforcerConfig {
        fn from(value: &PolicyEnforcerConfig) -> Self {
            value.clone()
        }
    }

    ///PolicyEvaluationRequest
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "clientId": {
    ///      "type": "string"
    ///    },
    ///    "context": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "object",
    ///        "additionalProperties": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "entitlements": {
    ///      "type": "boolean"
    ///    },
    ///    "resources": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ResourceRepresentation"
    ///      }
    ///    },
    ///    "roleIds": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "userId": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PolicyEvaluationRequest {
        #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
        pub client_id: Option<String>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub context: std::collections::HashMap<String, std::collections::HashMap<String, String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub entitlements: Option<bool>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub resources: Vec<ResourceRepresentation>,
        #[serde(rename = "roleIds", default, skip_serializing_if = "Vec::is_empty")]
        pub role_ids: Vec<String>,
        #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
        pub user_id: Option<String>,
    }

    impl From<&PolicyEvaluationRequest> for PolicyEvaluationRequest {
        fn from(value: &PolicyEvaluationRequest) -> Self {
            value.clone()
        }
    }

    ///PolicyEvaluationResponse
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "entitlements": {
    ///      "type": "boolean"
    ///    },
    ///    "results": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/EvaluationResultRepresentation"
    ///      }
    ///    },
    ///    "rpt": {
    ///      "$ref": "#/components/schemas/AccessToken"
    ///    },
    ///    "status": {
    ///      "$ref": "#/components/schemas/DecisionEffect"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PolicyEvaluationResponse {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub entitlements: Option<bool>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub results: Vec<EvaluationResultRepresentation>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub rpt: Option<AccessToken>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<DecisionEffect>,
    }

    impl From<&PolicyEvaluationResponse> for PolicyEvaluationResponse {
        fn from(value: &PolicyEvaluationResponse) -> Self {
            value.clone()
        }
    }

    ///PolicyProviderRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "group": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PolicyProviderRepresentation {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub group: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    impl From<&PolicyProviderRepresentation> for PolicyProviderRepresentation {
        fn from(value: &PolicyProviderRepresentation) -> Self {
            value.clone()
        }
    }

    ///PolicyRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "config": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "decisionStrategy": {
    ///      "$ref": "#/components/schemas/DecisionStrategy"
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "logic": {
    ///      "$ref": "#/components/schemas/Logic"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "owner": {
    ///      "type": "string"
    ///    },
    ///    "policies": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      },
    ///      "uniqueItems": true
    ///    },
    ///    "resources": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      },
    ///      "uniqueItems": true
    ///    },
    ///    "resourcesData": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ResourceRepresentation"
    ///      },
    ///      "uniqueItems": true
    ///    },
    ///    "scopes": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      },
    ///      "uniqueItems": true
    ///    },
    ///    "scopesData": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ScopeRepresentation"
    ///      },
    ///      "uniqueItems": true
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PolicyRepresentation {
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub config: std::collections::HashMap<String, String>,
        #[serde(
            rename = "decisionStrategy",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub decision_strategy: Option<DecisionStrategy>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub logic: Option<Logic>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub owner: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub policies: Option<Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub resources: Option<Vec<String>>,
        #[serde(
            rename = "resourcesData",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub resources_data: Option<Vec<ResourceRepresentation>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub scopes: Option<Vec<String>>,
        #[serde(
            rename = "scopesData",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub scopes_data: Option<Vec<ScopeRepresentation>>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
    }

    impl From<&PolicyRepresentation> for PolicyRepresentation {
        fn from(value: &PolicyRepresentation) -> Self {
            value.clone()
        }
    }

    ///PolicyResultRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "associatedPolicies": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/PolicyResultRepresentation"
    ///      }
    ///    },
    ///    "policy": {
    ///      "$ref": "#/components/schemas/PolicyRepresentation"
    ///    },
    ///    "scopes": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      },
    ///      "uniqueItems": true
    ///    },
    ///    "status": {
    ///      "$ref": "#/components/schemas/DecisionEffect"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PolicyResultRepresentation {
        #[serde(
            rename = "associatedPolicies",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub associated_policies: Vec<PolicyResultRepresentation>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub policy: Option<PolicyRepresentation>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub scopes: Option<Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub status: Option<DecisionEffect>,
    }

    impl From<&PolicyResultRepresentation> for PolicyResultRepresentation {
        fn from(value: &PolicyResultRepresentation) -> Self {
            value.clone()
        }
    }

    ///ProtocolMapperEvaluationRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "containerId": {
    ///      "type": "string"
    ///    },
    ///    "containerName": {
    ///      "type": "string"
    ///    },
    ///    "containerType": {
    ///      "type": "string"
    ///    },
    ///    "mapperId": {
    ///      "type": "string"
    ///    },
    ///    "mapperName": {
    ///      "type": "string"
    ///    },
    ///    "protocolMapper": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ProtocolMapperEvaluationRepresentation {
        #[serde(
            rename = "containerId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub container_id: Option<String>,
        #[serde(
            rename = "containerName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub container_name: Option<String>,
        #[serde(
            rename = "containerType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub container_type: Option<String>,
        #[serde(rename = "mapperId", default, skip_serializing_if = "Option::is_none")]
        pub mapper_id: Option<String>,
        #[serde(
            rename = "mapperName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub mapper_name: Option<String>,
        #[serde(
            rename = "protocolMapper",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub protocol_mapper: Option<String>,
    }

    impl From<&ProtocolMapperEvaluationRepresentation> for ProtocolMapperEvaluationRepresentation {
        fn from(value: &ProtocolMapperEvaluationRepresentation) -> Self {
            value.clone()
        }
    }

    ///ProtocolMapperRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "config": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "consentRequired": {
    ///      "deprecated": true,
    ///      "type": "boolean"
    ///    },
    ///    "consentText": {
    ///      "deprecated": true,
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "protocol": {
    ///      "type": "string"
    ///    },
    ///    "protocolMapper": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ProtocolMapperRepresentation {
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub config: std::collections::HashMap<String, String>,
        #[serde(
            rename = "consentRequired",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub consent_required: Option<bool>,
        #[serde(
            rename = "consentText",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub consent_text: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub protocol: Option<String>,
        #[serde(
            rename = "protocolMapper",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub protocol_mapper: Option<String>,
    }

    impl From<&ProtocolMapperRepresentation> for ProtocolMapperRepresentation {
        fn from(value: &ProtocolMapperRepresentation) -> Self {
            value.clone()
        }
    }

    ///PublishedRealmRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "account-service": {
    ///      "type": "string"
    ///    },
    ///    "public_key": {
    ///      "type": "string"
    ///    },
    ///    "realm": {
    ///      "type": "string"
    ///    },
    ///    "token-service": {
    ///      "type": "string"
    ///    },
    ///    "tokens-not-before": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct PublishedRealmRepresentation {
        #[serde(
            rename = "account-service",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub account_service: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub public_key: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub realm: Option<String>,
        #[serde(
            rename = "token-service",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub token_service: Option<String>,
        #[serde(
            rename = "tokens-not-before",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub tokens_not_before: Option<i32>,
    }

    impl From<&PublishedRealmRepresentation> for PublishedRealmRepresentation {
        fn from(value: &PublishedRealmRepresentation) -> Self {
            value.clone()
        }
    }

    ///RealmEventsConfigRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "adminEventsDetailsEnabled": {
    ///      "type": "boolean"
    ///    },
    ///    "adminEventsEnabled": {
    ///      "type": "boolean"
    ///    },
    ///    "enabledEventTypes": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "eventsEnabled": {
    ///      "type": "boolean"
    ///    },
    ///    "eventsExpiration": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "eventsListeners": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct RealmEventsConfigRepresentation {
        #[serde(
            rename = "adminEventsDetailsEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub admin_events_details_enabled: Option<bool>,
        #[serde(
            rename = "adminEventsEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub admin_events_enabled: Option<bool>,
        #[serde(
            rename = "enabledEventTypes",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub enabled_event_types: Vec<String>,
        #[serde(
            rename = "eventsEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub events_enabled: Option<bool>,
        #[serde(
            rename = "eventsExpiration",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub events_expiration: Option<i64>,
        #[serde(
            rename = "eventsListeners",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub events_listeners: Vec<String>,
    }

    impl From<&RealmEventsConfigRepresentation> for RealmEventsConfigRepresentation {
        fn from(value: &RealmEventsConfigRepresentation) -> Self {
            value.clone()
        }
    }

    ///RealmRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "accessCodeLifespan": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "accessCodeLifespanLogin": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "accessCodeLifespanUserAction": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "accessTokenLifespan": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "accessTokenLifespanForImplicitFlow": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "accountTheme": {
    ///      "type": "string"
    ///    },
    ///    "actionTokenGeneratedByAdminLifespan": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "actionTokenGeneratedByUserLifespan": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "adminEventsDetailsEnabled": {
    ///      "type": "boolean"
    ///    },
    ///    "adminEventsEnabled": {
    ///      "type": "boolean"
    ///    },
    ///    "adminTheme": {
    ///      "type": "string"
    ///    },
    ///    "applicationScopeMappings": {
    ///      "deprecated": true,
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "array",
    ///        "items": {
    ///          "$ref": "#/components/schemas/ScopeMappingRepresentation"
    ///        }
    ///      }
    ///    },
    ///    "applications": {
    ///      "deprecated": true,
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ApplicationRepresentation"
    ///      }
    ///    },
    ///    "attributes": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "authenticationFlows": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/AuthenticationFlowRepresentation"
    ///      }
    ///    },
    ///    "authenticatorConfig": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/AuthenticatorConfigRepresentation"
    ///      }
    ///    },
    ///    "browserFlow": {
    ///      "type": "string"
    ///    },
    ///    "browserSecurityHeaders": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "bruteForceProtected": {
    ///      "type": "boolean"
    ///    },
    ///    "certificate": {
    ///      "deprecated": true,
    ///      "type": "string"
    ///    },
    ///    "clientAuthenticationFlow": {
    ///      "type": "string"
    ///    },
    ///    "clientOfflineSessionIdleTimeout": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "clientOfflineSessionMaxLifespan": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "clientPolicies": {
    ///      "$ref": "#/components/schemas/ClientPoliciesRepresentation"
    ///    },
    ///    "clientProfiles": {
    ///      "$ref": "#/components/schemas/ClientProfilesRepresentation"
    ///    },
    ///    "clientScopeMappings": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "array",
    ///        "items": {
    ///          "$ref": "#/components/schemas/ScopeMappingRepresentation"
    ///        }
    ///      }
    ///    },
    ///    "clientScopes": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ClientScopeRepresentation"
    ///      }
    ///    },
    ///    "clientSessionIdleTimeout": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "clientSessionMaxLifespan": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "clientTemplates": {
    ///      "deprecated": true,
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ClientTemplateRepresentation"
    ///      }
    ///    },
    ///    "clients": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ClientRepresentation"
    ///      }
    ///    },
    ///    "codeSecret": {
    ///      "deprecated": true,
    ///      "type": "string"
    ///    },
    ///    "components": {
    ///      "$ref":
    /// "#/components/schemas/
    /// MultivaluedHashMapStringComponentExportRepresentation"
    ///    },
    ///    "defaultDefaultClientScopes": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "defaultGroups": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "defaultLocale": {
    ///      "type": "string"
    ///    },
    ///    "defaultOptionalClientScopes": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "defaultRole": {
    ///      "$ref": "#/components/schemas/RoleRepresentation"
    ///    },
    ///    "defaultRoles": {
    ///      "deprecated": true,
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "defaultSignatureAlgorithm": {
    ///      "type": "string"
    ///    },
    ///    "directGrantFlow": {
    ///      "type": "string"
    ///    },
    ///    "displayName": {
    ///      "type": "string"
    ///    },
    ///    "displayNameHtml": {
    ///      "type": "string"
    ///    },
    ///    "dockerAuthenticationFlow": {
    ///      "type": "string"
    ///    },
    ///    "duplicateEmailsAllowed": {
    ///      "type": "boolean"
    ///    },
    ///    "editUsernameAllowed": {
    ///      "type": "boolean"
    ///    },
    ///    "emailTheme": {
    ///      "type": "string"
    ///    },
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "enabledEventTypes": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "eventsEnabled": {
    ///      "type": "boolean"
    ///    },
    ///    "eventsExpiration": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "eventsListeners": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "failureFactor": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "federatedUsers": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/UserRepresentation"
    ///      }
    ///    },
    ///    "firstBrokerLoginFlow": {
    ///      "type": "string"
    ///    },
    ///    "groups": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/GroupRepresentation"
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "identityProviderMappers": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref":
    /// "#/components/schemas/IdentityProviderMapperRepresentation"
    ///      }
    ///    },
    ///    "identityProviders": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/IdentityProviderRepresentation"
    ///      }
    ///    },
    ///    "internationalizationEnabled": {
    ///      "type": "boolean"
    ///    },
    ///    "keycloakVersion": {
    ///      "type": "string"
    ///    },
    ///    "localizationTexts": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "object",
    ///        "additionalProperties": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "loginTheme": {
    ///      "type": "string"
    ///    },
    ///    "loginWithEmailAllowed": {
    ///      "type": "boolean"
    ///    },
    ///    "maxDeltaTimeSeconds": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "maxFailureWaitSeconds": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "maxTemporaryLockouts": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "minimumQuickLoginWaitSeconds": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "notBefore": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "oAuth2DeviceCodeLifespan": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "oAuth2DevicePollingInterval": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "oauth2DeviceCodeLifespan": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "oauth2DevicePollingInterval": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "oauthClients": {
    ///      "deprecated": true,
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/OAuthClientRepresentation"
    ///      }
    ///    },
    ///    "offlineSessionIdleTimeout": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "offlineSessionMaxLifespan": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "offlineSessionMaxLifespanEnabled": {
    ///      "type": "boolean"
    ///    },
    ///    "otpPolicyAlgorithm": {
    ///      "type": "string"
    ///    },
    ///    "otpPolicyCodeReusable": {
    ///      "type": "boolean"
    ///    },
    ///    "otpPolicyDigits": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "otpPolicyInitialCounter": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "otpPolicyLookAheadWindow": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "otpPolicyPeriod": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "otpPolicyType": {
    ///      "type": "string"
    ///    },
    ///    "otpSupportedApplications": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "passwordCredentialGrantAllowed": {
    ///      "deprecated": true,
    ///      "type": "boolean"
    ///    },
    ///    "passwordPolicy": {
    ///      "type": "string"
    ///    },
    ///    "permanentLockout": {
    ///      "type": "boolean"
    ///    },
    ///    "privateKey": {
    ///      "deprecated": true,
    ///      "type": "string"
    ///    },
    ///    "protocolMappers": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ProtocolMapperRepresentation"
    ///      }
    ///    },
    ///    "publicKey": {
    ///      "deprecated": true,
    ///      "type": "string"
    ///    },
    ///    "quickLoginCheckMilliSeconds": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "realm": {
    ///      "type": "string"
    ///    },
    ///    "realmCacheEnabled": {
    ///      "deprecated": true,
    ///      "type": "boolean"
    ///    },
    ///    "refreshTokenMaxReuse": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "registrationAllowed": {
    ///      "type": "boolean"
    ///    },
    ///    "registrationEmailAsUsername": {
    ///      "type": "boolean"
    ///    },
    ///    "registrationFlow": {
    ///      "type": "string"
    ///    },
    ///    "rememberMe": {
    ///      "type": "boolean"
    ///    },
    ///    "requiredActions": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref":
    /// "#/components/schemas/RequiredActionProviderRepresentation"
    ///      }
    ///    },
    ///    "requiredCredentials": {
    ///      "deprecated": true,
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      },
    ///      "uniqueItems": true
    ///    },
    ///    "resetCredentialsFlow": {
    ///      "type": "string"
    ///    },
    ///    "resetPasswordAllowed": {
    ///      "type": "boolean"
    ///    },
    ///    "revokeRefreshToken": {
    ///      "type": "boolean"
    ///    },
    ///    "roles": {
    ///      "$ref": "#/components/schemas/RolesRepresentation"
    ///    },
    ///    "scopeMappings": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ScopeMappingRepresentation"
    ///      }
    ///    },
    ///    "smtpServer": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "social": {
    ///      "deprecated": true,
    ///      "type": "boolean"
    ///    },
    ///    "socialProviders": {
    ///      "deprecated": true,
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "sslRequired": {
    ///      "type": "string"
    ///    },
    ///    "ssoSessionIdleTimeout": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "ssoSessionIdleTimeoutRememberMe": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "ssoSessionMaxLifespan": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "ssoSessionMaxLifespanRememberMe": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "supportedLocales": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      },
    ///      "uniqueItems": true
    ///    },
    ///    "updateProfileOnInitialSocialLogin": {
    ///      "deprecated": true,
    ///      "type": "boolean"
    ///    },
    ///    "userCacheEnabled": {
    ///      "deprecated": true,
    ///      "type": "boolean"
    ///    },
    ///    "userFederationMappers": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/UserFederationMapperRepresentation"
    ///      }
    ///    },
    ///    "userFederationProviders": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref":
    /// "#/components/schemas/UserFederationProviderRepresentation"
    ///      }
    ///    },
    ///    "userManagedAccessAllowed": {
    ///      "type": "boolean"
    ///    },
    ///    "users": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/UserRepresentation"
    ///      }
    ///    },
    ///    "verifyEmail": {
    ///      "type": "boolean"
    ///    },
    ///    "waitIncrementSeconds": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "webAuthnPolicyAcceptableAaguids": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "webAuthnPolicyAttestationConveyancePreference": {
    ///      "type": "string"
    ///    },
    ///    "webAuthnPolicyAuthenticatorAttachment": {
    ///      "type": "string"
    ///    },
    ///    "webAuthnPolicyAvoidSameAuthenticatorRegister": {
    ///      "type": "boolean"
    ///    },
    ///    "webAuthnPolicyCreateTimeout": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "webAuthnPolicyExtraOrigins": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "webAuthnPolicyPasswordlessAcceptableAaguids": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "webAuthnPolicyPasswordlessAttestationConveyancePreference": {
    ///      "type": "string"
    ///    },
    ///    "webAuthnPolicyPasswordlessAuthenticatorAttachment": {
    ///      "type": "string"
    ///    },
    ///    "webAuthnPolicyPasswordlessAvoidSameAuthenticatorRegister": {
    ///      "type": "boolean"
    ///    },
    ///    "webAuthnPolicyPasswordlessCreateTimeout": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "webAuthnPolicyPasswordlessExtraOrigins": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "webAuthnPolicyPasswordlessRequireResidentKey": {
    ///      "type": "string"
    ///    },
    ///    "webAuthnPolicyPasswordlessRpEntityName": {
    ///      "type": "string"
    ///    },
    ///    "webAuthnPolicyPasswordlessRpId": {
    ///      "type": "string"
    ///    },
    ///    "webAuthnPolicyPasswordlessSignatureAlgorithms": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "webAuthnPolicyPasswordlessUserVerificationRequirement": {
    ///      "type": "string"
    ///    },
    ///    "webAuthnPolicyRequireResidentKey": {
    ///      "type": "string"
    ///    },
    ///    "webAuthnPolicyRpEntityName": {
    ///      "type": "string"
    ///    },
    ///    "webAuthnPolicyRpId": {
    ///      "type": "string"
    ///    },
    ///    "webAuthnPolicySignatureAlgorithms": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "webAuthnPolicyUserVerificationRequirement": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct RealmRepresentation {
        #[serde(
            rename = "accessCodeLifespan",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub access_code_lifespan: Option<i32>,
        #[serde(
            rename = "accessCodeLifespanLogin",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub access_code_lifespan_login: Option<i32>,
        #[serde(
            rename = "accessCodeLifespanUserAction",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub access_code_lifespan_user_action: Option<i32>,
        #[serde(
            rename = "accessTokenLifespan",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub access_token_lifespan: Option<i32>,
        #[serde(
            rename = "accessTokenLifespanForImplicitFlow",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub access_token_lifespan_for_implicit_flow: Option<i32>,
        #[serde(
            rename = "accountTheme",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub account_theme: Option<String>,
        #[serde(
            rename = "actionTokenGeneratedByAdminLifespan",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub action_token_generated_by_admin_lifespan: Option<i32>,
        #[serde(
            rename = "actionTokenGeneratedByUserLifespan",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub action_token_generated_by_user_lifespan: Option<i32>,
        #[serde(
            rename = "adminEventsDetailsEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub admin_events_details_enabled: Option<bool>,
        #[serde(
            rename = "adminEventsEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub admin_events_enabled: Option<bool>,
        #[serde(
            rename = "adminTheme",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub admin_theme: Option<String>,
        #[serde(
            rename = "applicationScopeMappings",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub application_scope_mappings:
            std::collections::HashMap<String, Vec<ScopeMappingRepresentation>>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub applications: Vec<ApplicationRepresentation>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub attributes: std::collections::HashMap<String, String>,
        #[serde(
            rename = "authenticationFlows",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub authentication_flows: Vec<AuthenticationFlowRepresentation>,
        #[serde(
            rename = "authenticatorConfig",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub authenticator_config: Vec<AuthenticatorConfigRepresentation>,
        #[serde(
            rename = "browserFlow",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub browser_flow: Option<String>,
        #[serde(
            rename = "browserSecurityHeaders",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub browser_security_headers: std::collections::HashMap<String, String>,
        #[serde(
            rename = "bruteForceProtected",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub brute_force_protected: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub certificate: Option<String>,
        #[serde(
            rename = "clientAuthenticationFlow",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub client_authentication_flow: Option<String>,
        #[serde(
            rename = "clientOfflineSessionIdleTimeout",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub client_offline_session_idle_timeout: Option<i32>,
        #[serde(
            rename = "clientOfflineSessionMaxLifespan",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub client_offline_session_max_lifespan: Option<i32>,
        #[serde(
            rename = "clientPolicies",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub client_policies: Option<ClientPoliciesRepresentation>,
        #[serde(
            rename = "clientProfiles",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub client_profiles: Option<ClientProfilesRepresentation>,
        #[serde(
            rename = "clientScopeMappings",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub client_scope_mappings:
            std::collections::HashMap<String, Vec<ScopeMappingRepresentation>>,
        #[serde(
            rename = "clientScopes",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub client_scopes: Vec<ClientScopeRepresentation>,
        #[serde(
            rename = "clientSessionIdleTimeout",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub client_session_idle_timeout: Option<i32>,
        #[serde(
            rename = "clientSessionMaxLifespan",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub client_session_max_lifespan: Option<i32>,
        #[serde(
            rename = "clientTemplates",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub client_templates: Vec<ClientTemplateRepresentation>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub clients: Vec<ClientRepresentation>,
        #[serde(
            rename = "codeSecret",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub code_secret: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub components: Option<MultivaluedHashMapStringComponentExportRepresentation>,
        #[serde(
            rename = "defaultDefaultClientScopes",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub default_default_client_scopes: Vec<String>,
        #[serde(
            rename = "defaultGroups",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub default_groups: Vec<String>,
        #[serde(
            rename = "defaultLocale",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub default_locale: Option<String>,
        #[serde(
            rename = "defaultOptionalClientScopes",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub default_optional_client_scopes: Vec<String>,
        #[serde(
            rename = "defaultRole",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub default_role: Option<RoleRepresentation>,
        #[serde(
            rename = "defaultRoles",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub default_roles: Vec<String>,
        #[serde(
            rename = "defaultSignatureAlgorithm",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub default_signature_algorithm: Option<String>,
        #[serde(
            rename = "directGrantFlow",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub direct_grant_flow: Option<String>,
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub display_name: Option<String>,
        #[serde(
            rename = "displayNameHtml",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub display_name_html: Option<String>,
        #[serde(
            rename = "dockerAuthenticationFlow",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub docker_authentication_flow: Option<String>,
        #[serde(
            rename = "duplicateEmailsAllowed",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub duplicate_emails_allowed: Option<bool>,
        #[serde(
            rename = "editUsernameAllowed",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub edit_username_allowed: Option<bool>,
        #[serde(
            rename = "emailTheme",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub email_theme: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[serde(
            rename = "enabledEventTypes",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub enabled_event_types: Vec<String>,
        #[serde(
            rename = "eventsEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub events_enabled: Option<bool>,
        #[serde(
            rename = "eventsExpiration",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub events_expiration: Option<i64>,
        #[serde(
            rename = "eventsListeners",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub events_listeners: Vec<String>,
        #[serde(
            rename = "failureFactor",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub failure_factor: Option<i32>,
        #[serde(
            rename = "federatedUsers",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub federated_users: Vec<UserRepresentation>,
        #[serde(
            rename = "firstBrokerLoginFlow",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub first_broker_login_flow: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub groups: Vec<GroupRepresentation>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(
            rename = "identityProviderMappers",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub identity_provider_mappers: Vec<IdentityProviderMapperRepresentation>,
        #[serde(
            rename = "identityProviders",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub identity_providers: Vec<IdentityProviderRepresentation>,
        #[serde(
            rename = "internationalizationEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub internationalization_enabled: Option<bool>,
        #[serde(
            rename = "keycloakVersion",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub keycloak_version: Option<String>,
        #[serde(
            rename = "localizationTexts",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub localization_texts:
            std::collections::HashMap<String, std::collections::HashMap<String, String>>,
        #[serde(
            rename = "loginTheme",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub login_theme: Option<String>,
        #[serde(
            rename = "loginWithEmailAllowed",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub login_with_email_allowed: Option<bool>,
        #[serde(
            rename = "maxDeltaTimeSeconds",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub max_delta_time_seconds: Option<i32>,
        #[serde(
            rename = "maxFailureWaitSeconds",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub max_failure_wait_seconds: Option<i32>,
        #[serde(
            rename = "maxTemporaryLockouts",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub max_temporary_lockouts: Option<i32>,
        #[serde(
            rename = "minimumQuickLoginWaitSeconds",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub minimum_quick_login_wait_seconds: Option<i32>,
        #[serde(rename = "notBefore", default, skip_serializing_if = "Option::is_none")]
        pub not_before: Option<i32>,
        #[serde(
            rename = "oAuth2DeviceCodeLifespan",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub o_auth2_device_code_lifespan: Option<i32>,
        #[serde(
            rename = "oAuth2DevicePollingInterval",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub o_auth2_device_polling_interval: Option<i32>,
        #[serde(
            rename = "oauth2DeviceCodeLifespan",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub oauth2_device_code_lifespan: Option<i32>,
        #[serde(
            rename = "oauth2DevicePollingInterval",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub oauth2_device_polling_interval: Option<i32>,
        #[serde(
            rename = "oauthClients",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub oauth_clients: Vec<OAuthClientRepresentation>,
        #[serde(
            rename = "offlineSessionIdleTimeout",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub offline_session_idle_timeout: Option<i32>,
        #[serde(
            rename = "offlineSessionMaxLifespan",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub offline_session_max_lifespan: Option<i32>,
        #[serde(
            rename = "offlineSessionMaxLifespanEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub offline_session_max_lifespan_enabled: Option<bool>,
        #[serde(
            rename = "otpPolicyAlgorithm",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub otp_policy_algorithm: Option<String>,
        #[serde(
            rename = "otpPolicyCodeReusable",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub otp_policy_code_reusable: Option<bool>,
        #[serde(
            rename = "otpPolicyDigits",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub otp_policy_digits: Option<i32>,
        #[serde(
            rename = "otpPolicyInitialCounter",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub otp_policy_initial_counter: Option<i32>,
        #[serde(
            rename = "otpPolicyLookAheadWindow",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub otp_policy_look_ahead_window: Option<i32>,
        #[serde(
            rename = "otpPolicyPeriod",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub otp_policy_period: Option<i32>,
        #[serde(
            rename = "otpPolicyType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub otp_policy_type: Option<String>,
        #[serde(
            rename = "otpSupportedApplications",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub otp_supported_applications: Vec<String>,
        #[serde(
            rename = "passwordCredentialGrantAllowed",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub password_credential_grant_allowed: Option<bool>,
        #[serde(
            rename = "passwordPolicy",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub password_policy: Option<String>,
        #[serde(
            rename = "permanentLockout",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub permanent_lockout: Option<bool>,
        #[serde(
            rename = "privateKey",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub private_key: Option<String>,
        #[serde(
            rename = "protocolMappers",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub protocol_mappers: Vec<ProtocolMapperRepresentation>,
        #[serde(rename = "publicKey", default, skip_serializing_if = "Option::is_none")]
        pub public_key: Option<String>,
        #[serde(
            rename = "quickLoginCheckMilliSeconds",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub quick_login_check_milli_seconds: Option<i64>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub realm: Option<String>,
        #[serde(
            rename = "realmCacheEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub realm_cache_enabled: Option<bool>,
        #[serde(
            rename = "refreshTokenMaxReuse",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub refresh_token_max_reuse: Option<i32>,
        #[serde(
            rename = "registrationAllowed",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub registration_allowed: Option<bool>,
        #[serde(
            rename = "registrationEmailAsUsername",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub registration_email_as_username: Option<bool>,
        #[serde(
            rename = "registrationFlow",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub registration_flow: Option<String>,
        #[serde(
            rename = "rememberMe",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub remember_me: Option<bool>,
        #[serde(
            rename = "requiredActions",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub required_actions: Vec<RequiredActionProviderRepresentation>,
        #[serde(
            rename = "requiredCredentials",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub required_credentials: Option<Vec<String>>,
        #[serde(
            rename = "resetCredentialsFlow",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub reset_credentials_flow: Option<String>,
        #[serde(
            rename = "resetPasswordAllowed",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub reset_password_allowed: Option<bool>,
        #[serde(
            rename = "revokeRefreshToken",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub revoke_refresh_token: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub roles: Option<RolesRepresentation>,
        #[serde(
            rename = "scopeMappings",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub scope_mappings: Vec<ScopeMappingRepresentation>,
        #[serde(
            rename = "smtpServer",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub smtp_server: std::collections::HashMap<String, String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub social: Option<bool>,
        #[serde(
            rename = "socialProviders",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub social_providers: std::collections::HashMap<String, String>,
        #[serde(
            rename = "sslRequired",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub ssl_required: Option<String>,
        #[serde(
            rename = "ssoSessionIdleTimeout",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sso_session_idle_timeout: Option<i32>,
        #[serde(
            rename = "ssoSessionIdleTimeoutRememberMe",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sso_session_idle_timeout_remember_me: Option<i32>,
        #[serde(
            rename = "ssoSessionMaxLifespan",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sso_session_max_lifespan: Option<i32>,
        #[serde(
            rename = "ssoSessionMaxLifespanRememberMe",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub sso_session_max_lifespan_remember_me: Option<i32>,
        #[serde(
            rename = "supportedLocales",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub supported_locales: Option<Vec<String>>,
        #[serde(
            rename = "updateProfileOnInitialSocialLogin",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub update_profile_on_initial_social_login: Option<bool>,
        #[serde(
            rename = "userCacheEnabled",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub user_cache_enabled: Option<bool>,
        #[serde(
            rename = "userFederationMappers",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub user_federation_mappers: Vec<UserFederationMapperRepresentation>,
        #[serde(
            rename = "userFederationProviders",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub user_federation_providers: Vec<UserFederationProviderRepresentation>,
        #[serde(
            rename = "userManagedAccessAllowed",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub user_managed_access_allowed: Option<bool>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub users: Vec<UserRepresentation>,
        #[serde(
            rename = "verifyEmail",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub verify_email: Option<bool>,
        #[serde(
            rename = "waitIncrementSeconds",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub wait_increment_seconds: Option<i32>,
        #[serde(
            rename = "webAuthnPolicyAcceptableAaguids",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub web_authn_policy_acceptable_aaguids: Vec<String>,
        #[serde(
            rename = "webAuthnPolicyAttestationConveyancePreference",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub web_authn_policy_attestation_conveyance_preference: Option<String>,
        #[serde(
            rename = "webAuthnPolicyAuthenticatorAttachment",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub web_authn_policy_authenticator_attachment: Option<String>,
        #[serde(
            rename = "webAuthnPolicyAvoidSameAuthenticatorRegister",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub web_authn_policy_avoid_same_authenticator_register: Option<bool>,
        #[serde(
            rename = "webAuthnPolicyCreateTimeout",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub web_authn_policy_create_timeout: Option<i32>,
        #[serde(
            rename = "webAuthnPolicyExtraOrigins",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub web_authn_policy_extra_origins: Vec<String>,
        #[serde(
            rename = "webAuthnPolicyPasswordlessAcceptableAaguids",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub web_authn_policy_passwordless_acceptable_aaguids: Vec<String>,
        #[serde(
            rename = "webAuthnPolicyPasswordlessAttestationConveyancePreference",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub web_authn_policy_passwordless_attestation_conveyance_preference: Option<String>,
        #[serde(
            rename = "webAuthnPolicyPasswordlessAuthenticatorAttachment",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub web_authn_policy_passwordless_authenticator_attachment: Option<String>,
        #[serde(
            rename = "webAuthnPolicyPasswordlessAvoidSameAuthenticatorRegister",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub web_authn_policy_passwordless_avoid_same_authenticator_register: Option<bool>,
        #[serde(
            rename = "webAuthnPolicyPasswordlessCreateTimeout",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub web_authn_policy_passwordless_create_timeout: Option<i32>,
        #[serde(
            rename = "webAuthnPolicyPasswordlessExtraOrigins",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub web_authn_policy_passwordless_extra_origins: Vec<String>,
        #[serde(
            rename = "webAuthnPolicyPasswordlessRequireResidentKey",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub web_authn_policy_passwordless_require_resident_key: Option<String>,
        #[serde(
            rename = "webAuthnPolicyPasswordlessRpEntityName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub web_authn_policy_passwordless_rp_entity_name: Option<String>,
        #[serde(
            rename = "webAuthnPolicyPasswordlessRpId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub web_authn_policy_passwordless_rp_id: Option<String>,
        #[serde(
            rename = "webAuthnPolicyPasswordlessSignatureAlgorithms",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub web_authn_policy_passwordless_signature_algorithms: Vec<String>,
        #[serde(
            rename = "webAuthnPolicyPasswordlessUserVerificationRequirement",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub web_authn_policy_passwordless_user_verification_requirement: Option<String>,
        #[serde(
            rename = "webAuthnPolicyRequireResidentKey",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub web_authn_policy_require_resident_key: Option<String>,
        #[serde(
            rename = "webAuthnPolicyRpEntityName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub web_authn_policy_rp_entity_name: Option<String>,
        #[serde(
            rename = "webAuthnPolicyRpId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub web_authn_policy_rp_id: Option<String>,
        #[serde(
            rename = "webAuthnPolicySignatureAlgorithms",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub web_authn_policy_signature_algorithms: Vec<String>,
        #[serde(
            rename = "webAuthnPolicyUserVerificationRequirement",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub web_authn_policy_user_verification_requirement: Option<String>,
    }

    impl From<&RealmRepresentation> for RealmRepresentation {
        fn from(value: &RealmRepresentation) -> Self {
            value.clone()
        }
    }

    ///RequiredActionProviderRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "alias": {
    ///      "type": "string"
    ///    },
    ///    "config": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "defaultAction": {
    ///      "type": "boolean"
    ///    },
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "priority": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "providerId": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct RequiredActionProviderRepresentation {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub alias: Option<String>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub config: std::collections::HashMap<String, String>,
        #[serde(
            rename = "defaultAction",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub default_action: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub priority: Option<i32>,
        #[serde(
            rename = "providerId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub provider_id: Option<String>,
    }

    impl From<&RequiredActionProviderRepresentation> for RequiredActionProviderRepresentation {
        fn from(value: &RequiredActionProviderRepresentation) -> Self {
            value.clone()
        }
    }

    ///ResourceOwnerRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ResourceOwnerRepresentation {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    impl From<&ResourceOwnerRepresentation> for ResourceOwnerRepresentation {
        fn from(value: &ResourceOwnerRepresentation) -> Self {
            value.clone()
        }
    }

    ///ResourceRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "_id": {
    ///      "type": "string"
    ///    },
    ///    "attributes": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "array",
    ///        "items": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "displayName": {
    ///      "type": "string"
    ///    },
    ///    "icon_uri": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "owner": {
    ///      "readOnly": true,
    ///      "type": "object",
    ///      "allOf": [
    ///        {
    ///          "$ref": "#/components/schemas/ResourceOwnerRepresentation"
    ///        }
    ///      ]
    ///    },
    ///    "ownerManagedAccess": {
    ///      "type": "boolean"
    ///    },
    ///    "scopes": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ScopeRepresentation"
    ///      },
    ///      "uniqueItems": true
    ///    },
    ///    "scopesUma": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ScopeRepresentation"
    ///      },
    ///      "uniqueItems": true
    ///    },
    ///    "type": {
    ///      "type": "string"
    ///    },
    ///    "uri": {
    ///      "deprecated": true,
    ///      "writeOnly": true,
    ///      "type": "string"
    ///    },
    ///    "uris": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      },
    ///      "uniqueItems": true
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ResourceRepresentation {
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub attributes: std::collections::HashMap<String, Vec<String>>,
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub display_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub icon_uri: Option<String>,
        #[serde(rename = "_id", default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub owner: Option<ResourceOwnerRepresentation>,
        #[serde(
            rename = "ownerManagedAccess",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub owner_managed_access: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub scopes: Option<Vec<ScopeRepresentation>>,
        #[serde(rename = "scopesUma", default, skip_serializing_if = "Option::is_none")]
        pub scopes_uma: Option<Vec<ScopeRepresentation>>,
        #[serde(rename = "type", default, skip_serializing_if = "Option::is_none")]
        pub type_: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub uri: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub uris: Option<Vec<String>>,
    }

    impl From<&ResourceRepresentation> for ResourceRepresentation {
        fn from(value: &ResourceRepresentation) -> Self {
            value.clone()
        }
    }

    ///ResourceServerRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "allowRemoteResourceManagement": {
    ///      "type": "boolean"
    ///    },
    ///    "clientId": {
    ///      "type": "string"
    ///    },
    ///    "decisionStrategy": {
    ///      "$ref": "#/components/schemas/DecisionStrategy"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "policies": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/PolicyRepresentation"
    ///      }
    ///    },
    ///    "policyEnforcementMode": {
    ///      "$ref": "#/components/schemas/PolicyEnforcementMode"
    ///    },
    ///    "resources": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ResourceRepresentation"
    ///      }
    ///    },
    ///    "scopes": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ScopeRepresentation"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ResourceServerRepresentation {
        #[serde(
            rename = "allowRemoteResourceManagement",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub allow_remote_resource_management: Option<bool>,
        #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
        pub client_id: Option<String>,
        #[serde(
            rename = "decisionStrategy",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub decision_strategy: Option<DecisionStrategy>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub policies: Vec<PolicyRepresentation>,
        #[serde(
            rename = "policyEnforcementMode",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub policy_enforcement_mode: Option<PolicyEnforcementMode>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub resources: Vec<ResourceRepresentation>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub scopes: Vec<ScopeRepresentation>,
    }

    impl From<&ResourceServerRepresentation> for ResourceServerRepresentation {
        fn from(value: &ResourceServerRepresentation) -> Self {
            value.clone()
        }
    }

    ///RoleRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "attributes": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "array",
    ///        "items": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "clientRole": {
    ///      "type": "boolean"
    ///    },
    ///    "composite": {
    ///      "type": "boolean"
    ///    },
    ///    "composites": {
    ///      "$ref": "#/components/schemas/Composites"
    ///    },
    ///    "containerId": {
    ///      "type": "string"
    ///    },
    ///    "description": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "scopeParamRequired": {
    ///      "deprecated": true,
    ///      "type": "boolean"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct RoleRepresentation {
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub attributes: std::collections::HashMap<String, Vec<String>>,
        #[serde(
            rename = "clientRole",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub client_role: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub composite: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub composites: Option<Composites>,
        #[serde(
            rename = "containerId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub container_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub description: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(
            rename = "scopeParamRequired",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub scope_param_required: Option<bool>,
    }

    impl From<&RoleRepresentation> for RoleRepresentation {
        fn from(value: &RoleRepresentation) -> Self {
            value.clone()
        }
    }

    ///RolesRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "application": {
    ///      "deprecated": true,
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "array",
    ///        "items": {
    ///          "$ref": "#/components/schemas/RoleRepresentation"
    ///        }
    ///      }
    ///    },
    ///    "client": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "array",
    ///        "items": {
    ///          "$ref": "#/components/schemas/RoleRepresentation"
    ///        }
    ///      }
    ///    },
    ///    "realm": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/RoleRepresentation"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct RolesRepresentation {
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub application: std::collections::HashMap<String, Vec<RoleRepresentation>>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub client: std::collections::HashMap<String, Vec<RoleRepresentation>>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub realm: Vec<RoleRepresentation>,
    }

    impl From<&RolesRepresentation> for RolesRepresentation {
        fn from(value: &RolesRepresentation) -> Self {
            value.clone()
        }
    }

    ///ScopeEnforcementMode
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "ALL",
    ///    "ANY",
    ///    "DISABLED"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum ScopeEnforcementMode {
        #[serde(rename = "ALL")]
        All,
        #[serde(rename = "ANY")]
        Any,
        #[serde(rename = "DISABLED")]
        Disabled,
    }

    impl From<&ScopeEnforcementMode> for ScopeEnforcementMode {
        fn from(value: &ScopeEnforcementMode) -> Self {
            value.clone()
        }
    }

    impl ToString for ScopeEnforcementMode {
        fn to_string(&self) -> String {
            match *self {
                Self::All => "ALL".to_string(),
                Self::Any => "ANY".to_string(),
                Self::Disabled => "DISABLED".to_string(),
            }
        }
    }

    impl std::str::FromStr for ScopeEnforcementMode {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "ALL" => Ok(Self::All),
                "ANY" => Ok(Self::Any),
                "DISABLED" => Ok(Self::Disabled),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for ScopeEnforcementMode {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for ScopeEnforcementMode {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for ScopeEnforcementMode {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///ScopeMappingRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "client": {
    ///      "type": "string"
    ///    },
    ///    "clientScope": {
    ///      "type": "string"
    ///    },
    ///    "clientTemplate": {
    ///      "deprecated": true,
    ///      "type": "string"
    ///    },
    ///    "roles": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      },
    ///      "uniqueItems": true
    ///    },
    ///    "self": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ScopeMappingRepresentation {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub client: Option<String>,
        #[serde(
            rename = "clientScope",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub client_scope: Option<String>,
        #[serde(
            rename = "clientTemplate",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub client_template: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub roles: Option<Vec<String>>,
        #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
        pub self_: Option<String>,
    }

    impl From<&ScopeMappingRepresentation> for ScopeMappingRepresentation {
        fn from(value: &ScopeMappingRepresentation) -> Self {
            value.clone()
        }
    }

    ///ScopeRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "displayName": {
    ///      "type": "string"
    ///    },
    ///    "iconUri": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "policies": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/PolicyRepresentation"
    ///      }
    ///    },
    ///    "resources": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/ResourceRepresentation"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct ScopeRepresentation {
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub display_name: Option<String>,
        #[serde(rename = "iconUri", default, skip_serializing_if = "Option::is_none")]
        pub icon_uri: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub policies: Vec<PolicyRepresentation>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub resources: Vec<ResourceRepresentation>,
    }

    impl From<&ScopeRepresentation> for ScopeRepresentation {
        fn from(value: &ScopeRepresentation) -> Self {
            value.clone()
        }
    }

    ///SocialLinkRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "socialProvider": {
    ///      "type": "string"
    ///    },
    ///    "socialUserId": {
    ///      "type": "string"
    ///    },
    ///    "socialUsername": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct SocialLinkRepresentation {
        #[serde(
            rename = "socialProvider",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub social_provider: Option<String>,
        #[serde(
            rename = "socialUserId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub social_user_id: Option<String>,
        #[serde(
            rename = "socialUsername",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub social_username: Option<String>,
    }

    impl From<&SocialLinkRepresentation> for SocialLinkRepresentation {
        fn from(value: &SocialLinkRepresentation) -> Self {
            value.clone()
        }
    }

    ///UnmanagedAttributePolicy
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "string",
    ///  "enum": [
    ///    "ENABLED",
    ///    "ADMIN_VIEW",
    ///    "ADMIN_EDIT"
    ///  ]
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Copy, Debug, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize)]
    pub enum UnmanagedAttributePolicy {
        #[serde(rename = "ENABLED")]
        Enabled,
        #[serde(rename = "ADMIN_VIEW")]
        AdminView,
        #[serde(rename = "ADMIN_EDIT")]
        AdminEdit,
    }

    impl From<&UnmanagedAttributePolicy> for UnmanagedAttributePolicy {
        fn from(value: &UnmanagedAttributePolicy) -> Self {
            value.clone()
        }
    }

    impl ToString for UnmanagedAttributePolicy {
        fn to_string(&self) -> String {
            match *self {
                Self::Enabled => "ENABLED".to_string(),
                Self::AdminView => "ADMIN_VIEW".to_string(),
                Self::AdminEdit => "ADMIN_EDIT".to_string(),
            }
        }
    }

    impl std::str::FromStr for UnmanagedAttributePolicy {
        type Err = self::error::ConversionError;
        fn from_str(value: &str) -> Result<Self, self::error::ConversionError> {
            match value {
                "ENABLED" => Ok(Self::Enabled),
                "ADMIN_VIEW" => Ok(Self::AdminView),
                "ADMIN_EDIT" => Ok(Self::AdminEdit),
                _ => Err("invalid value".into()),
            }
        }
    }

    impl std::convert::TryFrom<&str> for UnmanagedAttributePolicy {
        type Error = self::error::ConversionError;
        fn try_from(value: &str) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<&String> for UnmanagedAttributePolicy {
        type Error = self::error::ConversionError;
        fn try_from(value: &String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    impl std::convert::TryFrom<String> for UnmanagedAttributePolicy {
        type Error = self::error::ConversionError;
        fn try_from(value: String) -> Result<Self, self::error::ConversionError> {
            value.parse()
        }
    }

    ///UpAttribute
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "annotations": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "displayName": {
    ///      "type": "string"
    ///    },
    ///    "group": {
    ///      "type": "string"
    ///    },
    ///    "multivalued": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "permissions": {
    ///      "$ref": "#/components/schemas/UPAttributePermissions"
    ///    },
    ///    "required": {
    ///      "$ref": "#/components/schemas/UPAttributeRequired"
    ///    },
    ///    "selector": {
    ///      "$ref": "#/components/schemas/UPAttributeSelector"
    ///    },
    ///    "validations": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "object",
    ///        "additionalProperties": {}
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UpAttribute {
        #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
        pub annotations: serde_json::Map<String, serde_json::Value>,
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub display_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub group: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub multivalued: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub permissions: Option<UpAttributePermissions>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub required: Option<UpAttributeRequired>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub selector: Option<UpAttributeSelector>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub validations:
            std::collections::HashMap<String, serde_json::Map<String, serde_json::Value>>,
    }

    impl From<&UpAttribute> for UpAttribute {
        fn from(value: &UpAttribute) -> Self {
            value.clone()
        }
    }

    ///UpAttributePermissions
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "edit": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      },
    ///      "uniqueItems": true
    ///    },
    ///    "view": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      },
    ///      "uniqueItems": true
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UpAttributePermissions {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub edit: Option<Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub view: Option<Vec<String>>,
    }

    impl From<&UpAttributePermissions> for UpAttributePermissions {
        fn from(value: &UpAttributePermissions) -> Self {
            value.clone()
        }
    }

    ///UpAttributeRequired
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "roles": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      },
    ///      "uniqueItems": true
    ///    },
    ///    "scopes": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      },
    ///      "uniqueItems": true
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UpAttributeRequired {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub roles: Option<Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub scopes: Option<Vec<String>>,
    }

    impl From<&UpAttributeRequired> for UpAttributeRequired {
        fn from(value: &UpAttributeRequired) -> Self {
            value.clone()
        }
    }

    ///UpAttributeSelector
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "scopes": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      },
    ///      "uniqueItems": true
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UpAttributeSelector {
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub scopes: Option<Vec<String>>,
    }

    impl From<&UpAttributeSelector> for UpAttributeSelector {
        fn from(value: &UpAttributeSelector) -> Self {
            value.clone()
        }
    }

    ///UpConfig
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "attributes": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/UPAttribute"
    ///      }
    ///    },
    ///    "groups": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/UPGroup"
    ///      }
    ///    },
    ///    "unmanagedAttributePolicy": {
    ///      "$ref": "#/components/schemas/UnmanagedAttributePolicy"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UpConfig {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub attributes: Vec<UpAttribute>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub groups: Vec<UpGroup>,
        #[serde(
            rename = "unmanagedAttributePolicy",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub unmanaged_attribute_policy: Option<UnmanagedAttributePolicy>,
    }

    impl From<&UpConfig> for UpConfig {
        fn from(value: &UpConfig) -> Self {
            value.clone()
        }
    }

    ///UpGroup
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "annotations": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "displayDescription": {
    ///      "type": "string"
    ///    },
    ///    "displayHeader": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UpGroup {
        #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
        pub annotations: serde_json::Map<String, serde_json::Value>,
        #[serde(
            rename = "displayDescription",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub display_description: Option<String>,
        #[serde(
            rename = "displayHeader",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub display_header: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    impl From<&UpGroup> for UpGroup {
        fn from(value: &UpGroup) -> Self {
            value.clone()
        }
    }

    ///UserConsentRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "clientId": {
    ///      "type": "string"
    ///    },
    ///    "createdDate": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "grantedClientScopes": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "grantedRealmRoles": {
    ///      "deprecated": true,
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "lastUpdatedDate": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UserConsentRepresentation {
        #[serde(rename = "clientId", default, skip_serializing_if = "Option::is_none")]
        pub client_id: Option<String>,
        #[serde(
            rename = "createdDate",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub created_date: Option<i64>,
        #[serde(
            rename = "grantedClientScopes",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub granted_client_scopes: Vec<String>,
        #[serde(
            rename = "grantedRealmRoles",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub granted_realm_roles: Vec<String>,
        #[serde(
            rename = "lastUpdatedDate",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub last_updated_date: Option<i64>,
    }

    impl From<&UserConsentRepresentation> for UserConsentRepresentation {
        fn from(value: &UserConsentRepresentation) -> Self {
            value.clone()
        }
    }

    ///UserFederationMapperRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "config": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "federationMapperType": {
    ///      "type": "string"
    ///    },
    ///    "federationProviderDisplayName": {
    ///      "type": "string"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UserFederationMapperRepresentation {
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub config: std::collections::HashMap<String, String>,
        #[serde(
            rename = "federationMapperType",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub federation_mapper_type: Option<String>,
        #[serde(
            rename = "federationProviderDisplayName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub federation_provider_display_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    impl From<&UserFederationMapperRepresentation> for UserFederationMapperRepresentation {
        fn from(value: &UserFederationMapperRepresentation) -> Self {
            value.clone()
        }
    }

    ///UserFederationProviderRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "changedSyncPeriod": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "config": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "displayName": {
    ///      "type": "string"
    ///    },
    ///    "fullSyncPeriod": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "lastSync": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "priority": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "providerName": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UserFederationProviderRepresentation {
        #[serde(
            rename = "changedSyncPeriod",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub changed_sync_period: Option<i32>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub config: std::collections::HashMap<String, String>,
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub display_name: Option<String>,
        #[serde(
            rename = "fullSyncPeriod",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub full_sync_period: Option<i32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(rename = "lastSync", default, skip_serializing_if = "Option::is_none")]
        pub last_sync: Option<i32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub priority: Option<i32>,
        #[serde(
            rename = "providerName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub provider_name: Option<String>,
    }

    impl From<&UserFederationProviderRepresentation> for UserFederationProviderRepresentation {
        fn from(value: &UserFederationProviderRepresentation) -> Self {
            value.clone()
        }
    }

    ///UserManagedAccessConfig
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object"
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UserManagedAccessConfig(pub serde_json::Map<String, serde_json::Value>);
    impl std::ops::Deref for UserManagedAccessConfig {
        type Target = serde_json::Map<String, serde_json::Value>;
        fn deref(&self) -> &serde_json::Map<String, serde_json::Value> {
            &self.0
        }
    }

    impl From<UserManagedAccessConfig> for serde_json::Map<String, serde_json::Value> {
        fn from(value: UserManagedAccessConfig) -> Self {
            value.0
        }
    }

    impl From<&UserManagedAccessConfig> for UserManagedAccessConfig {
        fn from(value: &UserManagedAccessConfig) -> Self {
            value.clone()
        }
    }

    impl From<serde_json::Map<String, serde_json::Value>> for UserManagedAccessConfig {
        fn from(value: serde_json::Map<String, serde_json::Value>) -> Self {
            Self(value)
        }
    }

    ///UserProfileAttributeGroupMetadata
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "annotations": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "displayDescription": {
    ///      "type": "string"
    ///    },
    ///    "displayHeader": {
    ///      "type": "string"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UserProfileAttributeGroupMetadata {
        #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
        pub annotations: serde_json::Map<String, serde_json::Value>,
        #[serde(
            rename = "displayDescription",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub display_description: Option<String>,
        #[serde(
            rename = "displayHeader",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub display_header: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
    }

    impl From<&UserProfileAttributeGroupMetadata> for UserProfileAttributeGroupMetadata {
        fn from(value: &UserProfileAttributeGroupMetadata) -> Self {
            value.clone()
        }
    }

    ///UserProfileAttributeMetadata
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "annotations": {
    ///      "type": "object",
    ///      "additionalProperties": {}
    ///    },
    ///    "displayName": {
    ///      "type": "string"
    ///    },
    ///    "group": {
    ///      "type": "string"
    ///    },
    ///    "multivalued": {
    ///      "type": "boolean"
    ///    },
    ///    "name": {
    ///      "type": "string"
    ///    },
    ///    "readOnly": {
    ///      "type": "boolean"
    ///    },
    ///    "required": {
    ///      "type": "boolean"
    ///    },
    ///    "validators": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "object",
    ///        "additionalProperties": {}
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UserProfileAttributeMetadata {
        #[serde(default, skip_serializing_if = "serde_json::Map::is_empty")]
        pub annotations: serde_json::Map<String, serde_json::Value>,
        #[serde(
            rename = "displayName",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub display_name: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub group: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub multivalued: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub name: Option<String>,
        #[serde(rename = "readOnly", default, skip_serializing_if = "Option::is_none")]
        pub read_only: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub required: Option<bool>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub validators:
            std::collections::HashMap<String, serde_json::Map<String, serde_json::Value>>,
    }

    impl From<&UserProfileAttributeMetadata> for UserProfileAttributeMetadata {
        fn from(value: &UserProfileAttributeMetadata) -> Self {
            value.clone()
        }
    }

    ///UserProfileMetadata
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "attributes": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/UserProfileAttributeMetadata"
    ///      }
    ///    },
    ///    "groups": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/UserProfileAttributeGroupMetadata"
    ///      }
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UserProfileMetadata {
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub attributes: Vec<UserProfileAttributeMetadata>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub groups: Vec<UserProfileAttributeGroupMetadata>,
    }

    impl From<&UserProfileMetadata> for UserProfileMetadata {
        fn from(value: &UserProfileMetadata) -> Self {
            value.clone()
        }
    }

    ///UserRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "access": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "boolean"
    ///      }
    ///    },
    ///    "applicationRoles": {
    ///      "deprecated": true,
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "array",
    ///        "items": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "attributes": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "array",
    ///        "items": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "clientConsents": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/UserConsentRepresentation"
    ///      }
    ///    },
    ///    "clientRoles": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "array",
    ///        "items": {
    ///          "type": "string"
    ///        }
    ///      }
    ///    },
    ///    "createdTimestamp": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "credentials": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/CredentialRepresentation"
    ///      }
    ///    },
    ///    "disableableCredentialTypes": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      },
    ///      "uniqueItems": true
    ///    },
    ///    "email": {
    ///      "type": "string"
    ///    },
    ///    "emailVerified": {
    ///      "type": "boolean"
    ///    },
    ///    "enabled": {
    ///      "type": "boolean"
    ///    },
    ///    "federatedIdentities": {
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/FederatedIdentityRepresentation"
    ///      }
    ///    },
    ///    "federationLink": {
    ///      "type": "string"
    ///    },
    ///    "firstName": {
    ///      "type": "string"
    ///    },
    ///    "groups": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "lastName": {
    ///      "type": "string"
    ///    },
    ///    "notBefore": {
    ///      "type": "integer",
    ///      "format": "int32"
    ///    },
    ///    "origin": {
    ///      "type": "string"
    ///    },
    ///    "realmRoles": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "requiredActions": {
    ///      "type": "array",
    ///      "items": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "self": {
    ///      "type": "string"
    ///    },
    ///    "serviceAccountClientId": {
    ///      "type": "string"
    ///    },
    ///    "socialLinks": {
    ///      "deprecated": true,
    ///      "type": "array",
    ///      "items": {
    ///        "$ref": "#/components/schemas/SocialLinkRepresentation"
    ///      }
    ///    },
    ///    "totp": {
    ///      "type": "boolean"
    ///    },
    ///    "userProfileMetadata": {
    ///      "$ref": "#/components/schemas/UserProfileMetadata"
    ///    },
    ///    "username": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UserRepresentation {
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub access: std::collections::HashMap<String, bool>,
        #[serde(
            rename = "applicationRoles",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub application_roles: std::collections::HashMap<String, Vec<String>>,
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub attributes: std::collections::HashMap<String, Vec<String>>,
        #[serde(
            rename = "clientConsents",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub client_consents: Vec<UserConsentRepresentation>,
        #[serde(
            rename = "clientRoles",
            default,
            skip_serializing_if = "std::collections::HashMap::is_empty"
        )]
        pub client_roles: std::collections::HashMap<String, Vec<String>>,
        #[serde(
            rename = "createdTimestamp",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub created_timestamp: Option<i64>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub credentials: Vec<CredentialRepresentation>,
        #[serde(
            rename = "disableableCredentialTypes",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub disableable_credential_types: Option<Vec<String>>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub email: Option<String>,
        #[serde(
            rename = "emailVerified",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub email_verified: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub enabled: Option<bool>,
        #[serde(
            rename = "federatedIdentities",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub federated_identities: Vec<FederatedIdentityRepresentation>,
        #[serde(
            rename = "federationLink",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub federation_link: Option<String>,
        #[serde(rename = "firstName", default, skip_serializing_if = "Option::is_none")]
        pub first_name: Option<String>,
        #[serde(default, skip_serializing_if = "Vec::is_empty")]
        pub groups: Vec<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(rename = "lastName", default, skip_serializing_if = "Option::is_none")]
        pub last_name: Option<String>,
        #[serde(rename = "notBefore", default, skip_serializing_if = "Option::is_none")]
        pub not_before: Option<i32>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub origin: Option<String>,
        #[serde(rename = "realmRoles", default, skip_serializing_if = "Vec::is_empty")]
        pub realm_roles: Vec<String>,
        #[serde(
            rename = "requiredActions",
            default,
            skip_serializing_if = "Vec::is_empty"
        )]
        pub required_actions: Vec<String>,
        #[serde(rename = "self", default, skip_serializing_if = "Option::is_none")]
        pub self_: Option<String>,
        #[serde(
            rename = "serviceAccountClientId",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub service_account_client_id: Option<String>,
        #[serde(rename = "socialLinks", default, skip_serializing_if = "Vec::is_empty")]
        pub social_links: Vec<SocialLinkRepresentation>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub totp: Option<bool>,
        #[serde(
            rename = "userProfileMetadata",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub user_profile_metadata: Option<UserProfileMetadata>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub username: Option<String>,
    }

    impl From<&UserRepresentation> for UserRepresentation {
        fn from(value: &UserRepresentation) -> Self {
            value.clone()
        }
    }

    ///UserSessionRepresentation
    ///
    /// <details><summary>JSON schema</summary>
    ///
    /// ```json
    ///{
    ///  "type": "object",
    ///  "properties": {
    ///    "clients": {
    ///      "type": "object",
    ///      "additionalProperties": {
    ///        "type": "string"
    ///      }
    ///    },
    ///    "id": {
    ///      "type": "string"
    ///    },
    ///    "ipAddress": {
    ///      "type": "string"
    ///    },
    ///    "lastAccess": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "rememberMe": {
    ///      "type": "boolean"
    ///    },
    ///    "start": {
    ///      "type": "integer",
    ///      "format": "int64"
    ///    },
    ///    "userId": {
    ///      "type": "string"
    ///    },
    ///    "username": {
    ///      "type": "string"
    ///    }
    ///  }
    ///}
    /// ```
    /// </details>
    #[derive(Clone, Debug, Deserialize, Serialize)]
    pub struct UserSessionRepresentation {
        #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
        pub clients: std::collections::HashMap<String, String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub id: Option<String>,
        #[serde(rename = "ipAddress", default, skip_serializing_if = "Option::is_none")]
        pub ip_address: Option<String>,
        #[serde(
            rename = "lastAccess",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub last_access: Option<i64>,
        #[serde(
            rename = "rememberMe",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        pub remember_me: Option<bool>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub start: Option<i64>,
        #[serde(rename = "userId", default, skip_serializing_if = "Option::is_none")]
        pub user_id: Option<String>,
        #[serde(default, skip_serializing_if = "Option::is_none")]
        pub username: Option<String>,
    }

    impl From<&UserSessionRepresentation> for UserSessionRepresentation {
        fn from(value: &UserSessionRepresentation) -> Self {
            value.clone()
        }
    }
}

#[derive(Clone, Debug)]
///Client for Keycloak Admin REST API
///
///This is a REST API reference for the Keycloak Admin REST API.
///
///Version: 1.0
pub struct Client {
    pub(crate) baseurl: String,
    pub(crate) client: reqwest::Client,
}

impl Client {
    /// Create a new client.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new(baseurl: &str) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let client = {
            let dur = std::time::Duration::from_secs(15);
            reqwest::ClientBuilder::new()
                .connect_timeout(dur)
                .timeout(dur)
        };
        #[cfg(target_arch = "wasm32")]
        let client = reqwest::ClientBuilder::new();
        Self::new_with_client(baseurl, client.build().unwrap())
    }

    /// Construct a new client with an existing `reqwest::Client`,
    /// allowing more control over its configuration.
    ///
    /// `baseurl` is the base URL provided to the internal
    /// `reqwest::Client`, and should include a scheme and hostname,
    /// as well as port and a path stem if applicable.
    pub fn new_with_client(baseurl: &str, client: reqwest::Client) -> Self {
        Self {
            baseurl: baseurl.to_string(),
            client,
        }
    }

    /// Get the base URL to which requests are made.
    pub fn baseurl(&self) -> &String {
        &self.baseurl
    }

    /// Get the internal `reqwest::Client` used to make requests.
    pub fn client(&self) -> &reqwest::Client {
        &self.client
    }

    /// Get the version of this API.
    ///
    /// This string is pulled directly from the source OpenAPI
    /// document and may be in any format the API selects.
    pub fn api_version(&self) -> &'static str {
        "1.0"
    }
}

#[allow(clippy::all)]
impl Client {
    ///Get accessible realms Returns a list of accessible realms. The list is
    /// filtered based on what realms the caller is allowed to view
    ///
    ///Sends a `GET` request to `/admin/realms`
    pub async fn get_realms<'a>(
        &'a self,
        brief_representation: Option<bool>,
    ) -> Result<ResponseValue<Vec<types::RealmRepresentation>>, Error<()>> {
        let url = format!("{}/admin/realms", self.baseurl,);
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &brief_representation {
            query.push(("briefRepresentation", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Import a realm. Imports a realm from a full representation of that realm
    ///
    ///Realm name must be unique.
    ///
    ///Sends a `POST` request to `/admin/realms`
    pub async fn clear_realms_cache<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/admin/realms/{}/clear-realm-cache", self.baseurl, realm);
        #[allow(unused_mut)]
        let mut request = self.client.post(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status() {
            StatusCode::NO_CONTENT => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Import a realm. Imports a realm from a full representation of that realm
    ///
    ///Realm name must be unique.
    ///
    ///Sends a `POST` request to `/admin/realms`
    pub async fn create_realms<'a>(
        &'a self,
        body: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!("{}/admin/realms", self.baseurl,);
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            201u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get the top-level representation of the realm It will not include nested
    /// information like User and Client representations
    ///
    ///Sends a `GET` request to `/admin/realms/{realm}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    pub async fn get_admin_realms_by_realm<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<ResponseValue<types::RealmRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update the top-level information of the realm Any user, roles or client
    /// information in the representation will be ignored
    ///
    ///This will only update top-level attributes of the realm.
    ///
    ///Sends a `PUT` request to `/admin/realms/{realm}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `body`
    pub async fn update_admin_realms_by_realm<'a>(
        &'a self,
        realm: &'a str,
        body: &'a types::RealmRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete the realm
    ///
    ///Sends a `DELETE` request to `/admin/realms/{realm}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    pub async fn delete_admin_realms_by_realm<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get admin events Returns all admin events, or filters events based on
    /// URL query parameters listed here
    ///
    ///Sends a `GET` request to `/admin/realms/{realm}/admin-events`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `auth_client`
    /// - `auth_ip_address`
    /// - `auth_realm`
    /// - `auth_user`: user id
    /// - `date_from`
    /// - `date_to`
    /// - `first`
    /// - `max`: Maximum results size (defaults to 100)
    /// - `operation_types`
    /// - `resource_path`
    /// - `resource_types`
    pub async fn get_admin_events<'a>(
        &'a self,
        realm: &'a str,
        auth_client: Option<&'a str>,
        auth_ip_address: Option<&'a str>,
        auth_realm: Option<&'a str>,
        auth_user: Option<&'a str>,
        date_from: Option<&'a str>,
        date_to: Option<&'a str>,
        first: Option<i32>,
        max: Option<i32>,
        operation_types: Option<&'a Vec<String>>,
        resource_path: Option<&'a str>,
        resource_types: Option<&'a Vec<String>>,
    ) -> Result<ResponseValue<Vec<types::AdminEventRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/admin-events",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        let mut query = Vec::with_capacity(11usize);
        if let Some(v) = &auth_client {
            query.push(("authClient", v.to_string()));
        }

        if let Some(v) = &auth_ip_address {
            query.push(("authIpAddress", v.to_string()));
        }

        if let Some(v) = &auth_realm {
            query.push(("authRealm", v.to_string()));
        }

        if let Some(v) = &auth_user {
            query.push(("authUser", v.to_string()));
        }

        if let Some(v) = &date_from {
            query.push(("dateFrom", v.to_string()));
        }

        if let Some(v) = &date_to {
            query.push(("dateTo", v.to_string()));
        }

        if let Some(v) = &first {
            query.push(("first", v.to_string()));
        }

        if let Some(v) = &max {
            query.push(("max", v.to_string()));
        }

        if let Some(v) = &operation_types {
            query.push(("operationTypes", v.join(",").to_string()));
        }

        if let Some(v) = &resource_path {
            query.push(("resourcePath", v.to_string()));
        }

        if let Some(v) = &resource_types {
            query.push(("resourceTypes", v.join(",").to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete all admin events
    ///
    ///Sends a `DELETE` request to `/admin/realms/{realm}/admin-events`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    pub async fn delete_admin_events<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/admin-events",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Clear any user login failures for all users This can release temporary
    /// disabled users
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/attack-detection/brute-force/users`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    pub async fn delete_attack_detection_brute_force_users<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/attack-detection/brute-force/users",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get status of a username in brute force detection
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/attack-detection/brute-force/users/{userId}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    pub async fn get_attack_detection_brute_force_users_by_user_id<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
    ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/attack-detection/brute-force/users/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Clear any user login failures for the user This can release temporary
    /// disabled user
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/attack-detection/brute-force/users/{userId}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    pub async fn delete_attack_detection_brute_force_users_by_user_id<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/attack-detection/brute-force/users/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get authenticator providers Returns a stream of authenticator providers
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/authentication/authenticator-providers`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    pub async fn get_authentication_authenticator_providers<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<ResponseValue<Vec<serde_json::Map<String, serde_json::Value>>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/authenticator-providers",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get client authenticator providers Returns a stream of client
    /// authenticator providers
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/authentication/client-authenticator-providers`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    pub async fn get_authentication_client_authenticator_providers<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<ResponseValue<Vec<serde_json::Map<String, serde_json::Value>>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/client-authenticator-providers",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create new authenticator configuration
    ///
    ///Sends a `POST` request to `/admin/realms/{realm}/authentication/config`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `body`
    pub async fn create_authentication_config<'a>(
        &'a self,
        realm: &'a str,
        body: &'a types::AuthenticatorConfigRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/config",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get authenticator provider's configuration description
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/authentication/config-description/{providerId}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `provider_id`
    pub async fn get_authentication_config_description_by_provider_id<'a>(
        &'a self,
        realm: &'a str,
        provider_id: &'a str,
    ) -> Result<ResponseValue<types::AuthenticatorConfigInfoRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/config-description/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&provider_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get authenticator configuration
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/authentication/config/{id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `id`: Configuration id
    pub async fn get_authentication_config_by_id<'a>(
        &'a self,
        realm: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::AuthenticatorConfigRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/config/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update authenticator configuration
    ///
    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/authentication/config/{id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `id`: Configuration id
    /// - `body`
    pub async fn update_authentication_config_by_id<'a>(
        &'a self,
        realm: &'a str,
        id: &'a str,
        body: &'a types::AuthenticatorConfigRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/config/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete authenticator configuration
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/authentication/config/{id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `id`: Configuration id
    pub async fn delete_authentication_config_by_id<'a>(
        &'a self,
        realm: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/config/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Add new authentication execution
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/authentication/executions`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `body`
    pub async fn create_authentication_executions<'a>(
        &'a self,
        realm: &'a str,
        body: &'a types::AuthenticationExecutionRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/executions",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get Single Execution
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/authentication/executions/{executionId}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `execution_id`
    pub async fn get_authentication_executions_by_execution_id<'a>(
        &'a self,
        realm: &'a str,
        execution_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/executions/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&execution_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete execution
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/authentication/executions/{executionId}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `execution_id`: Execution id
    pub async fn delete_authentication_executions_by_execution_id<'a>(
        &'a self,
        realm: &'a str,
        execution_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/executions/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&execution_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update execution with new configuration
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/authentication/executions/{executionId}/config`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `execution_id`: Execution id
    /// - `body`
    pub async fn create_authentication_executions_by_execution_id_config<'a>(
        &'a self,
        realm: &'a str,
        execution_id: &'a str,
        body: &'a types::AuthenticatorConfigRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/executions/{}/config",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&execution_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get execution's configuration
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/authentication/executions/{executionId}/config/
    /// {id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `execution_id`: Execution id
    /// - `id`: Configuration id
    pub async fn get_authentication_executions_by_execution_id_config_by_id<'a>(
        &'a self,
        realm: &'a str,
        execution_id: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::AuthenticatorConfigRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/executions/{}/config/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&execution_id.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Lower execution's priority
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/authentication/executions/{executionId}/
    /// lower-priority`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `execution_id`: Execution id
    pub async fn create_authentication_executions_by_execution_id_lower_priority<'a>(
        &'a self,
        realm: &'a str,
        execution_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/executions/{}/lower-priority",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&execution_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Raise execution's priority
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/authentication/executions/{executionId}/
    /// raise-priority`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `execution_id`: Execution id
    pub async fn create_authentication_executions_by_execution_id_raise_priority<'a>(
        &'a self,
        realm: &'a str,
        execution_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/executions/{}/raise-priority",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&execution_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get authentication flows Returns a stream of authentication flows
    ///
    ///Sends a `GET` request to `/admin/realms/{realm}/authentication/flows`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    pub async fn get_authentication_flows<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<ResponseValue<Vec<types::AuthenticationFlowRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/flows",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a new authentication flow
    ///
    ///Sends a `POST` request to `/admin/realms/{realm}/authentication/flows`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `body`
    pub async fn create_authentication_flows<'a>(
        &'a self,
        realm: &'a str,
        body: &'a types::AuthenticationFlowRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/flows",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Copy existing authentication flow under a new name The new name is given
    /// as 'newName' attribute of the passed JSON object
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/authentication/flows/{flowAlias}/copy`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `flow_alias`: name of the existing authentication flow
    /// - `body`
    pub async fn create_authentication_flows_by_flow_alias_copy<'a>(
        &'a self,
        realm: &'a str,
        flow_alias: &'a str,
        body: &'a std::collections::HashMap<String, String>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/flows/{}/copy",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&flow_alias.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get authentication executions for a flow
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/authentication/flows/{flowAlias}/executions`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `flow_alias`: Flow alias
    pub async fn get_authentication_flows_by_flow_alias_executions<'a>(
        &'a self,
        realm: &'a str,
        flow_alias: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/flows/{}/executions",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&flow_alias.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update authentication executions of a Flow
    ///
    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/authentication/flows/{flowAlias}/executions`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `flow_alias`: Flow alias
    /// - `body`
    pub async fn update_authentication_flows_by_flow_alias_executions<'a>(
        &'a self,
        realm: &'a str,
        flow_alias: &'a str,
        body: &'a types::AuthenticationExecutionInfoRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/flows/{}/executions",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&flow_alias.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Add new authentication execution to a flow
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/authentication/flows/{flowAlias}/executions/
    /// execution`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `flow_alias`: Alias of parent flow
    /// - `body`
    pub async fn create_authentication_flows_by_flow_alias_executions_execution<'a>(
        &'a self,
        realm: &'a str,
        flow_alias: &'a str,
        body: &'a std::collections::HashMap<String, String>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/flows/{}/executions/execution",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&flow_alias.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Add new flow with new execution to existing flow
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/authentication/flows/{flowAlias}/executions/flow`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `flow_alias`: Alias of parent authentication flow
    /// - `body`
    pub async fn create_authentication_flows_by_flow_alias_executions_flow<'a>(
        &'a self,
        realm: &'a str,
        flow_alias: &'a str,
        body: &'a std::collections::HashMap<String, String>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/flows/{}/executions/flow",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&flow_alias.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get authentication flow for id
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/authentication/flows/{id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `id`: Flow id
    pub async fn get_authentication_flows_by_id<'a>(
        &'a self,
        realm: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::AuthenticationFlowRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/flows/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update an authentication flow
    ///
    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/authentication/flows/{id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `body`
    pub async fn update_authentication_flows_by_id<'a>(
        &'a self,
        realm: &'a str,
        id: &'a str,
        body: &'a types::AuthenticationFlowRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/flows/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete an authentication flow
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/authentication/flows/{id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `id`: Flow id
    pub async fn delete_authentication_flows_by_id<'a>(
        &'a self,
        realm: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/flows/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get form action providers Returns a stream of form action providers
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/authentication/form-action-providers`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    pub async fn get_authentication_form_action_providers<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<ResponseValue<Vec<serde_json::Map<String, serde_json::Value>>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/form-action-providers",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get form providers Returns a stream of form providers
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/authentication/form-providers`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    pub async fn get_authentication_form_providers<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<ResponseValue<Vec<serde_json::Map<String, serde_json::Value>>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/form-providers",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get configuration descriptions for all clients
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/authentication/per-client-config-description`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    pub async fn get_authentication_per_client_config_description<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<
        ResponseValue<std::collections::HashMap<String, Vec<types::ConfigPropertyRepresentation>>>,
        Error<()>,
    > {
        let url = format!(
            "{}/admin/realms/{}/authentication/per-client-config-description",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Register a new required actions
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/authentication/register-required-action`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `body`
    pub async fn create_authentication_register_required_action<'a>(
        &'a self,
        realm: &'a str,
        body: &'a std::collections::HashMap<String, String>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/register-required-action",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get required actions Returns a stream of required actions
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/authentication/required-actions`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    pub async fn get_authentication_required_actions<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<ResponseValue<Vec<types::RequiredActionProviderRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/required-actions",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get required action for alias
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/authentication/required-actions/{alias}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `alias`: Alias of required action
    pub async fn get_authentication_required_actions_by_alias<'a>(
        &'a self,
        realm: &'a str,
        alias: &'a str,
    ) -> Result<ResponseValue<types::RequiredActionProviderRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/required-actions/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&alias.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update required action
    ///
    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/authentication/required-actions/{alias}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `alias`: Alias of required action
    /// - `body`
    pub async fn update_authentication_required_actions_by_alias<'a>(
        &'a self,
        realm: &'a str,
        alias: &'a str,
        body: &'a types::RequiredActionProviderRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/required-actions/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&alias.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete required action
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/authentication/required-actions/{alias}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `alias`: Alias of required action
    pub async fn delete_authentication_required_actions_by_alias<'a>(
        &'a self,
        realm: &'a str,
        alias: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/required-actions/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&alias.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Lower required action's priority
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/authentication/required-actions/{alias}/
    /// lower-priority`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `alias`: Alias of required action
    pub async fn create_authentication_required_actions_by_alias_lower_priority<'a>(
        &'a self,
        realm: &'a str,
        alias: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/required-actions/{}/lower-priority",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&alias.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Raise required action's priority
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/authentication/required-actions/{alias}/
    /// raise-priority`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `alias`: Alias of required action
    pub async fn create_authentication_required_actions_by_alias_raise_priority<'a>(
        &'a self,
        realm: &'a str,
        alias: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/required-actions/{}/raise-priority",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&alias.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get unregistered required actions Returns a stream of unregistered
    /// required actions
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/authentication/unregistered-required-actions`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    pub async fn get_authentication_unregistered_required_actions<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<ResponseValue<Vec<std::collections::HashMap<String, String>>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/authentication/unregistered-required-actions",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Base path for importing clients under this realm
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/client-description-converter`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `body`
    pub async fn create_client_description_converter<'a>(
        &'a self,
        realm: &'a str,
        body: &'a str,
    ) -> Result<ResponseValue<types::ClientRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-description-converter",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/client-policies/policies`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    pub async fn get_client_policies_policies<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<ResponseValue<types::ClientPoliciesRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-policies/policies",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/client-policies/policies`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `body`
    pub async fn update_client_policies_policies<'a>(
        &'a self,
        realm: &'a str,
        body: &'a types::ClientPoliciesRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-policies/policies",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/client-policies/profiles`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `include_global_profiles`
    pub async fn get_client_policies_profiles<'a>(
        &'a self,
        realm: &'a str,
        include_global_profiles: Option<bool>,
    ) -> Result<ResponseValue<types::ClientProfilesRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-policies/profiles",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &include_global_profiles {
            query.push(("include-global-profiles", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/client-policies/profiles`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `body`
    pub async fn update_client_policies_profiles<'a>(
        &'a self,
        realm: &'a str,
        body: &'a types::ClientProfilesRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-policies/profiles",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Base path for retrieve providers with the configProperties properly
    /// filled
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/client-registration-policy/providers`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    pub async fn get_client_registration_policy_providers<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<ResponseValue<Vec<types::ComponentTypeRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-registration-policy/providers",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get client scopes belonging to the realm Returns a list of client scopes
    /// belonging to the realm
    ///
    ///Sends a `GET` request to `/admin/realms/{realm}/client-scopes`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    pub async fn get_client_scopes<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<ResponseValue<Vec<types::ClientScopeRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-scopes",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a new client scope Client Scope’s name must be unique!
    ///
    ///Sends a `POST` request to `/admin/realms/{realm}/client-scopes`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `body`
    pub async fn create_client_scopes<'a>(
        &'a self,
        realm: &'a str,
        body: &'a types::ClientScopeRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-scopes",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get representation of the client scope
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/client-scopes/{client-scope-id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    pub async fn get_client_scopes_by_client_scope_id<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
    ) -> Result<ResponseValue<types::ClientScopeRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-scopes/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update the client scope
    ///
    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/client-scopes/{client-scope-id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `body`
    pub async fn update_client_scopes_by_client_scope_id<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
        body: &'a types::ClientScopeRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-scopes/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete the client scope
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/client-scopes/{client-scope-id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    pub async fn delete_client_scopes_by_client_scope_id<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-scopes/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create multiple mappers
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/client-scopes/{client-scope-id}/protocol-mappers/
    /// add-models`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `body`
    pub async fn create_client_scopes_by_client_scope_id_protocol_mappers_add_models<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
        body: &'a Vec<types::ProtocolMapperRepresentation>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-scopes/{}/protocol-mappers/add-models",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get mappers
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/client-scopes/{client-scope-id}/protocol-mappers/
    /// models`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    pub async fn get_client_scopes_by_client_scope_id_protocol_mappers_models<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
    ) -> Result<ResponseValue<Vec<types::ProtocolMapperRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-scopes/{}/protocol-mappers/models",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a mapper
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/client-scopes/{client-scope-id}/protocol-mappers/
    /// models`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `body`
    pub async fn create_client_scopes_by_client_scope_id_protocol_mappers_models<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
        body: &'a types::ProtocolMapperRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-scopes/{}/protocol-mappers/models",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get mapper by id
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/client-scopes/{client-scope-id}/protocol-mappers/
    /// models/{id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `id`: Mapper id
    pub async fn get_client_scopes_by_client_scope_id_protocol_mappers_models_by_id<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::ProtocolMapperRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-scopes/{}/protocol-mappers/models/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update the mapper
    ///
    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/client-scopes/{client-scope-id}/protocol-mappers/
    /// models/{id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `id`: Mapper id
    /// - `body`
    pub async fn update_client_scopes_by_client_scope_id_protocol_mappers_models_by_id<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
        id: &'a str,
        body: &'a types::ProtocolMapperRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-scopes/{}/protocol-mappers/models/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete the mapper
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/client-scopes/{client-scope-id}/protocol-mappers/
    /// models/{id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `id`: Mapper id
    pub async fn delete_client_scopes_by_client_scope_id_protocol_mappers_models_by_id<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-scopes/{}/protocol-mappers/models/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get mappers by name for a specific protocol
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/client-scopes/{client-scope-id}/protocol-mappers/
    /// protocol/{protocol}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `protocol`
    pub async fn get_client_scopes_by_client_scope_id_protocol_mappers_protocol_by_protocol<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
        protocol: &'a str,
    ) -> Result<ResponseValue<Vec<types::ProtocolMapperRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-scopes/{}/protocol-mappers/protocol/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
            encode_path(&protocol.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get all scope mappings for the client
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    pub async fn get_client_scopes_by_client_scope_id_scope_mappings<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
    ) -> Result<ResponseValue<types::MappingsRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-scopes/{}/scope-mappings",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get the roles associated with a client's scope Returns roles for the
    /// client
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings/
    /// clients/{client}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `client`
    pub async fn get_client_scopes_by_client_scope_id_scope_mappings_clients_by_client<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
        client: &'a str,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-scopes/{}/scope-mappings/clients/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
            encode_path(&client.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Add client-level roles to the client's scope
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings/
    /// clients/{client}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `client`
    /// - `body`
    pub async fn create_client_scopes_by_client_scope_id_scope_mappings_clients_by_client<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
        client: &'a str,
        body: &'a Vec<types::RoleRepresentation>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-scopes/{}/scope-mappings/clients/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
            encode_path(&client.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Remove client-level roles from the client's scope
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings/
    /// clients/{client}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `client`
    /// - `body`
    pub async fn delete_client_scopes_by_client_scope_id_scope_mappings_clients_by_client<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
        client: &'a str,
        body: &'a Vec<types::RoleRepresentation>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-scopes/{}/scope-mappings/clients/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
            encode_path(&client.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///The available client-level roles Returns the roles for the client that
    /// can be associated with the client's scope
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings/
    /// clients/{client}/available`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `client`
    pub async fn get_client_scopes_by_client_scope_id_scope_mappings_clients_by_client_available<
        'a,
    >(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
        client: &'a str,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-scopes/{}/scope-mappings/clients/{}/available",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
            encode_path(&client.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get effective client roles Returns the roles for the client that are
    /// associated with the client's scope
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings/
    /// clients/{client}/composite`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `client`
    /// - `brief_representation`: if false, return roles with their attributes
    pub async fn get_client_scopes_by_client_scope_id_scope_mappings_clients_by_client_composite<
        'a,
    >(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
        client: &'a str,
        brief_representation: Option<bool>,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-scopes/{}/scope-mappings/clients/{}/composite",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
            encode_path(&client.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &brief_representation {
            query.push(("briefRepresentation", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get realm-level roles associated with the client's scope
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings/
    /// realm`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    pub async fn get_client_scopes_by_client_scope_id_scope_mappings_realm<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-scopes/{}/scope-mappings/realm",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Add a set of realm-level roles to the client's scope
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings/
    /// realm`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `body`
    pub async fn create_client_scopes_by_client_scope_id_scope_mappings_realm<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
        body: &'a Vec<types::RoleRepresentation>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-scopes/{}/scope-mappings/realm",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Remove a set of realm-level roles from the client's scope
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings/
    /// realm`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `body`
    pub async fn delete_client_scopes_by_client_scope_id_scope_mappings_realm<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
        body: &'a Vec<types::RoleRepresentation>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-scopes/{}/scope-mappings/realm",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get realm-level roles that are available to attach to this client's
    /// scope
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings/
    /// realm/available`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    pub async fn get_client_scopes_by_client_scope_id_scope_mappings_realm_available<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-scopes/{}/scope-mappings/realm/available",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get effective realm-level roles associated with the client’s scope What
    /// this does is recurse any composite roles associated with the client’s
    /// scope and adds the roles to this lists
    ///
    ///The method is really to show a comprehensive total view of realm-level
    /// roles associated with the client.
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/client-scopes/{client-scope-id}/scope-mappings/
    /// realm/composite`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `brief_representation`: if false, return roles with their attributes
    pub async fn get_client_scopes_by_client_scope_id_scope_mappings_realm_composite<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
        brief_representation: Option<bool>,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-scopes/{}/scope-mappings/realm/composite",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &brief_representation {
            query.push(("briefRepresentation", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get client session stats Returns a JSON map
    ///
    ///The key is the client id, the value is the number of sessions that
    /// currently are active with that client. Only clients that actually have a
    /// session associated with them will be in this map.
    ///
    ///Sends a `GET` request to `/admin/realms/{realm}/client-session-stats`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    pub async fn get_client_session_stats<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<ResponseValue<Vec<std::collections::HashMap<String, String>>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-session-stats",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get client scopes belonging to the realm Returns a list of client scopes
    /// belonging to the realm
    ///
    ///Sends a `GET` request to `/admin/realms/{realm}/client-templates`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    pub async fn get_client_templates<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<ResponseValue<Vec<types::ClientScopeRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-templates",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a new client scope Client Scope’s name must be unique!
    ///
    ///Sends a `POST` request to `/admin/realms/{realm}/client-templates`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `body`
    pub async fn create_client_templates<'a>(
        &'a self,
        realm: &'a str,
        body: &'a types::ClientScopeRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-templates",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get representation of the client scope
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/client-templates/{client-scope-id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    pub async fn get_client_templates_by_client_scope_id<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
    ) -> Result<ResponseValue<types::ClientScopeRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-templates/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update the client scope
    ///
    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/client-templates/{client-scope-id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `body`
    pub async fn update_client_templates_by_client_scope_id<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
        body: &'a types::ClientScopeRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-templates/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete the client scope
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/client-templates/{client-scope-id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    pub async fn delete_client_templates_by_client_scope_id<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-templates/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create multiple mappers
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/client-templates/{client-scope-id}/
    /// protocol-mappers/add-models`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `body`
    pub async fn create_client_templates_by_client_scope_id_protocol_mappers_add_models<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
        body: &'a Vec<types::ProtocolMapperRepresentation>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-templates/{}/protocol-mappers/add-models",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get mappers
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/client-templates/{client-scope-id}/
    /// protocol-mappers/models`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    pub async fn get_client_templates_by_client_scope_id_protocol_mappers_models<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
    ) -> Result<ResponseValue<Vec<types::ProtocolMapperRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-templates/{}/protocol-mappers/models",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a mapper
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/client-templates/{client-scope-id}/
    /// protocol-mappers/models`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `body`
    pub async fn create_client_templates_by_client_scope_id_protocol_mappers_models<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
        body: &'a types::ProtocolMapperRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-templates/{}/protocol-mappers/models",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get mapper by id
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/client-templates/{client-scope-id}/
    /// protocol-mappers/models/{id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `id`: Mapper id
    pub async fn get_client_templates_by_client_scope_id_protocol_mappers_models_by_id<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::ProtocolMapperRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-templates/{}/protocol-mappers/models/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update the mapper
    ///
    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/client-templates/{client-scope-id}/
    /// protocol-mappers/models/{id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `id`: Mapper id
    /// - `body`
    pub async fn update_client_templates_by_client_scope_id_protocol_mappers_models_by_id<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
        id: &'a str,
        body: &'a types::ProtocolMapperRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-templates/{}/protocol-mappers/models/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete the mapper
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/client-templates/{client-scope-id}/
    /// protocol-mappers/models/{id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `id`: Mapper id
    pub async fn delete_client_templates_by_client_scope_id_protocol_mappers_models_by_id<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-templates/{}/protocol-mappers/models/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get mappers by name for a specific protocol
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/client-templates/{client-scope-id}/
    /// protocol-mappers/protocol/{protocol}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `protocol`
    pub async fn get_client_templates_by_client_scope_id_protocol_mappers_protocol_by_protocol<
        'a,
    >(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
        protocol: &'a str,
    ) -> Result<ResponseValue<Vec<types::ProtocolMapperRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-templates/{}/protocol-mappers/protocol/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
            encode_path(&protocol.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get all scope mappings for the client
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/client-templates/{client-scope-id}/
    /// scope-mappings`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    pub async fn get_client_templates_by_client_scope_id_scope_mappings<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
    ) -> Result<ResponseValue<types::MappingsRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-templates/{}/scope-mappings",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get the roles associated with a client's scope Returns roles for the
    /// client
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/client-templates/{client-scope-id}/
    /// scope-mappings/clients/{client}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `client`
    pub async fn get_client_templates_by_client_scope_id_scope_mappings_clients_by_client<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
        client: &'a str,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-templates/{}/scope-mappings/clients/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
            encode_path(&client.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Add client-level roles to the client's scope
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/client-templates/{client-scope-id}/
    /// scope-mappings/clients/{client}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `client`
    /// - `body`
    pub async fn create_client_templates_by_client_scope_id_scope_mappings_clients_by_client<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
        client: &'a str,
        body: &'a Vec<types::RoleRepresentation>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-templates/{}/scope-mappings/clients/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
            encode_path(&client.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Remove client-level roles from the client's scope
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/client-templates/{client-scope-id}/
    /// scope-mappings/clients/{client}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `client`
    /// - `body`
    pub async fn delete_client_templates_by_client_scope_id_scope_mappings_clients_by_client<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
        client: &'a str,
        body: &'a Vec<types::RoleRepresentation>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-templates/{}/scope-mappings/clients/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
            encode_path(&client.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///The available client-level roles Returns the roles for the client that
    /// can be associated with the client's scope
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/client-templates/{client-scope-id}/
    /// scope-mappings/clients/{client}/available`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `client`
    pub async fn get_client_templates_by_client_scope_id_scope_mappings_clients_by_client_available<
        'a,
    >(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
        client: &'a str,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-templates/{}/scope-mappings/clients/{}/available",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
            encode_path(&client.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get effective client roles Returns the roles for the client that are
    /// associated with the client's scope
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/client-templates/{client-scope-id}/
    /// scope-mappings/clients/{client}/composite`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `client`
    /// - `brief_representation`: if false, return roles with their attributes
    pub async fn get_client_templates_by_client_scope_id_scope_mappings_clients_by_client_composite<
        'a,
    >(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
        client: &'a str,
        brief_representation: Option<bool>,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-templates/{}/scope-mappings/clients/{}/composite",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
            encode_path(&client.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &brief_representation {
            query.push(("briefRepresentation", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get realm-level roles associated with the client's scope
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/client-templates/{client-scope-id}/
    /// scope-mappings/realm`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    pub async fn get_client_templates_by_client_scope_id_scope_mappings_realm<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-templates/{}/scope-mappings/realm",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Add a set of realm-level roles to the client's scope
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/client-templates/{client-scope-id}/
    /// scope-mappings/realm`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `body`
    pub async fn create_client_templates_by_client_scope_id_scope_mappings_realm<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
        body: &'a Vec<types::RoleRepresentation>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-templates/{}/scope-mappings/realm",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Remove a set of realm-level roles from the client's scope
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/client-templates/{client-scope-id}/
    /// scope-mappings/realm`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `body`
    pub async fn delete_client_templates_by_client_scope_id_scope_mappings_realm<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
        body: &'a Vec<types::RoleRepresentation>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-templates/{}/scope-mappings/realm",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get realm-level roles that are available to attach to this client's
    /// scope
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/client-templates/{client-scope-id}/
    /// scope-mappings/realm/available`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    pub async fn get_client_templates_by_client_scope_id_scope_mappings_realm_available<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-templates/{}/scope-mappings/realm/available",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get effective realm-level roles associated with the client’s scope What
    /// this does is recurse any composite roles associated with the client’s
    /// scope and adds the roles to this lists
    ///
    ///The method is really to show a comprehensive total view of realm-level
    /// roles associated with the client.
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/client-templates/{client-scope-id}/
    /// scope-mappings/realm/composite`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    /// - `brief_representation`: if false, return roles with their attributes
    pub async fn get_client_templates_by_client_scope_id_scope_mappings_realm_composite<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
        brief_representation: Option<bool>,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/client-templates/{}/scope-mappings/realm/composite",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &brief_representation {
            query.push(("briefRepresentation", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get clients belonging to the realm
    ///
    ///If a client can’t be retrieved from the storage due to a problem with
    /// the underlying storage, it is silently removed from the returned list.
    /// This ensures that concurrent modifications to the list don’t prevent
    /// callers from retrieving this list.
    ///
    ///Sends a `GET` request to `/admin/realms/{realm}/clients`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_id`: filter by clientId
    /// - `first`: the first result
    /// - `max`: the max results to return
    /// - `q`
    /// - `search`: whether this is a search query or a getClientById query
    /// - `viewable_only`: filter clients that cannot be viewed in full by admin
    pub async fn get_clients<'a>(
        &'a self,
        realm: &'a str,
        client_id: Option<&'a str>,
        first: Option<i32>,
        max: Option<i32>,
        q: Option<&'a str>,
        search: Option<bool>,
        viewable_only: Option<bool>,
    ) -> Result<ResponseValue<Vec<types::ClientRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        let mut query = Vec::with_capacity(6usize);
        if let Some(v) = &client_id {
            query.push(("clientId", v.to_string()));
        }

        if let Some(v) = &first {
            query.push(("first", v.to_string()));
        }

        if let Some(v) = &max {
            query.push(("max", v.to_string()));
        }

        if let Some(v) = &q {
            query.push(("q", v.to_string()));
        }

        if let Some(v) = &search {
            query.push(("search", v.to_string()));
        }

        if let Some(v) = &viewable_only {
            query.push(("viewableOnly", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a new client Client’s client_id must be unique!
    ///
    ///Sends a `POST` request to `/admin/realms/{realm}/clients`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `body`
    pub async fn create_clients<'a>(
        &'a self,
        realm: &'a str,
        body: &'a types::ClientRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            201u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/admin/realms/{realm}/clients-initial-access`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    pub async fn get_clients_initial_access<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<ResponseValue<Vec<types::ClientInitialAccessPresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients-initial-access",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a new initial access token
    ///
    ///Sends a `POST` request to `/admin/realms/{realm}/clients-initial-access`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `body`
    pub async fn create_clients_initial_access<'a>(
        &'a self,
        realm: &'a str,
        body: &'a types::ClientInitialAccessCreatePresentation,
    ) -> Result<ResponseValue<types::ClientInitialAccessCreatePresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients-initial-access",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/clients-initial-access/{id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `id`
    pub async fn delete_clients_initial_access_by_id<'a>(
        &'a self,
        realm: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients-initial-access/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get representation of the client
    ///
    ///Sends a `GET` request to `/admin/realms/{realm}/clients/{client-uuid}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    pub async fn get_clients_by_client_uuid<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
    ) -> Result<ResponseValue<types::ClientRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update the client
    ///
    ///Sends a `PUT` request to `/admin/realms/{realm}/clients/{client-uuid}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    pub async fn update_clients_by_client_uuid<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        body: &'a types::ClientRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete the client
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    pub async fn delete_clients_by_client_uuid<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    pub async fn get_clients_by_client_uuid_authz_resource_server<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
    ) -> Result<ResponseValue<types::ResourceServerRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/authz/resource-server",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    pub async fn update_clients_by_client_uuid_authz_resource_server<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        body: &'a types::ResourceServerRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/authz/resource-server",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/
    /// import`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    pub async fn create_clients_by_client_uuid_authz_resource_server_import<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        body: &'a types::ResourceServerRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/authz/resource-server/import",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/
    /// permission`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `fields`
    /// - `first`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `permission`
    /// - `policy_id`
    /// - `resource`
    /// - `scope`
    /// - `type_`
    pub async fn get_clients_by_client_uuid_authz_resource_server_permission<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        fields: Option<&'a str>,
        first: Option<i32>,
        max: Option<i32>,
        name: Option<&'a str>,
        owner: Option<&'a str>,
        permission: Option<bool>,
        policy_id: Option<&'a str>,
        resource: Option<&'a str>,
        scope: Option<&'a str>,
        type_: Option<&'a str>,
    ) -> Result<ResponseValue<Vec<types::AbstractPolicyRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/authz/resource-server/permission",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        let mut query = Vec::with_capacity(10usize);
        if let Some(v) = &fields {
            query.push(("fields", v.to_string()));
        }

        if let Some(v) = &first {
            query.push(("first", v.to_string()));
        }

        if let Some(v) = &max {
            query.push(("max", v.to_string()));
        }

        if let Some(v) = &name {
            query.push(("name", v.to_string()));
        }

        if let Some(v) = &owner {
            query.push(("owner", v.to_string()));
        }

        if let Some(v) = &permission {
            query.push(("permission", v.to_string()));
        }

        if let Some(v) = &policy_id {
            query.push(("policyId", v.to_string()));
        }

        if let Some(v) = &resource {
            query.push(("resource", v.to_string()));
        }

        if let Some(v) = &scope {
            query.push(("scope", v.to_string()));
        }

        if let Some(v) = &type_ {
            query.push(("type", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/
    /// permission`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    pub async fn create_clients_by_client_uuid_authz_resource_server_permission<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        body: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/authz/resource-server/permission",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            201u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/
    /// permission/evaluate`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    pub async fn create_clients_by_client_uuid_authz_resource_server_permission_evaluate<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        body: &'a types::PolicyEvaluationRequest,
    ) -> Result<ResponseValue<types::PolicyEvaluationResponse>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/authz/resource-server/permission/evaluate",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/
    /// permission/providers`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    pub async fn get_clients_by_client_uuid_authz_resource_server_permission_providers<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
    ) -> Result<ResponseValue<Vec<types::PolicyProviderRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/authz/resource-server/permission/providers",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/
    /// permission/search`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `fields`
    /// - `name`
    pub async fn get_clients_by_client_uuid_authz_resource_server_permission_search<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        fields: Option<&'a str>,
        name: Option<&'a str>,
    ) -> Result<ResponseValue<types::AbstractPolicyRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/authz/resource-server/permission/search",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        let mut query = Vec::with_capacity(2usize);
        if let Some(v) = &fields {
            query.push(("fields", v.to_string()));
        }

        if let Some(v) = &name {
            query.push(("name", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/
    /// policy`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `fields`
    /// - `first`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `permission`
    /// - `policy_id`
    /// - `resource`
    /// - `scope`
    /// - `type_`
    pub async fn get_clients_by_client_uuid_authz_resource_server_policy<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        fields: Option<&'a str>,
        first: Option<i32>,
        max: Option<i32>,
        name: Option<&'a str>,
        owner: Option<&'a str>,
        permission: Option<bool>,
        policy_id: Option<&'a str>,
        resource: Option<&'a str>,
        scope: Option<&'a str>,
        type_: Option<&'a str>,
    ) -> Result<ResponseValue<Vec<types::AbstractPolicyRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/authz/resource-server/policy",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        let mut query = Vec::with_capacity(10usize);
        if let Some(v) = &fields {
            query.push(("fields", v.to_string()));
        }

        if let Some(v) = &first {
            query.push(("first", v.to_string()));
        }

        if let Some(v) = &max {
            query.push(("max", v.to_string()));
        }

        if let Some(v) = &name {
            query.push(("name", v.to_string()));
        }

        if let Some(v) = &owner {
            query.push(("owner", v.to_string()));
        }

        if let Some(v) = &permission {
            query.push(("permission", v.to_string()));
        }

        if let Some(v) = &policy_id {
            query.push(("policyId", v.to_string()));
        }

        if let Some(v) = &resource {
            query.push(("resource", v.to_string()));
        }

        if let Some(v) = &scope {
            query.push(("scope", v.to_string()));
        }

        if let Some(v) = &type_ {
            query.push(("type", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/
    /// policy`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    pub async fn create_clients_by_client_uuid_authz_resource_server_policy<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        body: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/authz/resource-server/policy",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            201u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/
    /// policy/evaluate`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    pub async fn create_clients_by_client_uuid_authz_resource_server_policy_evaluate<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        body: &'a types::PolicyEvaluationRequest,
    ) -> Result<ResponseValue<types::PolicyEvaluationResponse>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/authz/resource-server/policy/evaluate",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            500u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/
    /// policy/providers`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    pub async fn get_clients_by_client_uuid_authz_resource_server_policy_providers<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
    ) -> Result<ResponseValue<Vec<types::PolicyProviderRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/authz/resource-server/policy/providers",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/
    /// policy/search`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `fields`
    /// - `name`
    pub async fn get_clients_by_client_uuid_authz_resource_server_policy_search<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        fields: Option<&'a str>,
        name: Option<&'a str>,
    ) -> Result<ResponseValue<types::AbstractPolicyRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/authz/resource-server/policy/search",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        let mut query = Vec::with_capacity(2usize);
        if let Some(v) = &fields {
            query.push(("fields", v.to_string()));
        }

        if let Some(v) = &name {
            query.push(("name", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/
    /// resource`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `id`
    /// - `deep`
    /// - `exact_name`
    /// - `first`
    /// - `matching_uri`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `scope`
    /// - `type_`
    /// - `uri`
    pub async fn get_clients_by_client_uuid_authz_resource_server_resource<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        id: Option<&'a str>,
        deep: Option<bool>,
        exact_name: Option<bool>,
        first: Option<i32>,
        matching_uri: Option<bool>,
        max: Option<i32>,
        name: Option<&'a str>,
        owner: Option<&'a str>,
        scope: Option<&'a str>,
        type_: Option<&'a str>,
        uri: Option<&'a str>,
    ) -> Result<ResponseValue<Vec<types::ResourceRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/authz/resource-server/resource",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        let mut query = Vec::with_capacity(11usize);
        if let Some(v) = &id {
            query.push(("_id", v.to_string()));
        }

        if let Some(v) = &deep {
            query.push(("deep", v.to_string()));
        }

        if let Some(v) = &exact_name {
            query.push(("exactName", v.to_string()));
        }

        if let Some(v) = &first {
            query.push(("first", v.to_string()));
        }

        if let Some(v) = &matching_uri {
            query.push(("matchingUri", v.to_string()));
        }

        if let Some(v) = &max {
            query.push(("max", v.to_string()));
        }

        if let Some(v) = &name {
            query.push(("name", v.to_string()));
        }

        if let Some(v) = &owner {
            query.push(("owner", v.to_string()));
        }

        if let Some(v) = &scope {
            query.push(("scope", v.to_string()));
        }

        if let Some(v) = &type_ {
            query.push(("type", v.to_string()));
        }

        if let Some(v) = &uri {
            query.push(("uri", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/
    /// resource`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `id`
    /// - `deep`
    /// - `exact_name`
    /// - `first`
    /// - `matching_uri`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `scope`
    /// - `type_`
    /// - `uri`
    /// - `body`
    pub async fn create_clients_by_client_uuid_authz_resource_server_resource<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        id: Option<&'a str>,
        deep: Option<bool>,
        exact_name: Option<bool>,
        first: Option<i32>,
        matching_uri: Option<bool>,
        max: Option<i32>,
        name: Option<&'a str>,
        owner: Option<&'a str>,
        scope: Option<&'a str>,
        type_: Option<&'a str>,
        uri: Option<&'a str>,
        body: &'a types::ResourceRepresentation,
    ) -> Result<ResponseValue<types::ResourceRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/authz/resource-server/resource",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        let mut query = Vec::with_capacity(11usize);
        if let Some(v) = &id {
            query.push(("_id", v.to_string()));
        }

        if let Some(v) = &deep {
            query.push(("deep", v.to_string()));
        }

        if let Some(v) = &exact_name {
            query.push(("exactName", v.to_string()));
        }

        if let Some(v) = &first {
            query.push(("first", v.to_string()));
        }

        if let Some(v) = &matching_uri {
            query.push(("matchingUri", v.to_string()));
        }

        if let Some(v) = &max {
            query.push(("max", v.to_string()));
        }

        if let Some(v) = &name {
            query.push(("name", v.to_string()));
        }

        if let Some(v) = &owner {
            query.push(("owner", v.to_string()));
        }

        if let Some(v) = &scope {
            query.push(("scope", v.to_string()));
        }

        if let Some(v) = &type_ {
            query.push(("type", v.to_string()));
        }

        if let Some(v) = &uri {
            query.push(("uri", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            201u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/
    /// resource/search`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `id`
    /// - `deep`
    /// - `exact_name`
    /// - `first`
    /// - `matching_uri`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `scope`
    /// - `type_`
    /// - `uri`
    pub async fn get_clients_by_client_uuid_authz_resource_server_resource_search<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        id: Option<&'a str>,
        deep: Option<bool>,
        exact_name: Option<bool>,
        first: Option<i32>,
        matching_uri: Option<bool>,
        max: Option<i32>,
        name: Option<&'a str>,
        owner: Option<&'a str>,
        scope: Option<&'a str>,
        type_: Option<&'a str>,
        uri: Option<&'a str>,
    ) -> Result<ResponseValue<types::ResourceRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/authz/resource-server/resource/search",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        let mut query = Vec::with_capacity(11usize);
        if let Some(v) = &id {
            query.push(("_id", v.to_string()));
        }

        if let Some(v) = &deep {
            query.push(("deep", v.to_string()));
        }

        if let Some(v) = &exact_name {
            query.push(("exactName", v.to_string()));
        }

        if let Some(v) = &first {
            query.push(("first", v.to_string()));
        }

        if let Some(v) = &matching_uri {
            query.push(("matchingUri", v.to_string()));
        }

        if let Some(v) = &max {
            query.push(("max", v.to_string()));
        }

        if let Some(v) = &name {
            query.push(("name", v.to_string()));
        }

        if let Some(v) = &owner {
            query.push(("owner", v.to_string()));
        }

        if let Some(v) = &scope {
            query.push(("scope", v.to_string()));
        }

        if let Some(v) = &type_ {
            query.push(("type", v.to_string()));
        }

        if let Some(v) = &uri {
            query.push(("uri", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/
    /// resource/{resource-id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `resource_id`
    /// - `id`
    /// - `deep`
    /// - `exact_name`
    /// - `first`
    /// - `matching_uri`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `scope`
    /// - `type_`
    /// - `uri`
    pub async fn get_clients_by_client_uuid_authz_resource_server_resource_by_resource_id<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        resource_id: &'a str,
        id: Option<&'a str>,
        deep: Option<bool>,
        exact_name: Option<bool>,
        first: Option<i32>,
        matching_uri: Option<bool>,
        max: Option<i32>,
        name: Option<&'a str>,
        owner: Option<&'a str>,
        scope: Option<&'a str>,
        type_: Option<&'a str>,
        uri: Option<&'a str>,
    ) -> Result<ResponseValue<types::ResourceRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/authz/resource-server/resource/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&resource_id.to_string()),
        );
        let mut query = Vec::with_capacity(11usize);
        if let Some(v) = &id {
            query.push(("_id", v.to_string()));
        }

        if let Some(v) = &deep {
            query.push(("deep", v.to_string()));
        }

        if let Some(v) = &exact_name {
            query.push(("exactName", v.to_string()));
        }

        if let Some(v) = &first {
            query.push(("first", v.to_string()));
        }

        if let Some(v) = &matching_uri {
            query.push(("matchingUri", v.to_string()));
        }

        if let Some(v) = &max {
            query.push(("max", v.to_string()));
        }

        if let Some(v) = &name {
            query.push(("name", v.to_string()));
        }

        if let Some(v) = &owner {
            query.push(("owner", v.to_string()));
        }

        if let Some(v) = &scope {
            query.push(("scope", v.to_string()));
        }

        if let Some(v) = &type_ {
            query.push(("type", v.to_string()));
        }

        if let Some(v) = &uri {
            query.push(("uri", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/
    /// resource/{resource-id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `resource_id`
    /// - `id`
    /// - `deep`
    /// - `exact_name`
    /// - `first`
    /// - `matching_uri`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `scope`
    /// - `type_`
    /// - `uri`
    /// - `body`
    pub async fn update_clients_by_client_uuid_authz_resource_server_resource_by_resource_id<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        resource_id: &'a str,
        id: Option<&'a str>,
        deep: Option<bool>,
        exact_name: Option<bool>,
        first: Option<i32>,
        matching_uri: Option<bool>,
        max: Option<i32>,
        name: Option<&'a str>,
        owner: Option<&'a str>,
        scope: Option<&'a str>,
        type_: Option<&'a str>,
        uri: Option<&'a str>,
        body: &'a types::ResourceRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/authz/resource-server/resource/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&resource_id.to_string()),
        );
        let mut query = Vec::with_capacity(11usize);
        if let Some(v) = &id {
            query.push(("_id", v.to_string()));
        }

        if let Some(v) = &deep {
            query.push(("deep", v.to_string()));
        }

        if let Some(v) = &exact_name {
            query.push(("exactName", v.to_string()));
        }

        if let Some(v) = &first {
            query.push(("first", v.to_string()));
        }

        if let Some(v) = &matching_uri {
            query.push(("matchingUri", v.to_string()));
        }

        if let Some(v) = &max {
            query.push(("max", v.to_string()));
        }

        if let Some(v) = &name {
            query.push(("name", v.to_string()));
        }

        if let Some(v) = &owner {
            query.push(("owner", v.to_string()));
        }

        if let Some(v) = &scope {
            query.push(("scope", v.to_string()));
        }

        if let Some(v) = &type_ {
            query.push(("type", v.to_string()));
        }

        if let Some(v) = &uri {
            query.push(("uri", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/
    /// resource/{resource-id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `resource_id`
    /// - `id`
    /// - `deep`
    /// - `exact_name`
    /// - `first`
    /// - `matching_uri`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `scope`
    /// - `type_`
    /// - `uri`
    pub async fn delete_clients_by_client_uuid_authz_resource_server_resource_by_resource_id<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        resource_id: &'a str,
        id: Option<&'a str>,
        deep: Option<bool>,
        exact_name: Option<bool>,
        first: Option<i32>,
        matching_uri: Option<bool>,
        max: Option<i32>,
        name: Option<&'a str>,
        owner: Option<&'a str>,
        scope: Option<&'a str>,
        type_: Option<&'a str>,
        uri: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/authz/resource-server/resource/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&resource_id.to_string()),
        );
        let mut query = Vec::with_capacity(11usize);
        if let Some(v) = &id {
            query.push(("_id", v.to_string()));
        }

        if let Some(v) = &deep {
            query.push(("deep", v.to_string()));
        }

        if let Some(v) = &exact_name {
            query.push(("exactName", v.to_string()));
        }

        if let Some(v) = &first {
            query.push(("first", v.to_string()));
        }

        if let Some(v) = &matching_uri {
            query.push(("matchingUri", v.to_string()));
        }

        if let Some(v) = &max {
            query.push(("max", v.to_string()));
        }

        if let Some(v) = &name {
            query.push(("name", v.to_string()));
        }

        if let Some(v) = &owner {
            query.push(("owner", v.to_string()));
        }

        if let Some(v) = &scope {
            query.push(("scope", v.to_string()));
        }

        if let Some(v) = &type_ {
            query.push(("type", v.to_string()));
        }

        if let Some(v) = &uri {
            query.push(("uri", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self.client.delete(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/
    /// resource/{resource-id}/attributes`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `resource_id`
    /// - `id`
    /// - `deep`
    /// - `exact_name`
    /// - `first`
    /// - `matching_uri`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `scope`
    /// - `type_`
    /// - `uri`
    pub async fn get_clients_by_client_uuid_authz_resource_server_resource_by_resource_id_attributes<
        'a,
    >(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        resource_id: &'a str,
        id: Option<&'a str>,
        deep: Option<bool>,
        exact_name: Option<bool>,
        first: Option<i32>,
        matching_uri: Option<bool>,
        max: Option<i32>,
        name: Option<&'a str>,
        owner: Option<&'a str>,
        scope: Option<&'a str>,
        type_: Option<&'a str>,
        uri: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/authz/resource-server/resource/{}/attributes",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&resource_id.to_string()),
        );
        let mut query = Vec::with_capacity(11usize);
        if let Some(v) = &id {
            query.push(("_id", v.to_string()));
        }

        if let Some(v) = &deep {
            query.push(("deep", v.to_string()));
        }

        if let Some(v) = &exact_name {
            query.push(("exactName", v.to_string()));
        }

        if let Some(v) = &first {
            query.push(("first", v.to_string()));
        }

        if let Some(v) = &matching_uri {
            query.push(("matchingUri", v.to_string()));
        }

        if let Some(v) = &max {
            query.push(("max", v.to_string()));
        }

        if let Some(v) = &name {
            query.push(("name", v.to_string()));
        }

        if let Some(v) = &owner {
            query.push(("owner", v.to_string()));
        }

        if let Some(v) = &scope {
            query.push(("scope", v.to_string()));
        }

        if let Some(v) = &type_ {
            query.push(("type", v.to_string()));
        }

        if let Some(v) = &uri {
            query.push(("uri", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/
    /// resource/{resource-id}/permissions`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `resource_id`
    /// - `id`
    /// - `deep`
    /// - `exact_name`
    /// - `first`
    /// - `matching_uri`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `scope`
    /// - `type_`
    /// - `uri`
    pub async fn get_clients_by_client_uuid_authz_resource_server_resource_by_resource_id_permissions<
        'a,
    >(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        resource_id: &'a str,
        id: Option<&'a str>,
        deep: Option<bool>,
        exact_name: Option<bool>,
        first: Option<i32>,
        matching_uri: Option<bool>,
        max: Option<i32>,
        name: Option<&'a str>,
        owner: Option<&'a str>,
        scope: Option<&'a str>,
        type_: Option<&'a str>,
        uri: Option<&'a str>,
    ) -> Result<ResponseValue<Vec<types::PolicyRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/authz/resource-server/resource/{}/permissions",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&resource_id.to_string()),
        );
        let mut query = Vec::with_capacity(11usize);
        if let Some(v) = &id {
            query.push(("_id", v.to_string()));
        }

        if let Some(v) = &deep {
            query.push(("deep", v.to_string()));
        }

        if let Some(v) = &exact_name {
            query.push(("exactName", v.to_string()));
        }

        if let Some(v) = &first {
            query.push(("first", v.to_string()));
        }

        if let Some(v) = &matching_uri {
            query.push(("matchingUri", v.to_string()));
        }

        if let Some(v) = &max {
            query.push(("max", v.to_string()));
        }

        if let Some(v) = &name {
            query.push(("name", v.to_string()));
        }

        if let Some(v) = &owner {
            query.push(("owner", v.to_string()));
        }

        if let Some(v) = &scope {
            query.push(("scope", v.to_string()));
        }

        if let Some(v) = &type_ {
            query.push(("type", v.to_string()));
        }

        if let Some(v) = &uri {
            query.push(("uri", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/
    /// resource/{resource-id}/scopes`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `resource_id`
    /// - `id`
    /// - `deep`
    /// - `exact_name`
    /// - `first`
    /// - `matching_uri`
    /// - `max`
    /// - `name`
    /// - `owner`
    /// - `scope`
    /// - `type_`
    /// - `uri`
    pub async fn get_clients_by_client_uuid_authz_resource_server_resource_by_resource_id_scopes<
        'a,
    >(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        resource_id: &'a str,
        id: Option<&'a str>,
        deep: Option<bool>,
        exact_name: Option<bool>,
        first: Option<i32>,
        matching_uri: Option<bool>,
        max: Option<i32>,
        name: Option<&'a str>,
        owner: Option<&'a str>,
        scope: Option<&'a str>,
        type_: Option<&'a str>,
        uri: Option<&'a str>,
    ) -> Result<ResponseValue<Vec<types::ScopeRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/authz/resource-server/resource/{}/scopes",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&resource_id.to_string()),
        );
        let mut query = Vec::with_capacity(11usize);
        if let Some(v) = &id {
            query.push(("_id", v.to_string()));
        }

        if let Some(v) = &deep {
            query.push(("deep", v.to_string()));
        }

        if let Some(v) = &exact_name {
            query.push(("exactName", v.to_string()));
        }

        if let Some(v) = &first {
            query.push(("first", v.to_string()));
        }

        if let Some(v) = &matching_uri {
            query.push(("matchingUri", v.to_string()));
        }

        if let Some(v) = &max {
            query.push(("max", v.to_string()));
        }

        if let Some(v) = &name {
            query.push(("name", v.to_string()));
        }

        if let Some(v) = &owner {
            query.push(("owner", v.to_string()));
        }

        if let Some(v) = &scope {
            query.push(("scope", v.to_string()));
        }

        if let Some(v) = &type_ {
            query.push(("type", v.to_string()));
        }

        if let Some(v) = &uri {
            query.push(("uri", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/
    /// scope`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `first`
    /// - `max`
    /// - `name`
    /// - `scope_id`
    pub async fn get_clients_by_client_uuid_authz_resource_server_scope<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        first: Option<i32>,
        max: Option<i32>,
        name: Option<&'a str>,
        scope_id: Option<&'a str>,
    ) -> Result<ResponseValue<Vec<types::ScopeRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/authz/resource-server/scope",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        let mut query = Vec::with_capacity(4usize);
        if let Some(v) = &first {
            query.push(("first", v.to_string()));
        }

        if let Some(v) = &max {
            query.push(("max", v.to_string()));
        }

        if let Some(v) = &name {
            query.push(("name", v.to_string()));
        }

        if let Some(v) = &scope_id {
            query.push(("scopeId", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/
    /// scope`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    pub async fn create_clients_by_client_uuid_authz_resource_server_scope<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        body: &'a types::ScopeRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/authz/resource-server/scope",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/
    /// scope/search`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `name`
    pub async fn get_clients_by_client_uuid_authz_resource_server_scope_search<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        name: Option<&'a str>,
    ) -> Result<ResponseValue<Vec<types::ScopeRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/authz/resource-server/scope/search",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &name {
            query.push(("name", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            400u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/
    /// scope/{scope-id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `scope_id`
    pub async fn get_clients_by_client_uuid_authz_resource_server_scope_by_scope_id<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        scope_id: &'a str,
    ) -> Result<ResponseValue<types::ScopeRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/authz/resource-server/scope/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/
    /// scope/{scope-id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `scope_id`
    /// - `body`
    pub async fn update_clients_by_client_uuid_authz_resource_server_scope_by_scope_id<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        scope_id: &'a str,
        body: &'a types::ScopeRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/authz/resource-server/scope/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/
    /// scope/{scope-id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `scope_id`
    pub async fn delete_clients_by_client_uuid_authz_resource_server_scope_by_scope_id<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        scope_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/authz/resource-server/scope/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/
    /// scope/{scope-id}/permissions`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `scope_id`
    pub async fn get_clients_by_client_uuid_authz_resource_server_scope_by_scope_id_permissions<
        'a,
    >(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        scope_id: &'a str,
    ) -> Result<ResponseValue<Vec<types::PolicyRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/authz/resource-server/scope/{}/permissions",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/
    /// scope/{scope-id}/resources`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `scope_id`
    pub async fn get_clients_by_client_uuid_authz_resource_server_scope_by_scope_id_resources<
        'a,
    >(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        scope_id: &'a str,
    ) -> Result<ResponseValue<Vec<types::ResourceRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/authz/resource-server/scope/{}/resources",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            404u16 => Err(Error::ErrorResponse(ResponseValue::empty(response))),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/authz/resource-server/
    /// settings`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    pub async fn get_clients_by_client_uuid_authz_resource_server_settings<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
    ) -> Result<ResponseValue<types::ResourceServerRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/authz/resource-server/settings",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get key info
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/certificates/{attr}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `attr`
    pub async fn get_clients_by_client_uuid_certificates_by_attr<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        attr: &'a str,
    ) -> Result<ResponseValue<types::CertificateRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/certificates/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&attr.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get a keystore file for the client, containing private key and public
    /// certificate
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/certificates/{attr}/
    /// download`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `attr`
    /// - `body`
    pub async fn create_clients_by_client_uuid_certificates_by_attr_download<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        attr: &'a str,
        body: &'a types::KeyStoreConfig,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/certificates/{}/download",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&attr.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Generate a new certificate with new key pair
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/certificates/{attr}/
    /// generate`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `attr`
    pub async fn create_clients_by_client_uuid_certificates_by_attr_generate<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        attr: &'a str,
    ) -> Result<ResponseValue<types::CertificateRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/certificates/{}/generate",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&attr.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Generate a new keypair and certificate, and get the private key file
    ///
    ///Generates a keypair and certificate and serves the private key in a
    /// specified keystore format. Only generated public certificate is
    /// saved in Keycloak DB - the private key is not
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/certificates/{attr}/
    /// generate-and-download`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `attr`
    /// - `body`
    pub async fn create_clients_by_client_uuid_certificates_by_attr_generate_and_download<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        attr: &'a str,
        body: &'a types::KeyStoreConfig,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/certificates/{}/generate-and-download",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&attr.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Upload certificate and eventually private key
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/certificates/{attr}/upload`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `attr`
    pub async fn create_clients_by_client_uuid_certificates_by_attr_upload<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        attr: &'a str,
    ) -> Result<ResponseValue<types::CertificateRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/certificates/{}/upload",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&attr.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Upload only certificate, not private key
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/certificates/{attr}/
    /// upload-certificate`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `attr`
    pub async fn create_clients_by_client_uuid_certificates_by_attr_upload_certificate<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        attr: &'a str,
    ) -> Result<ResponseValue<types::CertificateRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/certificates/{}/upload-certificate",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&attr.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get the client secret
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/client-secret`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    pub async fn get_clients_by_client_uuid_client_secret<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
    ) -> Result<ResponseValue<types::CredentialRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/client-secret",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Generate a new secret for the client
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/client-secret`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    pub async fn create_clients_by_client_uuid_client_secret<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
    ) -> Result<ResponseValue<types::CredentialRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/client-secret",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get the rotated client secret
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/client-secret/rotated`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    pub async fn get_clients_by_client_uuid_client_secret_rotated<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
    ) -> Result<ResponseValue<types::CredentialRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/client-secret/rotated",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Invalidate the rotated secret for the client
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/client-secret/rotated`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    pub async fn delete_clients_by_client_uuid_client_secret_rotated<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/client-secret/rotated",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get default client scopes.  Only name and ids are returned
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/default-client-scopes`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    pub async fn get_clients_by_client_uuid_default_client_scopes<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
    ) -> Result<ResponseValue<Vec<types::ClientScopeRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/default-client-scopes",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/default-client-scopes/
    /// {clientScopeId}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `client_scope_id`
    pub async fn update_clients_by_client_uuid_default_client_scopes_by_client_scope_id<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        client_scope_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/default-client-scopes/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/default-client-scopes/
    /// {clientScopeId}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `client_scope_id`
    pub async fn delete_clients_by_client_uuid_default_client_scopes_by_client_scope_id<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        client_scope_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/default-client-scopes/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create JSON with payload of example access token
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/evaluate-scopes/
    /// generate-example-access-token`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `scope`
    /// - `user_id`
    pub async fn get_clients_by_client_uuid_evaluate_scopes_generate_example_access_token<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        scope: Option<&'a str>,
        user_id: Option<&'a str>,
    ) -> Result<ResponseValue<types::AccessToken>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/evaluate-scopes/generate-example-access-token",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        let mut query = Vec::with_capacity(2usize);
        if let Some(v) = &scope {
            query.push(("scope", v.to_string()));
        }

        if let Some(v) = &user_id {
            query.push(("userId", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create JSON with payload of example id token
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/evaluate-scopes/
    /// generate-example-id-token`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `scope`
    /// - `user_id`
    pub async fn get_clients_by_client_uuid_evaluate_scopes_generate_example_id_token<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        scope: Option<&'a str>,
        user_id: Option<&'a str>,
    ) -> Result<ResponseValue<types::IdToken>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/evaluate-scopes/generate-example-id-token",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        let mut query = Vec::with_capacity(2usize);
        if let Some(v) = &scope {
            query.push(("scope", v.to_string()));
        }

        if let Some(v) = &user_id {
            query.push(("userId", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create JSON with payload of example user info
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/evaluate-scopes/
    /// generate-example-userinfo`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `scope`
    /// - `user_id`
    pub async fn get_clients_by_client_uuid_evaluate_scopes_generate_example_userinfo<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        scope: Option<&'a str>,
        user_id: Option<&'a str>,
    ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/evaluate-scopes/generate-example-userinfo",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        let mut query = Vec::with_capacity(2usize);
        if let Some(v) = &scope {
            query.push(("scope", v.to_string()));
        }

        if let Some(v) = &user_id {
            query.push(("userId", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Return list of all protocol mappers, which will be used when generating
    /// tokens issued for particular client
    ///
    ///This means protocol mappers assigned to this client directly and
    /// protocol mappers assigned to all client scopes of this client.
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/evaluate-scopes/
    /// protocol-mappers`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `scope`
    pub async fn get_clients_by_client_uuid_evaluate_scopes_protocol_mappers<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        scope: Option<&'a str>,
    ) -> Result<ResponseValue<Vec<types::ProtocolMapperEvaluationRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/evaluate-scopes/protocol-mappers",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &scope {
            query.push(("scope", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get effective scope mapping of all roles of particular role container,
    /// which this client is defacto allowed to have in the accessToken issued
    /// for him
    ///
    ///This contains scope mappings, which this client has directly, as well as
    /// scope mappings, which are granted to all client scopes, which are linked
    /// with this client.
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/evaluate-scopes/
    /// scope-mappings/{roleContainerId}/granted`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_container_id`: either realm name OR client UUID
    /// - `scope`
    pub async fn get_clients_by_client_uuid_evaluate_scopes_scope_mappings_by_role_container_id_granted<
        'a,
    >(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        role_container_id: &'a str,
        scope: Option<&'a str>,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/evaluate-scopes/scope-mappings/{}/granted",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&role_container_id.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &scope {
            query.push(("scope", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get roles, which this client doesn't have scope for and can't have them
    /// in the accessToken issued for him
    ///
    ///Defacto all the other roles of particular role container, which are not
    /// in {@link #getGrantedScopeMappings()}
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/evaluate-scopes/
    /// scope-mappings/{roleContainerId}/not-granted`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_container_id`: either realm name OR client UUID
    /// - `scope`
    pub async fn get_clients_by_client_uuid_evaluate_scopes_scope_mappings_by_role_container_id_not_granted<
        'a,
    >(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        role_container_id: &'a str,
        scope: Option<&'a str>,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/evaluate-scopes/scope-mappings/{}/not-granted",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&role_container_id.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &scope {
            query.push(("scope", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/installation/providers/
    /// {providerId}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `provider_id`
    pub async fn get_clients_by_client_uuid_installation_providers_by_provider_id<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        provider_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/installation/providers/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&provider_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Return object stating whether client Authorization permissions have been
    /// initialized or not and a reference
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/management/permissions`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    pub async fn get_clients_by_client_uuid_management_permissions<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
    ) -> Result<ResponseValue<types::ManagementPermissionReference>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/management/permissions",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Return object stating whether client Authorization permissions have been
    /// initialized or not and a reference
    ///
    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/management/permissions`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    pub async fn update_clients_by_client_uuid_management_permissions<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        body: &'a types::ManagementPermissionReference,
    ) -> Result<ResponseValue<types::ManagementPermissionReference>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/management/permissions",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Register a cluster node with the client Manually register cluster node
    /// to this client - usually it’s not needed to call this directly as
    /// adapter should handle by sending registration request to Keycloak
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/nodes`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    pub async fn create_clients_by_client_uuid_nodes<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        body: &'a std::collections::HashMap<String, String>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/nodes",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Unregister a cluster node from the client
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/nodes/{node}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `node`
    pub async fn delete_clients_by_client_uuid_nodes_by_node<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        node: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/nodes/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&node.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get application offline session count Returns a number of offline user
    /// sessions associated with this client { "count": number }
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/offline-session-count`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    pub async fn get_clients_by_client_uuid_offline_session_count<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
    ) -> Result<ResponseValue<std::collections::HashMap<String, i64>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/offline-session-count",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get offline sessions for client Returns a list of offline user sessions
    /// associated with this client
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/offline-sessions`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `first`: Paging offset
    /// - `max`: Maximum results size (defaults to 100)
    pub async fn get_clients_by_client_uuid_offline_sessions<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<ResponseValue<Vec<types::UserSessionRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/offline-sessions",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        let mut query = Vec::with_capacity(2usize);
        if let Some(v) = &first {
            query.push(("first", v.to_string()));
        }

        if let Some(v) = &max {
            query.push(("max", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get optional client scopes.  Only name and ids are returned
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/optional-client-scopes`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    pub async fn get_clients_by_client_uuid_optional_client_scopes<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
    ) -> Result<ResponseValue<Vec<types::ClientScopeRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/optional-client-scopes",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/optional-client-scopes/
    /// {clientScopeId}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `client_scope_id`
    pub async fn update_clients_by_client_uuid_optional_client_scopes_by_client_scope_id<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        client_scope_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/optional-client-scopes/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/optional-client-scopes/
    /// {clientScopeId}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `client_scope_id`
    pub async fn delete_clients_by_client_uuid_optional_client_scopes_by_client_scope_id<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        client_scope_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/optional-client-scopes/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create multiple mappers
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/protocol-mappers/
    /// add-models`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    pub async fn create_clients_by_client_uuid_protocol_mappers_add_models<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        body: &'a Vec<types::ProtocolMapperRepresentation>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/protocol-mappers/add-models",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get mappers
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/protocol-mappers/models`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    pub async fn get_clients_by_client_uuid_protocol_mappers_models<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
    ) -> Result<ResponseValue<Vec<types::ProtocolMapperRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/protocol-mappers/models",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a mapper
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/protocol-mappers/models`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    pub async fn create_clients_by_client_uuid_protocol_mappers_models<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        body: &'a types::ProtocolMapperRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/protocol-mappers/models",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get mapper by id
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/protocol-mappers/models/
    /// {id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `id`: Mapper id
    pub async fn get_clients_by_client_uuid_protocol_mappers_models_by_id<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::ProtocolMapperRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/protocol-mappers/models/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update the mapper
    ///
    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/protocol-mappers/models/
    /// {id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `id`: Mapper id
    /// - `body`
    pub async fn update_clients_by_client_uuid_protocol_mappers_models_by_id<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        id: &'a str,
        body: &'a types::ProtocolMapperRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/protocol-mappers/models/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete the mapper
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/protocol-mappers/models/
    /// {id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `id`: Mapper id
    pub async fn delete_clients_by_client_uuid_protocol_mappers_models_by_id<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/protocol-mappers/models/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get mappers by name for a specific protocol
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/protocol-mappers/protocol/
    /// {protocol}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `protocol`
    pub async fn get_clients_by_client_uuid_protocol_mappers_protocol_by_protocol<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        protocol: &'a str,
    ) -> Result<ResponseValue<Vec<types::ProtocolMapperRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/protocol-mappers/protocol/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&protocol.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Push the client's revocation policy to its admin URL If the client has
    /// an admin URL, push revocation policy to it
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/push-revocation`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    pub async fn create_clients_by_client_uuid_push_revocation<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
    ) -> Result<ResponseValue<types::GlobalRequestResult>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/push-revocation",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Generate a new registration access token for the client
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/registration-access-token`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    pub async fn create_clients_by_client_uuid_registration_access_token<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
    ) -> Result<ResponseValue<types::ClientRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/registration-access-token",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get all roles for the realm or client
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/roles`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `brief_representation`
    /// - `first`
    /// - `max`
    /// - `search`
    pub async fn get_clients_by_client_uuid_roles<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        search: Option<&'a str>,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/roles",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        let mut query = Vec::with_capacity(4usize);
        if let Some(v) = &brief_representation {
            query.push(("briefRepresentation", v.to_string()));
        }

        if let Some(v) = &first {
            query.push(("first", v.to_string()));
        }

        if let Some(v) = &max {
            query.push(("max", v.to_string()));
        }

        if let Some(v) = &search {
            query.push(("search", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a new role for the realm or client
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/roles`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    pub async fn create_clients_by_client_uuid_roles<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        body: &'a types::RoleRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/roles",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get a role by name
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: role's name (not id!)
    pub async fn get_clients_by_client_uuid_roles_by_role_name<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        role_name: &'a str,
    ) -> Result<ResponseValue<types::RoleRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/roles/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&role_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update a role by name
    ///
    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: role's name (not id!)
    /// - `body`
    pub async fn update_clients_by_client_uuid_roles_by_role_name<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        role_name: &'a str,
        body: &'a types::RoleRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/roles/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&role_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete a role by name
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: role's name (not id!)
    pub async fn delete_clients_by_client_uuid_roles_by_role_name<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        role_name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/roles/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&role_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get composites of the role
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/
    /// composites`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: role's name (not id!)
    pub async fn get_clients_by_client_uuid_roles_by_role_name_composites<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        role_name: &'a str,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/roles/{}/composites",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&role_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Add a composite to the role
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/
    /// composites`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: role's name (not id!)
    /// - `body`
    pub async fn create_clients_by_client_uuid_roles_by_role_name_composites<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        role_name: &'a str,
        body: &'a Vec<types::RoleRepresentation>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/roles/{}/composites",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&role_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Remove roles from the role's composite
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/
    /// composites`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: role's name (not id!)
    /// - `body`
    pub async fn delete_clients_by_client_uuid_roles_by_role_name_composites<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        role_name: &'a str,
        body: &'a Vec<types::RoleRepresentation>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/roles/{}/composites",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&role_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get client-level roles for the client that are in the role's composite
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/
    /// composites/clients/{client-uuid}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`
    /// - `role_name`: role's name (not id!)
    pub async fn get_clients_by_client_uuid_roles_by_role_name_composites_clients_by_client_uuid<
        'a,
    >(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        role_name: &'a str,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/roles/{}/composites/clients/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&role_name.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get realm-level roles of the role's composite
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/
    /// composites/realm`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: role's name (not id!)
    pub async fn get_clients_by_client_uuid_roles_by_role_name_composites_realm<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        role_name: &'a str,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/roles/{}/composites/realm",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&role_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Returns a stream of groups that have the specified role name
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/groups`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: the role name.
    /// - `brief_representation`: if false, return a full representation of the
    ///   {@code GroupRepresentation} objects.
    /// - `first`: first result to return. Ignored if negative or {@code null}.
    /// - `max`: maximum number of results to return. Ignored if negative or
    ///   {@code null}.
    pub async fn get_clients_by_client_uuid_roles_by_role_name_groups<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        role_name: &'a str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<ResponseValue<Vec<types::GroupRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/roles/{}/groups",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&role_name.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &brief_representation {
            query.push(("briefRepresentation", v.to_string()));
        }

        if let Some(v) = &first {
            query.push(("first", v.to_string()));
        }

        if let Some(v) = &max {
            query.push(("max", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Return object stating whether role Authorization permissions have been
    /// initialized or not and a reference
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/
    /// management/permissions`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`
    pub async fn get_clients_by_client_uuid_roles_by_role_name_management_permissions<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        role_name: &'a str,
    ) -> Result<ResponseValue<types::ManagementPermissionReference>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/roles/{}/management/permissions",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&role_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Return object stating whether role Authorization permissions have been
    /// initialized or not and a reference
    ///
    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/
    /// management/permissions`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`
    /// - `body`
    pub async fn update_clients_by_client_uuid_roles_by_role_name_management_permissions<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        role_name: &'a str,
        body: &'a types::ManagementPermissionReference,
    ) -> Result<ResponseValue<types::ManagementPermissionReference>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/roles/{}/management/permissions",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&role_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Returns a stream of users that have the specified role name
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/roles/{role-name}/users`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `role_name`: the role name.
    /// - `first`: first result to return. Ignored if negative or {@code null}.
    /// - `max`: maximum number of results to return. Ignored if negative or
    ///   {@code null}.
    pub async fn get_clients_by_client_uuid_roles_by_role_name_users<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        role_name: &'a str,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<ResponseValue<Vec<types::UserRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/roles/{}/users",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&role_name.to_string()),
        );
        let mut query = Vec::with_capacity(2usize);
        if let Some(v) = &first {
            query.push(("first", v.to_string()));
        }

        if let Some(v) = &max {
            query.push(("max", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get all scope mappings for the client
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/scope-mappings`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    pub async fn get_clients_by_client_uuid_scope_mappings<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
    ) -> Result<ResponseValue<types::MappingsRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/scope-mappings",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get the roles associated with a client's scope Returns roles for the
    /// client
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/scope-mappings/clients/
    /// {client}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `client`
    pub async fn get_clients_by_client_uuid_scope_mappings_clients_by_client<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        client: &'a str,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/scope-mappings/clients/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&client.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Add client-level roles to the client's scope
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/scope-mappings/clients/
    /// {client}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `client`
    /// - `body`
    pub async fn create_clients_by_client_uuid_scope_mappings_clients_by_client<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        client: &'a str,
        body: &'a Vec<types::RoleRepresentation>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/scope-mappings/clients/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&client.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Remove client-level roles from the client's scope
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/scope-mappings/clients/
    /// {client}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `client`
    /// - `body`
    pub async fn delete_clients_by_client_uuid_scope_mappings_clients_by_client<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        client: &'a str,
        body: &'a Vec<types::RoleRepresentation>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/scope-mappings/clients/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&client.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///The available client-level roles Returns the roles for the client that
    /// can be associated with the client's scope
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/scope-mappings/clients/
    /// {client}/available`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `client`
    pub async fn get_clients_by_client_uuid_scope_mappings_clients_by_client_available<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        client: &'a str,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/scope-mappings/clients/{}/available",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&client.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get effective client roles Returns the roles for the client that are
    /// associated with the client's scope
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/scope-mappings/clients/
    /// {client}/composite`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `client`
    /// - `brief_representation`: if false, return roles with their attributes
    pub async fn get_clients_by_client_uuid_scope_mappings_clients_by_client_composite<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        client: &'a str,
        brief_representation: Option<bool>,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/scope-mappings/clients/{}/composite",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
            encode_path(&client.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &brief_representation {
            query.push(("briefRepresentation", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get realm-level roles associated with the client's scope
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/scope-mappings/realm`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    pub async fn get_clients_by_client_uuid_scope_mappings_realm<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/scope-mappings/realm",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Add a set of realm-level roles to the client's scope
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/scope-mappings/realm`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    pub async fn create_clients_by_client_uuid_scope_mappings_realm<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        body: &'a Vec<types::RoleRepresentation>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/scope-mappings/realm",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Remove a set of realm-level roles from the client's scope
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/scope-mappings/realm`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `body`
    pub async fn delete_clients_by_client_uuid_scope_mappings_realm<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        body: &'a Vec<types::RoleRepresentation>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/scope-mappings/realm",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get realm-level roles that are available to attach to this client's
    /// scope
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/scope-mappings/realm/
    /// available`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    pub async fn get_clients_by_client_uuid_scope_mappings_realm_available<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/scope-mappings/realm/available",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get effective realm-level roles associated with the client’s scope What
    /// this does is recurse any composite roles associated with the client’s
    /// scope and adds the roles to this lists
    ///
    ///The method is really to show a comprehensive total view of realm-level
    /// roles associated with the client.
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/scope-mappings/realm/
    /// composite`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `brief_representation`: if false, return roles with their attributes
    pub async fn get_clients_by_client_uuid_scope_mappings_realm_composite<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        brief_representation: Option<bool>,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/scope-mappings/realm/composite",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &brief_representation {
            query.push(("briefRepresentation", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get a user dedicated to the service account
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/service-account-user`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    pub async fn get_clients_by_client_uuid_service_account_user<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
    ) -> Result<ResponseValue<types::UserRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/service-account-user",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get application session count Returns a number of user sessions
    /// associated with this client { "count": number }
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/session-count`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    pub async fn get_clients_by_client_uuid_session_count<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
    ) -> Result<ResponseValue<std::collections::HashMap<String, i64>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/session-count",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Test if registered cluster nodes are available Tests availability by
    /// sending 'ping' request to all cluster nodes
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/test-nodes-available`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    pub async fn get_clients_by_client_uuid_test_nodes_available<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
    ) -> Result<ResponseValue<types::GlobalRequestResult>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/test-nodes-available",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get user sessions for client Returns a list of user sessions associated
    /// with this client
    ///
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/clients/{client-uuid}/user-sessions`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_uuid`: id of client (not client-id!)
    /// - `first`: Paging offset
    /// - `max`: Maximum results size (defaults to 100)
    pub async fn get_clients_by_client_uuid_user_sessions<'a>(
        &'a self,
        realm: &'a str,
        client_uuid: &'a str,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<ResponseValue<Vec<types::UserSessionRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/clients/{}/user-sessions",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        let mut query = Vec::with_capacity(2usize);
        if let Some(v) = &first {
            query.push(("first", v.to_string()));
        }

        if let Some(v) = &max {
            query.push(("max", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/admin/realms/{realm}/components`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `name`
    /// - `parent`
    /// - `type_`
    pub async fn get_components<'a>(
        &'a self,
        realm: &'a str,
        name: Option<&'a str>,
        parent: Option<&'a str>,
        type_: Option<&'a str>,
    ) -> Result<ResponseValue<Vec<types::ComponentRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/components",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &name {
            query.push(("name", v.to_string()));
        }

        if let Some(v) = &parent {
            query.push(("parent", v.to_string()));
        }

        if let Some(v) = &type_ {
            query.push(("type", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `POST` request to `/admin/realms/{realm}/components`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `body`
    pub async fn create_components<'a>(
        &'a self,
        realm: &'a str,
        body: &'a types::ComponentRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/components",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/admin/realms/{realm}/components/{id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `id`
    pub async fn get_components_by_id<'a>(
        &'a self,
        realm: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::ComponentRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/components/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PUT` request to `/admin/realms/{realm}/components/{id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `body`
    pub async fn update_components_by_id<'a>(
        &'a self,
        realm: &'a str,
        id: &'a str,
        body: &'a types::ComponentRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/components/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `DELETE` request to `/admin/realms/{realm}/components/{id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `id`
    pub async fn delete_components_by_id<'a>(
        &'a self,
        realm: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/components/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List of subcomponent types that are available to configure for a
    /// particular parent component
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/components/{id}/sub-component-types`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `id`
    /// - `type_`
    pub async fn get_components_by_id_sub_component_types<'a>(
        &'a self,
        realm: &'a str,
        id: &'a str,
        type_: Option<&'a str>,
    ) -> Result<ResponseValue<Vec<types::ComponentTypeRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/components/{}/sub-component-types",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&id.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &type_ {
            query.push(("type", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/admin/realms/{realm}/credential-registrators`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    pub async fn get_credential_registrators<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<ResponseValue<Vec<String>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/credential-registrators",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get realm default client scopes.  Only name and ids are returned
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/default-default-client-scopes`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    pub async fn get_default_default_client_scopes<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<ResponseValue<Vec<types::ClientScopeRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/default-default-client-scopes",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/default-default-client-scopes/{clientScopeId}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    pub async fn update_default_default_client_scopes_by_client_scope_id<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/default-default-client-scopes/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/default-default-client-scopes/{clientScopeId}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    pub async fn delete_default_default_client_scopes_by_client_scope_id<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/default-default-client-scopes/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get group hierarchy.  Only name and ids are returned
    ///
    ///Sends a `GET` request to `/admin/realms/{realm}/default-groups`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    pub async fn get_default_groups<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<ResponseValue<Vec<types::GroupRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/default-groups",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/default-groups/{groupId}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    pub async fn update_default_groups_by_group_id<'a>(
        &'a self,
        realm: &'a str,
        group_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/default-groups/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&group_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/default-groups/{groupId}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    pub async fn delete_default_groups_by_group_id<'a>(
        &'a self,
        realm: &'a str,
        group_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/default-groups/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&group_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get realm optional client scopes.  Only name and ids are returned
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/default-optional-client-scopes`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    pub async fn get_default_optional_client_scopes<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<ResponseValue<Vec<types::ClientScopeRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/default-optional-client-scopes",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/default-optional-client-scopes/{clientScopeId}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    pub async fn update_default_optional_client_scopes_by_client_scope_id<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/default-optional-client-scopes/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/default-optional-client-scopes/{clientScopeId}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client_scope_id`
    pub async fn delete_default_optional_client_scopes_by_client_scope_id<'a>(
        &'a self,
        realm: &'a str,
        client_scope_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/default-optional-client-scopes/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&client_scope_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get events Returns all events, or filters them based on URL query
    /// parameters listed here
    ///
    ///Sends a `GET` request to `/admin/realms/{realm}/events`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `client`: App or oauth client name
    /// - `date_from`: From date
    /// - `date_to`: To date
    /// - `first`: Paging offset
    /// - `ip_address`: IP Address
    /// - `max`: Maximum results size (defaults to 100)
    /// - `type_`: The types of events to return
    /// - `user`: User id
    pub async fn get_events<'a>(
        &'a self,
        realm: &'a str,
        client: Option<&'a str>,
        date_from: Option<&'a str>,
        date_to: Option<&'a str>,
        first: Option<i32>,
        ip_address: Option<&'a str>,
        max: Option<i32>,
        type_: Option<&'a Vec<String>>,
        user: Option<&'a str>,
    ) -> Result<ResponseValue<Vec<types::EventRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/events",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        let mut query = Vec::with_capacity(8usize);
        if let Some(v) = &client {
            query.push(("client", v.to_string()));
        }

        if let Some(v) = &date_from {
            query.push(("dateFrom", v.to_string()));
        }

        if let Some(v) = &date_to {
            query.push(("dateTo", v.to_string()));
        }

        if let Some(v) = &first {
            query.push(("first", v.to_string()));
        }

        if let Some(v) = &ip_address {
            query.push(("ipAddress", v.to_string()));
        }

        if let Some(v) = &max {
            query.push(("max", v.to_string()));
        }

        if let Some(v) = &type_ {
            query.push(("type", v.join(",").to_string()));
        }

        if let Some(v) = &user {
            query.push(("user", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete all events
    ///
    ///Sends a `DELETE` request to `/admin/realms/{realm}/events`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    pub async fn delete_events<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/events",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get the events provider configuration Returns JSON object with events
    /// provider configuration
    ///
    ///Sends a `GET` request to `/admin/realms/{realm}/events/config`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    pub async fn get_events_config<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<ResponseValue<types::RealmEventsConfigRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/events/config",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update the events provider Change the events provider and/or its
    /// configuration
    ///
    ///Sends a `PUT` request to `/admin/realms/{realm}/events/config`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `body`
    pub async fn update_events_config<'a>(
        &'a self,
        realm: &'a str,
        body: &'a types::RealmEventsConfigRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/events/config",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/admin/realms/{realm}/group-by-path/{path}`
    ///
    /// TODO Fix the path handling
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `path`
    pub async fn get_group_by_path_by_path<'a>(
        &'a self,
        realm: &'a str,
        _path: &'a Vec<types::PathSegment>,
    ) -> Result<ResponseValue<types::GroupRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/group-by-path/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&"broken")
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get group hierarchy.  Only name and ids are returned
    ///
    ///Sends a `GET` request to `/admin/realms/{realm}/groups`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `brief_representation`
    /// - `exact`
    /// - `first`
    /// - `max`
    /// - `populate_hierarchy`
    /// - `q`
    /// - `search`
    pub async fn get_groups<'a>(
        &'a self,
        realm: &'a str,
        brief_representation: Option<bool>,
        exact: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        populate_hierarchy: Option<bool>,
        q: Option<&'a str>,
        search: Option<&'a str>,
    ) -> Result<ResponseValue<Vec<types::GroupRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/groups",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        let mut query = Vec::with_capacity(7usize);
        if let Some(v) = &brief_representation {
            query.push(("briefRepresentation", v.to_string()));
        }

        if let Some(v) = &exact {
            query.push(("exact", v.to_string()));
        }

        if let Some(v) = &first {
            query.push(("first", v.to_string()));
        }

        if let Some(v) = &max {
            query.push(("max", v.to_string()));
        }

        if let Some(v) = &populate_hierarchy {
            query.push(("populateHierarchy", v.to_string()));
        }

        if let Some(v) = &q {
            query.push(("q", v.to_string()));
        }

        if let Some(v) = &search {
            query.push(("search", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///create or add a top level realm groupSet or create child
    ///
    ///This will update the group and set the parent if it exists. Create it
    /// and set the parent if the group doesn’t exist.
    ///
    ///Sends a `POST` request to `/admin/realms/{realm}/groups`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `body`
    pub async fn create_groups<'a>(
        &'a self,
        realm: &'a str,
        body: &'a types::GroupRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/groups",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Returns the groups counts
    ///
    ///Sends a `GET` request to `/admin/realms/{realm}/groups/count`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `search`
    /// - `top`
    pub async fn get_groups_count<'a>(
        &'a self,
        realm: &'a str,
        search: Option<&'a str>,
        top: Option<bool>,
    ) -> Result<ResponseValue<std::collections::HashMap<String, i64>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/groups/count",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        let mut query = Vec::with_capacity(2usize);
        if let Some(v) = &search {
            query.push(("search", v.to_string()));
        }

        if let Some(v) = &top {
            query.push(("top", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/admin/realms/{realm}/groups/{group-id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    pub async fn get_groups_by_group_id<'a>(
        &'a self,
        realm: &'a str,
        group_id: &'a str,
    ) -> Result<ResponseValue<types::GroupRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/groups/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&group_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update group, ignores subgroups
    ///
    ///Sends a `PUT` request to `/admin/realms/{realm}/groups/{group-id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `body`
    pub async fn update_groups_by_group_id<'a>(
        &'a self,
        realm: &'a str,
        group_id: &'a str,
        body: &'a types::GroupRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/groups/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&group_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `DELETE` request to `/admin/realms/{realm}/groups/{group-id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    pub async fn delete_groups_by_group_id<'a>(
        &'a self,
        realm: &'a str,
        group_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/groups/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&group_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Return a paginated list of subgroups that have a parent group
    /// corresponding to the group on the URL
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/groups/{group-id}/children`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `brief_representation`
    /// - `first`
    /// - `max`
    pub async fn get_groups_by_group_id_children<'a>(
        &'a self,
        realm: &'a str,
        group_id: &'a str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<ResponseValue<Vec<types::GroupRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/groups/{}/children",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&group_id.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &brief_representation {
            query.push(("briefRepresentation", v.to_string()));
        }

        if let Some(v) = &first {
            query.push(("first", v.to_string()));
        }

        if let Some(v) = &max {
            query.push(("max", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Set or create child
    ///
    ///This will just set the parent if it exists. Create it and set the parent
    /// if the group doesn’t exist.
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/groups/{group-id}/children`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `body`
    pub async fn create_groups_by_group_id_children<'a>(
        &'a self,
        realm: &'a str,
        group_id: &'a str,
        body: &'a types::GroupRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/groups/{}/children",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&group_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Return object stating whether client Authorization permissions have been
    /// initialized or not and a reference
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/groups/{group-id}/management/permissions`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    pub async fn get_groups_by_group_id_management_permissions<'a>(
        &'a self,
        realm: &'a str,
        group_id: &'a str,
    ) -> Result<ResponseValue<types::ManagementPermissionReference>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/groups/{}/management/permissions",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&group_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Return object stating whether client Authorization permissions have been
    /// initialized or not and a reference
    ///
    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/groups/{group-id}/management/permissions`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `body`
    pub async fn update_groups_by_group_id_management_permissions<'a>(
        &'a self,
        realm: &'a str,
        group_id: &'a str,
        body: &'a types::ManagementPermissionReference,
    ) -> Result<ResponseValue<types::ManagementPermissionReference>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/groups/{}/management/permissions",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&group_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get users Returns a stream of users, filtered according to query
    /// parameters
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/groups/{group-id}/members`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `brief_representation`: Only return basic information (only guaranteed
    ///   to return id, username, created, first and last name, email, enabled
    ///   state, email verification state, federation link, and access. Note
    ///   that it means that namely user attributes, required actions, and not
    ///   before are not returned.)
    /// - `first`: Pagination offset
    /// - `max`: Maximum results size (defaults to 100)
    pub async fn get_groups_by_group_id_members<'a>(
        &'a self,
        realm: &'a str,
        group_id: &'a str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<ResponseValue<Vec<types::UserRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/groups/{}/members",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&group_id.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &brief_representation {
            query.push(("briefRepresentation", v.to_string()));
        }

        if let Some(v) = &first {
            query.push(("first", v.to_string()));
        }

        if let Some(v) = &max {
            query.push(("max", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get role mappings
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/groups/{group-id}/role-mappings`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    pub async fn get_groups_by_group_id_role_mappings<'a>(
        &'a self,
        realm: &'a str,
        group_id: &'a str,
    ) -> Result<ResponseValue<types::MappingsRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/groups/{}/role-mappings",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&group_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get client-level role mappings for the user, and the app
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/groups/{group-id}/role-mappings/clients/{client}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `client`
    pub async fn get_groups_by_group_id_role_mappings_clients_by_client<'a>(
        &'a self,
        realm: &'a str,
        group_id: &'a str,
        client: &'a str,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/groups/{}/role-mappings/clients/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&group_id.to_string()),
            encode_path(&client.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Add client-level roles to the user role mapping
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/groups/{group-id}/role-mappings/clients/{client}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `client`
    /// - `body`
    pub async fn create_groups_by_group_id_role_mappings_clients_by_client<'a>(
        &'a self,
        realm: &'a str,
        group_id: &'a str,
        client: &'a str,
        body: &'a Vec<types::RoleRepresentation>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/groups/{}/role-mappings/clients/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&group_id.to_string()),
            encode_path(&client.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete client-level roles from user role mapping
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/groups/{group-id}/role-mappings/clients/{client}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `client`
    /// - `body`
    pub async fn delete_groups_by_group_id_role_mappings_clients_by_client<'a>(
        &'a self,
        realm: &'a str,
        group_id: &'a str,
        client: &'a str,
        body: &'a Vec<types::RoleRepresentation>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/groups/{}/role-mappings/clients/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&group_id.to_string()),
            encode_path(&client.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get available client-level roles that can be mapped to the user
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/groups/{group-id}/role-mappings/clients/{client}/
    /// available`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `client`
    pub async fn get_groups_by_group_id_role_mappings_clients_by_client_available<'a>(
        &'a self,
        realm: &'a str,
        group_id: &'a str,
        client: &'a str,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/groups/{}/role-mappings/clients/{}/available",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&group_id.to_string()),
            encode_path(&client.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get effective client-level role mappings This recurses any composite
    /// roles
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/groups/{group-id}/role-mappings/clients/{client}/
    /// composite`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `client`
    /// - `brief_representation`: if false, return roles with their attributes
    pub async fn get_groups_by_group_id_role_mappings_clients_by_client_composite<'a>(
        &'a self,
        realm: &'a str,
        group_id: &'a str,
        client: &'a str,
        brief_representation: Option<bool>,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/groups/{}/role-mappings/clients/{}/composite",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&group_id.to_string()),
            encode_path(&client.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &brief_representation {
            query.push(("briefRepresentation", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get realm-level role mappings
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/groups/{group-id}/role-mappings/realm`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    pub async fn get_groups_by_group_id_role_mappings_realm<'a>(
        &'a self,
        realm: &'a str,
        group_id: &'a str,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/groups/{}/role-mappings/realm",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&group_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Add realm-level role mappings to the user
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/groups/{group-id}/role-mappings/realm`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `body`
    pub async fn create_groups_by_group_id_role_mappings_realm<'a>(
        &'a self,
        realm: &'a str,
        group_id: &'a str,
        body: &'a Vec<types::RoleRepresentation>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/groups/{}/role-mappings/realm",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&group_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete realm-level role mappings
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/groups/{group-id}/role-mappings/realm`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `body`
    pub async fn delete_groups_by_group_id_role_mappings_realm<'a>(
        &'a self,
        realm: &'a str,
        group_id: &'a str,
        body: &'a Vec<types::RoleRepresentation>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/groups/{}/role-mappings/realm",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&group_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get realm-level roles that can be mapped
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/groups/{group-id}/role-mappings/realm/available`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    pub async fn get_groups_by_group_id_role_mappings_realm_available<'a>(
        &'a self,
        realm: &'a str,
        group_id: &'a str,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/groups/{}/role-mappings/realm/available",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&group_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get effective realm-level role mappings This will recurse all composite
    /// roles to get the result
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/groups/{group-id}/role-mappings/realm/composite`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `group_id`
    /// - `brief_representation`: if false, return roles with their attributes
    pub async fn get_groups_by_group_id_role_mappings_realm_composite<'a>(
        &'a self,
        realm: &'a str,
        group_id: &'a str,
        brief_representation: Option<bool>,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/groups/{}/role-mappings/realm/composite",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&group_id.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &brief_representation {
            query.push(("briefRepresentation", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Import identity provider from JSON body
    ///
    ///Import identity provider from uploaded JSON file
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/identity-provider/import-config`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `body`
    pub async fn create_identity_provider_import_config<'a>(
        &'a self,
        realm: &'a str,
        body: &'a serde_json::Map<String, serde_json::Value>,
    ) -> Result<ResponseValue<std::collections::HashMap<String, String>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/identity-provider/import-config",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///List identity providers
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/identity-provider/instances`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `brief_representation`: Boolean which defines whether brief
    ///   representations are returned (default: false)
    /// - `first`: Pagination offset
    /// - `max`: Maximum results size (defaults to 100)
    /// - `search`: Filter specific providers by name. Search can be prefix
    ///   (name*), contains (*name*) or exact ("name"). Default prefixed.
    pub async fn get_identity_provider_instances<'a>(
        &'a self,
        realm: &'a str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        search: Option<&'a str>,
    ) -> Result<ResponseValue<Vec<types::IdentityProviderRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/identity-provider/instances",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        let mut query = Vec::with_capacity(4usize);
        if let Some(v) = &brief_representation {
            query.push(("briefRepresentation", v.to_string()));
        }

        if let Some(v) = &first {
            query.push(("first", v.to_string()));
        }

        if let Some(v) = &max {
            query.push(("max", v.to_string()));
        }

        if let Some(v) = &search {
            query.push(("search", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a new identity provider
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/identity-provider/instances`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `body`
    pub async fn create_identity_provider_instances<'a>(
        &'a self,
        realm: &'a str,
        body: &'a types::IdentityProviderRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/identity-provider/instances",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get the identity provider
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/identity-provider/instances/{alias}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `alias`
    pub async fn get_identity_provider_instances_by_alias<'a>(
        &'a self,
        realm: &'a str,
        alias: &'a str,
    ) -> Result<ResponseValue<types::IdentityProviderRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/identity-provider/instances/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&alias.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update the identity provider
    ///
    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/identity-provider/instances/{alias}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `alias`
    /// - `body`
    pub async fn update_identity_provider_instances_by_alias<'a>(
        &'a self,
        realm: &'a str,
        alias: &'a str,
        body: &'a types::IdentityProviderRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/identity-provider/instances/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&alias.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete the identity provider
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/identity-provider/instances/{alias}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `alias`
    pub async fn delete_identity_provider_instances_by_alias<'a>(
        &'a self,
        realm: &'a str,
        alias: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/identity-provider/instances/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&alias.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Export public broker configuration for identity provider
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/identity-provider/instances/{alias}/export`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `alias`
    /// - `format`: Format to use
    pub async fn get_identity_provider_instances_by_alias_export<'a>(
        &'a self,
        realm: &'a str,
        alias: &'a str,
        format: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/identity-provider/instances/{}/export",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&alias.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &format {
            query.push(("format", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self.client.get(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Return object stating whether client Authorization permissions have been
    /// initialized or not and a reference
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/identity-provider/instances/{alias}/management/
    /// permissions`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `alias`
    pub async fn get_identity_provider_instances_by_alias_management_permissions<'a>(
        &'a self,
        realm: &'a str,
        alias: &'a str,
    ) -> Result<ResponseValue<types::ManagementPermissionReference>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/identity-provider/instances/{}/management/permissions",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&alias.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Return object stating whether client Authorization permissions have been
    /// initialized or not and a reference
    ///
    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/identity-provider/instances/{alias}/management/
    /// permissions`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `alias`
    /// - `body`
    pub async fn update_identity_provider_instances_by_alias_management_permissions<'a>(
        &'a self,
        realm: &'a str,
        alias: &'a str,
        body: &'a types::ManagementPermissionReference,
    ) -> Result<ResponseValue<types::ManagementPermissionReference>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/identity-provider/instances/{}/management/permissions",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&alias.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get mapper types for identity provider
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/identity-provider/instances/{alias}/mapper-types`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `alias`
    pub async fn get_identity_provider_instances_by_alias_mapper_types<'a>(
        &'a self,
        realm: &'a str,
        alias: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/identity-provider/instances/{}/mapper-types",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&alias.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get mappers for identity provider
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/identity-provider/instances/{alias}/mappers`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `alias`
    pub async fn get_identity_provider_instances_by_alias_mappers<'a>(
        &'a self,
        realm: &'a str,
        alias: &'a str,
    ) -> Result<ResponseValue<Vec<types::IdentityProviderMapperRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/identity-provider/instances/{}/mappers",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&alias.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Add a mapper to identity provider
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/identity-provider/instances/{alias}/mappers`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `alias`
    /// - `body`
    pub async fn create_identity_provider_instances_by_alias_mappers<'a>(
        &'a self,
        realm: &'a str,
        alias: &'a str,
        body: &'a types::IdentityProviderMapperRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/identity-provider/instances/{}/mappers",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&alias.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get mapper by id for the identity provider
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/identity-provider/instances/{alias}/mappers/{id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `alias`
    /// - `id`
    pub async fn get_identity_provider_instances_by_alias_mappers_by_id<'a>(
        &'a self,
        realm: &'a str,
        alias: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<types::IdentityProviderMapperRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/identity-provider/instances/{}/mappers/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&alias.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update a mapper for the identity provider
    ///
    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/identity-provider/instances/{alias}/mappers/{id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `alias`
    /// - `id`: Mapper id
    /// - `body`
    pub async fn update_identity_provider_instances_by_alias_mappers_by_id<'a>(
        &'a self,
        realm: &'a str,
        alias: &'a str,
        id: &'a str,
        body: &'a types::IdentityProviderMapperRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/identity-provider/instances/{}/mappers/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&alias.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete a mapper for the identity provider
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/identity-provider/instances/{alias}/mappers/{id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `alias`
    /// - `id`: Mapper id
    pub async fn delete_identity_provider_instances_by_alias_mappers_by_id<'a>(
        &'a self,
        realm: &'a str,
        alias: &'a str,
        id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/identity-provider/instances/{}/mappers/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&alias.to_string()),
            encode_path(&id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Reaload keys for the identity provider if the provider supports it,
    /// "true" is returned if reload was performed, "false" if not
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/identity-provider/instances/{alias}/reload-keys`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `alias`
    pub async fn get_identity_provider_instances_by_alias_reload_keys<'a>(
        &'a self,
        realm: &'a str,
        alias: &'a str,
    ) -> Result<ResponseValue<bool>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/identity-provider/instances/{}/reload-keys",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&alias.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get the identity provider factory for that provider id
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/identity-provider/providers/{provider_id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `provider_id`: The provider id to get the factory
    pub async fn get_identity_provider_providers_by_provider_id<'a>(
        &'a self,
        realm: &'a str,
        provider_id: &'a str,
    ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/identity-provider/providers/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&provider_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/admin/realms/{realm}/keys`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    pub async fn get_keys<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<ResponseValue<types::KeysMetadataRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/keys",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/admin/realms/{realm}/localization`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    pub async fn get_localization<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<ResponseValue<Vec<String>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/localization",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/admin/realms/{realm}/localization/{locale}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `locale`
    /// - `use_realm_default_locale_fallback`
    pub async fn get_localization_by_locale<'a>(
        &'a self,
        realm: &'a str,
        locale: &'a str,
        use_realm_default_locale_fallback: Option<bool>,
    ) -> Result<ResponseValue<std::collections::HashMap<String, String>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/localization/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&locale.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &use_realm_default_locale_fallback {
            query.push(("useRealmDefaultLocaleFallback", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Import localization from uploaded JSON file
    ///
    ///Sends a `POST` request to `/admin/realms/{realm}/localization/{locale}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `locale`
    /// - `body`
    pub async fn create_localization_by_locale<'a>(
        &'a self,
        realm: &'a str,
        locale: &'a str,
        body: &'a std::collections::HashMap<String, String>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/localization/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&locale.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/localization/{locale}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `locale`
    pub async fn delete_localization_by_locale<'a>(
        &'a self,
        realm: &'a str,
        locale: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/localization/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&locale.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/localization/{locale}/{key}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `locale`
    /// - `key`
    pub async fn get_localization_by_locale_by_key<'a>(
        &'a self,
        realm: &'a str,
        locale: &'a str,
        key: &'a str,
    ) -> Result<ResponseValue<ByteStream>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/localization/{}/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&locale.to_string()),
            encode_path(&key.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.get(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::stream(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/localization/{locale}/{key}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `locale`
    /// - `key`
    /// - `body`
    pub async fn update_localization_by_locale_by_key<'a>(
        &'a self,
        realm: &'a str,
        locale: &'a str,
        key: &'a str,
        body: String,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/localization/{}/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&locale.to_string()),
            encode_path(&key.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                reqwest::header::CONTENT_TYPE,
                reqwest::header::HeaderValue::from_static("text/plain"),
            )
            .body(body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/localization/{locale}/{key}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `locale`
    /// - `key`
    pub async fn delete_localization_by_locale_by_key<'a>(
        &'a self,
        realm: &'a str,
        locale: &'a str,
        key: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/localization/{}/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&locale.to_string()),
            encode_path(&key.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Removes all user sessions
    ///
    ///Any client that has an admin url will also be told to invalidate any
    /// sessions they have.
    ///
    ///Sends a `POST` request to `/admin/realms/{realm}/logout-all`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    pub async fn create_logout_all<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<ResponseValue<types::GlobalRequestResult>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/logout-all",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Partial export of existing realm into a JSON file
    ///
    ///Sends a `POST` request to `/admin/realms/{realm}/partial-export`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `export_clients`
    /// - `export_groups_and_roles`
    pub async fn create_partial_export<'a>(
        &'a self,
        realm: &'a str,
        export_clients: Option<bool>,
        export_groups_and_roles: Option<bool>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/partial-export",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        let mut query = Vec::with_capacity(2usize);
        if let Some(v) = &export_clients {
            query.push(("exportClients", v.to_string()));
        }

        if let Some(v) = &export_groups_and_roles {
            query.push(("exportGroupsAndRoles", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self.client.post(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Partial import from a JSON file to an existing realm
    ///
    ///Sends a `POST` request to `/admin/realms/{realm}/partialImport`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `body`
    pub async fn create_partial_import<'a>(
        &'a self,
        realm: &'a str,
        body: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/partialImport",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Push the realm's revocation policy to any client that has an admin url
    /// associated with it
    ///
    ///Sends a `POST` request to `/admin/realms/{realm}/push-revocation`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    pub async fn create_push_revocation<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<ResponseValue<types::GlobalRequestResult>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/push-revocation",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get all roles for the realm or client
    ///
    ///Sends a `GET` request to `/admin/realms/{realm}/roles`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `brief_representation`
    /// - `first`
    /// - `max`
    /// - `search`
    pub async fn get_roles<'a>(
        &'a self,
        realm: &'a str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        search: Option<&'a str>,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/roles",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        let mut query = Vec::with_capacity(4usize);
        if let Some(v) = &brief_representation {
            query.push(("briefRepresentation", v.to_string()));
        }

        if let Some(v) = &first {
            query.push(("first", v.to_string()));
        }

        if let Some(v) = &max {
            query.push(("max", v.to_string()));
        }

        if let Some(v) = &search {
            query.push(("search", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a new role for the realm or client
    ///
    ///Sends a `POST` request to `/admin/realms/{realm}/roles`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `body`
    pub async fn create_roles<'a>(
        &'a self,
        realm: &'a str,
        body: &'a types::RoleRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/roles",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get a specific role's representation
    ///
    ///Sends a `GET` request to `/admin/realms/{realm}/roles-by-id/{role-id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `role_id`: id of role
    pub async fn get_roles_by_id_by_role_id<'a>(
        &'a self,
        realm: &'a str,
        role_id: &'a str,
    ) -> Result<ResponseValue<types::RoleRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/roles-by-id/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&role_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update the role
    ///
    ///Sends a `PUT` request to `/admin/realms/{realm}/roles-by-id/{role-id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `role_id`: id of role
    /// - `body`
    pub async fn update_roles_by_id_by_role_id<'a>(
        &'a self,
        realm: &'a str,
        role_id: &'a str,
        body: &'a types::RoleRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/roles-by-id/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&role_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete the role
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/roles-by-id/{role-id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `role_id`: id of role
    pub async fn delete_roles_by_id_by_role_id<'a>(
        &'a self,
        realm: &'a str,
        role_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/roles-by-id/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&role_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get role's children Returns a set of role's children provided the role
    /// is a composite
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/roles-by-id/{role-id}/composites`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `role_id`
    /// - `first`
    /// - `max`
    /// - `search`
    pub async fn get_roles_by_id_by_role_id_composites<'a>(
        &'a self,
        realm: &'a str,
        role_id: &'a str,
        first: Option<i32>,
        max: Option<i32>,
        search: Option<&'a str>,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/roles-by-id/{}/composites",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&role_id.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &first {
            query.push(("first", v.to_string()));
        }

        if let Some(v) = &max {
            query.push(("max", v.to_string()));
        }

        if let Some(v) = &search {
            query.push(("search", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Make the role a composite role by associating some child roles
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/roles-by-id/{role-id}/composites`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `role_id`
    /// - `body`
    pub async fn create_roles_by_id_by_role_id_composites<'a>(
        &'a self,
        realm: &'a str,
        role_id: &'a str,
        body: &'a Vec<types::RoleRepresentation>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/roles-by-id/{}/composites",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&role_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Remove a set of roles from the role's composite
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/roles-by-id/{role-id}/composites`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `role_id`: Role id
    /// - `body`
    pub async fn delete_roles_by_id_by_role_id_composites<'a>(
        &'a self,
        realm: &'a str,
        role_id: &'a str,
        body: &'a Vec<types::RoleRepresentation>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/roles-by-id/{}/composites",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&role_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get client-level roles for the client that are in the role's composite
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/roles-by-id/{role-id}/composites/clients/
    /// {clientUuid}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `role_id`
    /// - `client_uuid`
    pub async fn get_roles_by_id_by_role_id_composites_clients_by_client_uuid<'a>(
        &'a self,
        realm: &'a str,
        role_id: &'a str,
        client_uuid: &'a str,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/roles-by-id/{}/composites/clients/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&role_id.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get realm-level roles that are in the role's composite
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/roles-by-id/{role-id}/composites/realm`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `role_id`
    pub async fn get_roles_by_id_by_role_id_composites_realm<'a>(
        &'a self,
        realm: &'a str,
        role_id: &'a str,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/roles-by-id/{}/composites/realm",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&role_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Return object stating whether role Authorization permissions have been
    /// initialized or not and a reference
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/roles-by-id/{role-id}/management/permissions`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `role_id`
    pub async fn get_roles_by_id_by_role_id_management_permissions<'a>(
        &'a self,
        realm: &'a str,
        role_id: &'a str,
    ) -> Result<ResponseValue<types::ManagementPermissionReference>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/roles-by-id/{}/management/permissions",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&role_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Return object stating whether role Authorization permissions have been
    /// initialized or not and a reference
    ///
    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/roles-by-id/{role-id}/management/permissions`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `role_id`
    /// - `body`
    pub async fn update_roles_by_id_by_role_id_management_permissions<'a>(
        &'a self,
        realm: &'a str,
        role_id: &'a str,
        body: &'a types::ManagementPermissionReference,
    ) -> Result<ResponseValue<types::ManagementPermissionReference>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/roles-by-id/{}/management/permissions",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&role_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get a role by name
    ///
    ///Sends a `GET` request to `/admin/realms/{realm}/roles/{role-name}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role's name (not id!)
    pub async fn get_roles_by_role_name<'a>(
        &'a self,
        realm: &'a str,
        role_name: &'a str,
    ) -> Result<ResponseValue<types::RoleRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/roles/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&role_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update a role by name
    ///
    ///Sends a `PUT` request to `/admin/realms/{realm}/roles/{role-name}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role's name (not id!)
    /// - `body`
    pub async fn update_roles_by_role_name<'a>(
        &'a self,
        realm: &'a str,
        role_name: &'a str,
        body: &'a types::RoleRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/roles/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&role_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete a role by name
    ///
    ///Sends a `DELETE` request to `/admin/realms/{realm}/roles/{role-name}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role's name (not id!)
    pub async fn delete_roles_by_role_name<'a>(
        &'a self,
        realm: &'a str,
        role_name: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/roles/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&role_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get composites of the role
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/roles/{role-name}/composites`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role's name (not id!)
    pub async fn get_roles_by_role_name_composites<'a>(
        &'a self,
        realm: &'a str,
        role_name: &'a str,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/roles/{}/composites",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&role_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Add a composite to the role
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/roles/{role-name}/composites`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role's name (not id!)
    /// - `body`
    pub async fn create_roles_by_role_name_composites<'a>(
        &'a self,
        realm: &'a str,
        role_name: &'a str,
        body: &'a Vec<types::RoleRepresentation>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/roles/{}/composites",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&role_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Remove roles from the role's composite
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/roles/{role-name}/composites`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role's name (not id!)
    /// - `body`
    pub async fn delete_roles_by_role_name_composites<'a>(
        &'a self,
        realm: &'a str,
        role_name: &'a str,
        body: &'a Vec<types::RoleRepresentation>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/roles/{}/composites",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&role_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get client-level roles for the client that are in the role's composite
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/roles/{role-name}/composites/clients/
    /// {client-uuid}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role's name (not id!)
    /// - `client_uuid`
    pub async fn get_roles_by_role_name_composites_clients_by_client_uuid<'a>(
        &'a self,
        realm: &'a str,
        role_name: &'a str,
        client_uuid: &'a str,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/roles/{}/composites/clients/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&role_name.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get realm-level roles of the role's composite
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/roles/{role-name}/composites/realm`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `role_name`: role's name (not id!)
    pub async fn get_roles_by_role_name_composites_realm<'a>(
        &'a self,
        realm: &'a str,
        role_name: &'a str,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/roles/{}/composites/realm",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&role_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Returns a stream of groups that have the specified role name
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/roles/{role-name}/groups`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `role_name`: the role name.
    /// - `brief_representation`: if false, return a full representation of the
    ///   {@code GroupRepresentation} objects.
    /// - `first`: first result to return. Ignored if negative or {@code null}.
    /// - `max`: maximum number of results to return. Ignored if negative or
    ///   {@code null}.
    pub async fn get_roles_by_role_name_groups<'a>(
        &'a self,
        realm: &'a str,
        role_name: &'a str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<ResponseValue<Vec<types::GroupRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/roles/{}/groups",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&role_name.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &brief_representation {
            query.push(("briefRepresentation", v.to_string()));
        }

        if let Some(v) = &first {
            query.push(("first", v.to_string()));
        }

        if let Some(v) = &max {
            query.push(("max", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Return object stating whether role Authorization permissions have been
    /// initialized or not and a reference
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/roles/{role-name}/management/permissions`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `role_name`
    pub async fn get_roles_by_role_name_management_permissions<'a>(
        &'a self,
        realm: &'a str,
        role_name: &'a str,
    ) -> Result<ResponseValue<types::ManagementPermissionReference>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/roles/{}/management/permissions",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&role_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Return object stating whether role Authorization permissions have been
    /// initialized or not and a reference
    ///
    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/roles/{role-name}/management/permissions`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `role_name`
    /// - `body`
    pub async fn update_roles_by_role_name_management_permissions<'a>(
        &'a self,
        realm: &'a str,
        role_name: &'a str,
        body: &'a types::ManagementPermissionReference,
    ) -> Result<ResponseValue<types::ManagementPermissionReference>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/roles/{}/management/permissions",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&role_name.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Returns a stream of users that have the specified role name
    ///
    ///Sends a `GET` request to `/admin/realms/{realm}/roles/{role-name}/users`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `role_name`: the role name.
    /// - `first`: first result to return. Ignored if negative or {@code null}.
    /// - `max`: maximum number of results to return. Ignored if negative or
    ///   {@code null}.
    pub async fn get_roles_by_role_name_users<'a>(
        &'a self,
        realm: &'a str,
        role_name: &'a str,
        first: Option<i32>,
        max: Option<i32>,
    ) -> Result<ResponseValue<Vec<types::UserRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/roles/{}/users",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&role_name.to_string()),
        );
        let mut query = Vec::with_capacity(2usize);
        if let Some(v) = &first {
            query.push(("first", v.to_string()));
        }

        if let Some(v) = &max {
            query.push(("max", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Remove a specific user session
    ///
    ///Any client that has an admin url will also be told to invalidate this
    /// particular session.
    ///
    ///Sends a `DELETE` request to `/admin/realms/{realm}/sessions/{session}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `session`
    /// - `is_offline`
    pub async fn delete_sessions_by_session<'a>(
        &'a self,
        realm: &'a str,
        session: &'a str,
        is_offline: Option<bool>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/sessions/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&session.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &is_offline {
            query.push(("isOffline", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self.client.delete(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Test SMTP connection with current logged in user
    ///
    ///Sends a `POST` request to `/admin/realms/{realm}/testSMTPConnection`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `body`
    pub async fn create_test_smtp_connection<'a>(
        &'a self,
        realm: &'a str,
        body: &'a std::collections::HashMap<String, String>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/testSMTPConnection",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get users Returns a stream of users, filtered according to query
    /// parameters
    ///
    ///Sends a `GET` request to `/admin/realms/{realm}/users`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `brief_representation`: Boolean which defines whether brief
    ///   representations are returned (default: false)
    /// - `email`: A String contained in email, or the complete email, if param
    ///   "exact" is true
    /// - `email_verified`: whether the email has been verified
    /// - `enabled`: Boolean representing if user is enabled or not
    /// - `exact`: Boolean which defines whether the params "last", "first",
    ///   "email" and "username" must match exactly
    /// - `first`: Pagination offset
    /// - `first_name`: A String contained in firstName, or the complete
    ///   firstName, if param "exact" is true
    /// - `idp_alias`: The alias of an Identity Provider linked to the user
    /// - `idp_user_id`: The userId at an Identity Provider linked to the user
    /// - `last_name`: A String contained in lastName, or the complete lastName,
    ///   if param "exact" is true
    /// - `max`: Maximum results size (defaults to 100)
    /// - `q`: A query to search for custom attributes, in the format
    ///   'key1:value2 key2:value2'
    /// - `search`: A String contained in username, first or last name, or
    ///   email. Default search behavior is prefix-based (e.g., foo or foo*).
    ///   Use *foo* for infix search and "foo" for exact search.
    /// - `username`: A String contained in username, or the complete username,
    ///   if param "exact" is true
    pub async fn get_users<'a>(
        &'a self,
        realm: &'a str,
        brief_representation: Option<bool>,
        email: Option<&'a str>,
        email_verified: Option<bool>,
        enabled: Option<bool>,
        exact: Option<bool>,
        first: Option<i32>,
        first_name: Option<&'a str>,
        idp_alias: Option<&'a str>,
        idp_user_id: Option<&'a str>,
        last_name: Option<&'a str>,
        max: Option<i32>,
        q: Option<&'a str>,
        search: Option<&'a str>,
        username: Option<&'a str>,
    ) -> Result<ResponseValue<Vec<types::UserRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        let mut query = Vec::with_capacity(14usize);
        if let Some(v) = &brief_representation {
            query.push(("briefRepresentation", v.to_string()));
        }

        if let Some(v) = &email {
            query.push(("email", v.to_string()));
        }

        if let Some(v) = &email_verified {
            query.push(("emailVerified", v.to_string()));
        }

        if let Some(v) = &enabled {
            query.push(("enabled", v.to_string()));
        }

        if let Some(v) = &exact {
            query.push(("exact", v.to_string()));
        }

        if let Some(v) = &first {
            query.push(("first", v.to_string()));
        }

        if let Some(v) = &first_name {
            query.push(("firstName", v.to_string()));
        }

        if let Some(v) = &idp_alias {
            query.push(("idpAlias", v.to_string()));
        }

        if let Some(v) = &idp_user_id {
            query.push(("idpUserId", v.to_string()));
        }

        if let Some(v) = &last_name {
            query.push(("lastName", v.to_string()));
        }

        if let Some(v) = &max {
            query.push(("max", v.to_string()));
        }

        if let Some(v) = &q {
            query.push(("q", v.to_string()));
        }

        if let Some(v) = &search {
            query.push(("search", v.to_string()));
        }

        if let Some(v) = &username {
            query.push(("username", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Create a new user Username must be unique
    ///
    ///Sends a `POST` request to `/admin/realms/{realm}/users`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `body`
    pub async fn create_users<'a>(
        &'a self,
        realm: &'a str,
        body: &'a types::UserRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/users-management-permissions`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    pub async fn get_users_management_permissions<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<ResponseValue<types::ManagementPermissionReference>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users-management-permissions",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/users-management-permissions`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `body`
    pub async fn update_users_management_permissions<'a>(
        &'a self,
        realm: &'a str,
        body: &'a types::ManagementPermissionReference,
    ) -> Result<ResponseValue<types::ManagementPermissionReference>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users-management-permissions",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Returns the number of users that match the given criteria
    ///
    ///It can be called in three different ways. 1. Don’t specify any criteria
    /// and pass {@code null}. The number of all users within that realm will be
    /// returned. <p> 2. If {@code search} is specified other criteria such as
    /// {@code last} will be ignored even though you set them. The {@code
    /// search} string will be matched against the first and last name, the
    /// username and the email of a user. <p> 3. If {@code search} is
    /// unspecified but any of {@code last}, {@code first}, {@code email} or
    /// {@code username} those criteria are matched against their respective
    /// fields on a user entity. Combined with a logical and.
    ///
    ///Sends a `GET` request to `/admin/realms/{realm}/users/count`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `email`: email filter
    /// - `email_verified`
    /// - `enabled`: Boolean representing if user is enabled or not
    /// - `first_name`: first name filter
    /// - `last_name`: last name filter
    /// - `q`
    /// - `search`: arbitrary search string for all the fields below. Default
    ///   search behavior is prefix-based (e.g., foo or foo*). Use *foo* for
    ///   infix search and "foo" for exact search.
    /// - `username`: username filter
    pub async fn get_users_count<'a>(
        &'a self,
        realm: &'a str,
        email: Option<&'a str>,
        email_verified: Option<bool>,
        enabled: Option<bool>,
        first_name: Option<&'a str>,
        last_name: Option<&'a str>,
        q: Option<&'a str>,
        search: Option<&'a str>,
        username: Option<&'a str>,
    ) -> Result<ResponseValue<i32>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/count",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        let mut query = Vec::with_capacity(8usize);
        if let Some(v) = &email {
            query.push(("email", v.to_string()));
        }

        if let Some(v) = &email_verified {
            query.push(("emailVerified", v.to_string()));
        }

        if let Some(v) = &enabled {
            query.push(("enabled", v.to_string()));
        }

        if let Some(v) = &first_name {
            query.push(("firstName", v.to_string()));
        }

        if let Some(v) = &last_name {
            query.push(("lastName", v.to_string()));
        }

        if let Some(v) = &q {
            query.push(("q", v.to_string()));
        }

        if let Some(v) = &search {
            query.push(("search", v.to_string()));
        }

        if let Some(v) = &username {
            query.push(("username", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get the configuration for the user profile
    ///
    ///Sends a `GET` request to `/admin/realms/{realm}/users/profile`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    pub async fn get_users_profile<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<ResponseValue<types::UpConfig>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/profile",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Set the configuration for the user profile
    ///
    ///Sends a `PUT` request to `/admin/realms/{realm}/users/profile`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `body`
    pub async fn update_users_profile<'a>(
        &'a self,
        realm: &'a str,
        body: &'a types::UpConfig,
    ) -> Result<ResponseValue<types::UpConfig>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/profile",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .json(&body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get the UserProfileMetadata from the configuration
    ///
    ///Sends a `GET` request to `/admin/realms/{realm}/users/profile/metadata`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    pub async fn get_users_profile_metadata<'a>(
        &'a self,
        realm: &'a str,
    ) -> Result<ResponseValue<types::UserProfileMetadata>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/profile/metadata",
            self.baseurl,
            encode_path(&realm.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get representation of the user
    ///
    ///Sends a `GET` request to `/admin/realms/{realm}/users/{user-id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `user_profile_metadata`: Indicates if the user profile metadata should
    ///   be added to the response
    pub async fn get_users_by_user_id<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
        user_profile_metadata: Option<bool>,
    ) -> Result<ResponseValue<types::UserRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &user_profile_metadata {
            query.push(("userProfileMetadata", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update the user
    ///
    ///Sends a `PUT` request to `/admin/realms/{realm}/users/{user-id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `body`
    pub async fn update_users_by_user_id<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
        body: &'a types::UserRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete the user
    ///
    ///Sends a `DELETE` request to `/admin/realms/{realm}/users/{user-id}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    pub async fn delete_users_by_user_id<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Return credential types, which are provided by the user storage where
    /// user is stored
    ///
    ///Returned values can contain for example "password", "otp" etc. This will
    /// always return empty list for "local" users, which are not backed by any
    /// user storage
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/users/{user-id}/
    /// configured-user-storage-credential-types`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    pub async fn get_users_by_user_id_configured_user_storage_credential_types<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
    ) -> Result<ResponseValue<Vec<String>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/configured-user-storage-credential-types",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get consents granted by the user
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/users/{user-id}/consents`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    pub async fn get_users_by_user_id_consents<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
    ) -> Result<ResponseValue<Vec<serde_json::Map<String, serde_json::Value>>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/consents",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Revoke consent and offline tokens for particular client from user
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/users/{user-id}/consents/{client}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client`: Client id
    pub async fn delete_users_by_user_id_consents_by_client<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
        client: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/consents/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
            encode_path(&client.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/users/{user-id}/credentials`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    pub async fn get_users_by_user_id_credentials<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
    ) -> Result<ResponseValue<Vec<types::CredentialRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/credentials",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Remove a credential for a user
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/users/{user-id}/credentials/{credentialId}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `credential_id`
    pub async fn delete_users_by_user_id_credentials_by_credential_id<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
        credential_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/credentials/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
            encode_path(&credential_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Move a credential to a position behind another credential
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/users/{user-id}/credentials/{credentialId}/
    /// moveAfter/{newPreviousCredentialId}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `credential_id`: The credential to move
    /// - `new_previous_credential_id`: The credential that will be the previous
    ///   element in the list. If set to null, the moved credential will be the
    ///   first element in the list.
    pub async fn create_users_by_user_id_credentials_by_credential_id_move_after_by_new_previous_credential_id<
        'a,
    >(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
        credential_id: &'a str,
        new_previous_credential_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/credentials/{}/moveAfter/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
            encode_path(&credential_id.to_string()),
            encode_path(&new_previous_credential_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Move a credential to a first position in the credentials list of the
    /// user
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/users/{user-id}/credentials/{credentialId}/
    /// moveToFirst`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `credential_id`: The credential to move
    pub async fn create_users_by_user_id_credentials_by_credential_id_move_to_first<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
        credential_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/credentials/{}/moveToFirst",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
            encode_path(&credential_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Update a credential label for a user
    ///
    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/users/{user-id}/credentials/{credentialId}/
    /// userLabel`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `credential_id`
    /// - `body`
    pub async fn update_users_by_user_id_credentials_by_credential_id_user_label<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
        credential_id: &'a str,
        body: String,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/credentials/{}/userLabel",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
            encode_path(&credential_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .put(url)
            .header(
                reqwest::header::CONTENT_TYPE,
                reqwest::header::HeaderValue::from_static("text/plain"),
            )
            .body(body)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Disable all credentials for a user of a specific type
    ///
    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/users/{user-id}/disable-credential-types`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `body`
    pub async fn update_users_by_user_id_disable_credential_types<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
        body: &'a Vec<String>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/disable-credential-types",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Send an email to the user with a link they can click to execute
    /// particular actions
    ///
    ///An email contains a link the user can click to perform a set of required
    /// actions. The redirectUri and clientId parameters are optional. If no
    /// redirect is given, then there will be no link back to click after
    /// actions have completed. Redirect uri must be a valid uri for the
    /// particular clientId.
    ///
    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/users/{user-id}/execute-actions-email`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client_id`: Client id
    /// - `lifespan`: Number of seconds after which the generated token expires
    /// - `redirect_uri`: Redirect uri
    /// - `body`
    pub async fn update_users_by_user_id_execute_actions_email<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
        client_id: Option<&'a str>,
        lifespan: Option<i32>,
        redirect_uri: Option<&'a str>,
        body: &'a Vec<String>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/execute-actions-email",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &client_id {
            query.push(("client_id", v.to_string()));
        }

        if let Some(v) = &lifespan {
            query.push(("lifespan", v.to_string()));
        }

        if let Some(v) = &redirect_uri {
            query.push(("redirect_uri", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get social logins associated with the user
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/users/{user-id}/federated-identity`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    pub async fn get_users_by_user_id_federated_identity<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
    ) -> Result<ResponseValue<Vec<types::FederatedIdentityRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/federated-identity",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Add a social login provider to the user
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/users/{user-id}/federated-identity/{provider}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `provider`: Social login provider id
    pub async fn create_users_by_user_id_federated_identity_by_provider<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
        provider: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/federated-identity/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
            encode_path(&provider.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Remove a social login provider from user
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/users/{user-id}/federated-identity/{provider}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `provider`: Social login provider id
    pub async fn delete_users_by_user_id_federated_identity_by_provider<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
        provider: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/federated-identity/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
            encode_path(&provider.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to `/admin/realms/{realm}/users/{user-id}/groups`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `brief_representation`
    /// - `first`
    /// - `max`
    /// - `search`
    pub async fn get_users_by_user_id_groups<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
        brief_representation: Option<bool>,
        first: Option<i32>,
        max: Option<i32>,
        search: Option<&'a str>,
    ) -> Result<ResponseValue<Vec<types::GroupRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/groups",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
        );
        let mut query = Vec::with_capacity(4usize);
        if let Some(v) = &brief_representation {
            query.push(("briefRepresentation", v.to_string()));
        }

        if let Some(v) = &first {
            query.push(("first", v.to_string()));
        }

        if let Some(v) = &max {
            query.push(("max", v.to_string()));
        }

        if let Some(v) = &search {
            query.push(("search", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/users/{user-id}/groups/count`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `search`
    pub async fn get_users_by_user_id_groups_count<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
        search: Option<&'a str>,
    ) -> Result<ResponseValue<std::collections::HashMap<String, i64>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/groups/count",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &search {
            query.push(("search", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/users/{user-id}/groups/{groupId}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `group_id`
    pub async fn update_users_by_user_id_groups_by_group_id<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
        group_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/groups/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
            encode_path(&group_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/users/{user-id}/groups/{groupId}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `group_id`
    pub async fn delete_users_by_user_id_groups_by_group_id<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
        group_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/groups/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
            encode_path(&group_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Impersonate the user
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/users/{user-id}/impersonation`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    pub async fn create_users_by_user_id_impersonation<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
    ) -> Result<ResponseValue<serde_json::Map<String, serde_json::Value>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/impersonation",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .post(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Remove all user sessions associated with the user Also send notification
    /// to all clients that have an admin URL to invalidate the sessions for the
    /// particular user
    ///
    ///Sends a `POST` request to `/admin/realms/{realm}/users/{user-id}/logout`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    pub async fn create_users_by_user_id_logout<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/logout",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get offline sessions associated with the user and client
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/users/{user-id}/offline-sessions/{clientUuid}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client_uuid`
    pub async fn get_users_by_user_id_offline_sessions_by_client_uuid<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
        client_uuid: &'a str,
    ) -> Result<ResponseValue<Vec<types::UserSessionRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/offline-sessions/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
            encode_path(&client_uuid.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Set up a new password for the user
    ///
    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/users/{user-id}/reset-password`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `body`
    pub async fn update_users_by_user_id_reset_password<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
        body: &'a types::CredentialRepresentation,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/reset-password",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.put(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Send an email to the user with a link they can click to reset their
    /// password
    ///
    ///The redirectUri and clientId parameters are optional. The default for
    /// the redirect is the account client. This endpoint has been deprecated.
    /// Please use the execute-actions-email passing a list with UPDATE_PASSWORD
    /// within it.
    ///
    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/users/{user-id}/reset-password-email`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client_id`: client id
    /// - `redirect_uri`: redirect uri
    pub async fn update_users_by_user_id_reset_password_email<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
        client_id: Option<&'a str>,
        redirect_uri: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/reset-password-email",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
        );
        let mut query = Vec::with_capacity(2usize);
        if let Some(v) = &client_id {
            query.push(("client_id", v.to_string()));
        }

        if let Some(v) = &redirect_uri {
            query.push(("redirect_uri", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self.client.put(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get role mappings
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/users/{user-id}/role-mappings`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    pub async fn get_users_by_user_id_role_mappings<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
    ) -> Result<ResponseValue<types::MappingsRepresentation>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/role-mappings",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get client-level role mappings for the user, and the app
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/users/{user-id}/role-mappings/clients/{client}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client`
    pub async fn get_users_by_user_id_role_mappings_clients_by_client<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
        client: &'a str,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/role-mappings/clients/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
            encode_path(&client.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Add client-level roles to the user role mapping
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/users/{user-id}/role-mappings/clients/{client}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client`
    /// - `body`
    pub async fn create_users_by_user_id_role_mappings_clients_by_client<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
        client: &'a str,
        body: &'a Vec<types::RoleRepresentation>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/role-mappings/clients/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
            encode_path(&client.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete client-level roles from user role mapping
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/users/{user-id}/role-mappings/clients/{client}`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client`
    /// - `body`
    pub async fn delete_users_by_user_id_role_mappings_clients_by_client<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
        client: &'a str,
        body: &'a Vec<types::RoleRepresentation>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/role-mappings/clients/{}",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
            encode_path(&client.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get available client-level roles that can be mapped to the user
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/users/{user-id}/role-mappings/clients/{client}/
    /// available`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client`
    pub async fn get_users_by_user_id_role_mappings_clients_by_client_available<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
        client: &'a str,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/role-mappings/clients/{}/available",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
            encode_path(&client.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get effective client-level role mappings This recurses any composite
    /// roles
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/users/{user-id}/role-mappings/clients/{client}/
    /// composite`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client`
    /// - `brief_representation`: if false, return roles with their attributes
    pub async fn get_users_by_user_id_role_mappings_clients_by_client_composite<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
        client: &'a str,
        brief_representation: Option<bool>,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/role-mappings/clients/{}/composite",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
            encode_path(&client.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &brief_representation {
            query.push(("briefRepresentation", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get realm-level role mappings
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/users/{user-id}/role-mappings/realm`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    pub async fn get_users_by_user_id_role_mappings_realm<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/role-mappings/realm",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Add realm-level role mappings to the user
    ///
    ///Sends a `POST` request to
    /// `/admin/realms/{realm}/users/{user-id}/role-mappings/realm`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `body`
    pub async fn create_users_by_user_id_role_mappings_realm<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
        body: &'a Vec<types::RoleRepresentation>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/role-mappings/realm",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.post(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Delete realm-level role mappings
    ///
    ///Sends a `DELETE` request to
    /// `/admin/realms/{realm}/users/{user-id}/role-mappings/realm`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `body`
    pub async fn delete_users_by_user_id_role_mappings_realm<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
        body: &'a Vec<types::RoleRepresentation>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/role-mappings/realm",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self.client.delete(url).json(&body).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            204u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get realm-level roles that can be mapped
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/users/{user-id}/role-mappings/realm/available`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    pub async fn get_users_by_user_id_role_mappings_realm_available<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/role-mappings/realm/available",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get effective realm-level role mappings This will recurse all composite
    /// roles to get the result
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/users/{user-id}/role-mappings/realm/composite`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `brief_representation`: if false, return roles with their attributes
    pub async fn get_users_by_user_id_role_mappings_realm_composite<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
        brief_representation: Option<bool>,
    ) -> Result<ResponseValue<Vec<types::RoleRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/role-mappings/realm/composite",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
        );
        let mut query = Vec::with_capacity(1usize);
        if let Some(v) = &brief_representation {
            query.push(("briefRepresentation", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .query(&query)
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Send an email-verification email to the user An email contains a link
    /// the user can click to verify their email address
    ///
    ///The redirectUri, clientId and lifespan parameters are optional. The
    /// default for the redirect is the account client. The default for the
    /// lifespan is 12 hours
    ///
    ///Sends a `PUT` request to
    /// `/admin/realms/{realm}/users/{user-id}/send-verify-email`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    /// - `client_id`: Client id
    /// - `lifespan`: Number of seconds after which the generated token expires
    /// - `redirect_uri`: Redirect uri
    pub async fn update_users_by_user_id_send_verify_email<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
        client_id: Option<&'a str>,
        lifespan: Option<i32>,
        redirect_uri: Option<&'a str>,
    ) -> Result<ResponseValue<()>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/send-verify-email",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
        );
        let mut query = Vec::with_capacity(3usize);
        if let Some(v) = &client_id {
            query.push(("client_id", v.to_string()));
        }

        if let Some(v) = &lifespan {
            query.push(("lifespan", v.to_string()));
        }

        if let Some(v) = &redirect_uri {
            query.push(("redirect_uri", v.to_string()));
        }

        #[allow(unused_mut)]
        let mut request = self.client.put(url).query(&query).build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => Ok(ResponseValue::empty(response)),
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }

    ///Get sessions associated with the user
    ///
    ///Sends a `GET` request to
    /// `/admin/realms/{realm}/users/{user-id}/sessions`
    ///
    ///Arguments:
    /// - `realm`: realm name (not id!)
    /// - `user_id`
    pub async fn get_users_by_user_id_sessions<'a>(
        &'a self,
        realm: &'a str,
        user_id: &'a str,
    ) -> Result<ResponseValue<Vec<types::UserSessionRepresentation>>, Error<()>> {
        let url = format!(
            "{}/admin/realms/{}/users/{}/sessions",
            self.baseurl,
            encode_path(&realm.to_string()),
            encode_path(&user_id.to_string()),
        );
        #[allow(unused_mut)]
        let mut request = self
            .client
            .get(url)
            .header(
                reqwest::header::ACCEPT,
                reqwest::header::HeaderValue::from_static("application/json"),
            )
            .build()?;
        let result = self.client.execute(request).await;
        let response = result?;
        match response.status().as_u16() {
            200u16 => ResponseValue::from_response(response).await,
            _ => Err(Error::UnexpectedResponse(response)),
        }
    }
}

/// Items consumers will typically use such as the Client.
pub mod prelude {
    #[allow(unused_imports)]
    pub use super::Client;
}
