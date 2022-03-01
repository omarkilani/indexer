use objects::{nft::Nft, storefront::Storefront};
use scalars::Lamports;

use super::prelude::*;

#[derive(Debug, Clone)]
pub struct Bid {
    pub listing_address: String,
    pub bidder_address: String,
    pub last_bid_time: String,
    pub last_bid_amount: Lamports,
    pub cancelled: bool,
}

impl<'a> TryFrom<models::Bid<'a>> for Bid {
    type Error = std::num::TryFromIntError;

    fn try_from(
        models::Bid {
            listing_address,
            bidder_address,
            last_bid_time,
            last_bid_amount,
            cancelled,
            ..
        }: models::Bid,
    ) -> Result<Self, Self::Error> {
        Ok(Self {
            listing_address: listing_address.into_owned(),
            bidder_address: bidder_address.into_owned(),
            last_bid_time: last_bid_time.to_string(),
            last_bid_amount: last_bid_amount.try_into()?,
            cancelled,
        })
    }
}

#[graphql_object(Context = AppContext)]
impl Bid {
    pub fn listing_address(&self) -> String {
        self.listing_address.clone()
    }

    pub fn bidder_address(&self) -> String {
        self.bidder_address.clone()
    }

    pub fn last_bid_time(&self) -> String {
        self.last_bid_time.clone()
    }

    pub fn last_bid_amount(&self) -> Lamports {
        self.last_bid_amount
    }

    pub fn cancelled(&self) -> bool {
        self.cancelled
    }

    pub async fn listing(&self, ctx: &AppContext) -> FieldResult<Option<Listing>> {
        ctx.listing_loader
            .load(self.listing_address.clone().into())
            .await
            .map_err(Into::into)
    }
}

#[derive(Debug, Clone)]
pub struct Listing {
    pub address: String,
    pub store_address: String,
    pub ended: bool,
}

pub type ListingRow = (
    String,                // address
    String,                // store_address
    Option<NaiveDateTime>, // ends_at
    Option<i32>,           // gap_time
    Option<NaiveDateTime>, // last_bid_time
);

impl Listing {
    pub fn address((address, ..): &ListingRow) -> String {
        address.clone()
    }

    pub fn new(
        (address, store_address, ends_at, gap_time, last_bid_time): ListingRow,
        now: NaiveDateTime,
    ) -> Result<Self> {
        Ok(Self {
            address,
            store_address,
            ended: indexer_core::util::get_end_info(
                ends_at,
                gap_time.map(|i| chrono::Duration::seconds(i.into())),
                last_bid_time,
                now,
            )?
            .1,
        })
    }
}

#[graphql_object(Context = AppContext)]
impl Listing {
    pub fn address(&self) -> String {
        self.address.clone()
    }

    pub fn store_address(&self) -> String {
        self.store_address.clone()
    }

    pub fn ended(&self) -> bool {
        self.ended
    }

    pub async fn storefront(&self, ctx: &AppContext) -> FieldResult<Option<Storefront>> {
        ctx.storefront_loader
            .load(self.store_address.clone().into())
            .await
            .map_err(Into::into)
    }

    pub async fn nfts(&self, ctx: &AppContext) -> FieldResult<Vec<Nft>> {
        ctx.listing_nfts_loader
            .load(self.address.clone().into())
            .await
            .map_err(Into::into)
    }

    pub async fn bids(&self, ctx: &AppContext) -> FieldResult<Vec<Bid>> {
        ctx.listing_bids_loader
            .load(self.address.clone().into())
            .await
            .map_err(Into::into)
    }
}