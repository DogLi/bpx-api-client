use crate::error::Result;
use std::collections::HashMap;

use bpx_api_types::{
    capital::{Balance, Deposit, DepositAddress, RequestWithdrawalPayload, Withdrawal},
    Blockchain,
};

use crate::BpxClient;

impl BpxClient {
    pub async fn get_balances(&self) -> Result<HashMap<String, Balance>> {
        let url = format!("{}/api/v1/capital", self.base_url);
        let res = self.get(url).await?;
        Self::handle_response(res).await
    }

    pub async fn get_deposits(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Deposit>> {
        let mut url = format!("{}/wapi/v1/capital/deposits", self.base_url);
        for (k, v) in [("limit", limit), ("offset", offset)] {
            if let Some(v) = v {
                url.push_str(&format!("&{}={}", k, v));
            }
        }
        let res = self.get(url).await?;
        Self::handle_response(res).await
    }

    pub async fn get_deposit_address(&self, blockchain: Blockchain) -> Result<DepositAddress> {
        let url = format!(
            "{}/wapi/v1/capital/deposit/address?blockchain={}",
            self.base_url, blockchain
        );
        let res = self.get(url).await?;
        Self::handle_response(res).await
    }

    pub async fn get_withdrawals(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<Withdrawal>> {
        let mut url = format!("{}/wapi/v1/capital/withdrawals", self.base_url);
        for (k, v) in [("limit", limit), ("offset", offset)] {
            if let Some(v) = v {
                url.push_str(&format!("{}={}&", k, v));
            }
        }
        let res = self.get(url).await?;
        Self::handle_response(res).await
    }

    pub async fn request_withdrawal(&self, payload: RequestWithdrawalPayload) -> Result<()> {
        let endpoint = format!("{}/wapi/v1/capital/withdrawals", self.base_url);
        self.post(endpoint, payload).await.map(|_| ())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[tokio::test]
    async fn test_get_balances() {
        let api_key = "jDh8SSOiDcwhrcEurUWnKuBgeYDS4wBBIZ8uquoJ0Q8=";
        let secret_key = "W5hZ57pzJza1QGqZyu+y0N+cwvKome4rsLE3rdJ1S/c=";
        let client = BpxClient::init(api_key, secret_key).unwrap();
        let balances = client.get_balances().await.unwrap();
        println!("{balances:?}");
    }
}
