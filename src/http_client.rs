//! HTTPクライアントを定義する。

use crate::error::*;
use crate::response::*;
use async_trait::async_trait;

/// HTTPクライアントのtrait。GET, POSTとか。
#[async_trait]
pub trait HttpClient {
    async fn get(&self, url: String) -> Result<RawResponse, Error>;
}

/// ネットワークアクセス時に用いるHttpクライアント。
/// Rustではreqwestがデファクトっぽいのでネットワークアクセスするときはreqwestを使う。
pub struct Reqwest;

#[async_trait]
impl HttpClient for Reqwest {
    async fn get(&self, url: String) -> Result<RawResponse, Error> {
        // ここでunwrapを使うとエラーが起きた時にPanicになるが、どうやってそれを回避すればいいのかがわからない。
        // ParseErrorをcrate::Errorに変換したいけどどうやってやるんだ？
        let url_as_reqwest_style = reqwest::Url::parse(&url).unwrap();

        let response = reqwest::get(url_as_reqwest_style).await?;
        let status_code = response.status().as_u16();
        let body = response.text().await?;
        Ok(RawResponse {
            http_status_code: (status_code),
            body_text: (body),
        })
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    /// 単体テスト用のHttpクライアント。
    pub struct InmemClient {
        pub http_status_code: u16,
        pub body_text: String,
        pub return_error: bool,
    }

    #[async_trait]
    impl HttpClient for InmemClient {
        async fn get(&self, _url: String) -> Result<RawResponse, Error> {
            if (self.return_error) {
                return Err(Error::UnknownError {});
            }

            Ok(RawResponse {
                http_status_code: (self.http_status_code),
                body_text: (self.body_text.clone()),
            })
        }
    }
}
