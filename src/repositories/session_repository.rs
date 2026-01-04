const SUBJECT_SESSION_KEY: &str = "OIDC_SUBJECT_KEY";
const OIDC_CSRF_STATE_KEY: &str = "OIDC_CSRF_STATE_KEY";
const OIDC_NONCE_KEY: &str = "OIDC_NONCE_KEY";
const OIDC_PKCE_VERIFIER_KEY: &str = "OIDC_PKCE_VERIFIER_KEY";
use openidconnect::{Nonce, PkceCodeVerifier};
use tokio::try_join;
use tower_sessions::Session;
use tower_sessions_core::session::Error;

pub struct SessionRepositoryError {
    pub description: String,
}

impl From<Error> for SessionRepositoryError {
    fn from(value: Error) -> Self {
        SessionRepositoryError {
            description: value.to_string(),
        }
    }
}

pub trait SessionRepository {
    async fn get_oidc_user_subject(&self) -> Result<Option<String>, SessionRepositoryError>;
    async fn get_oidc_crsf_token(&self) -> Result<Option<String>, SessionRepositoryError>;
    async fn get_oidc_nonce_key(&self) -> Result<Option<Nonce>, SessionRepositoryError>;
    async fn get_pkce_verifier(&self) -> Result<Option<PkceCodeVerifier>, SessionRepositoryError>;

    async fn clear_oidc_flow_details(&self) -> Result<(), SessionRepositoryError>;

    async fn save_oidc_user_subject(
        &self,
        oidc_subject: String,
    ) -> Result<(), SessionRepositoryError>;
    async fn save_oidc_crsf_token(&self, csrf_token: String) -> Result<(), SessionRepositoryError>;
    async fn save_oidc_nonce_key(&self, nonce: Nonce) -> Result<(), SessionRepositoryError>;
    async fn save_pkce_verifier(
        &self,
        verifier: PkceCodeVerifier,
    ) -> Result<(), SessionRepositoryError>;
}

pub struct TowerSessionRepository {
    session: Session,
}

impl TowerSessionRepository {
    pub fn new(session: Session) -> TowerSessionRepository {
        TowerSessionRepository { session }
    }
}

impl SessionRepository for TowerSessionRepository {
    async fn get_oidc_user_subject(&self) -> Result<Option<String>, SessionRepositoryError> {
        let test = self.session.get::<String>(SUBJECT_SESSION_KEY).await?;
        Ok(test)
    }

    async fn get_oidc_crsf_token(&self) -> Result<Option<String>, SessionRepositoryError> {
        let csrf_token = self.session.get::<String>(OIDC_CSRF_STATE_KEY).await?;
        Ok(csrf_token)
    }

    async fn get_oidc_nonce_key(&self) -> Result<Option<Nonce>, SessionRepositoryError> {
        let nonce_key = self.session.get::<Nonce>(OIDC_NONCE_KEY).await?;
        Ok(nonce_key)
    }

    async fn get_pkce_verifier(&self) -> Result<Option<PkceCodeVerifier>, SessionRepositoryError> {
        let pkce_verifier = self
            .session
            .get::<PkceCodeVerifier>(OIDC_PKCE_VERIFIER_KEY)
            .await?;
        Ok(pkce_verifier)
    }

    async fn save_oidc_user_subject(
        &self,
        oidc_subject: String,
    ) -> Result<(), SessionRepositoryError> {
        self.session
            .insert(SUBJECT_SESSION_KEY, oidc_subject)
            .await?;

        Ok(())
    }

    async fn save_oidc_crsf_token(&self, csrf_token: String) -> Result<(), SessionRepositoryError> {
        self.session.insert(OIDC_CSRF_STATE_KEY, csrf_token).await?;
        Ok(())
    }

    async fn save_oidc_nonce_key(&self, nonce: Nonce) -> Result<(), SessionRepositoryError> {
        self.session.insert(OIDC_NONCE_KEY, nonce).await?;
        Ok(())
    }

    async fn save_pkce_verifier(
        &self,
        verifier: PkceCodeVerifier,
    ) -> Result<(), SessionRepositoryError> {
        self.session
            .insert(OIDC_PKCE_VERIFIER_KEY, verifier)
            .await?;
        Ok(())
    }

    async fn clear_oidc_flow_details(&self) -> Result<(), SessionRepositoryError> {
        let d1 = self.session.remove::<String>(OIDC_CSRF_STATE_KEY);
        let d2 = self.session.remove::<Nonce>(OIDC_NONCE_KEY);
        let d3 = self
            .session
            .remove::<PkceCodeVerifier>(OIDC_PKCE_VERIFIER_KEY);

        try_join!(d1, d2, d3)?;

        Ok(())
    }
}
