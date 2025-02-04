#![cfg(any(feature = "serde_support"))]

use monero::util::amount::{
    serde_impl::{SerdeAmount, SerdeAmountForOpt},
    SignedAmount,
};
use monero::Amount;
use serde::{Deserialize, Serialize};

#[test]
fn serde_impl_amount_and_signed_amount() {
    #[derive(Serialize, Deserialize)]
    pub struct HasAmount<T: SerdeAmountForOpt + SerdeAmount> {
        #[serde(with = "monero::util::amount::serde_impl::as_xmr")]
        pub xmr_amount: T,
        #[serde(with = "monero::util::amount::serde_impl::as_xmr::opt")]
        pub some_xmr_amount: Option<T>,
        #[serde(with = "monero::util::amount::serde_impl::as_pico")]
        pub pico_amount: T,
        #[serde(with = "monero::util::amount::serde_impl::as_pico::opt")]
        pub some_pico_amount: Option<T>,
    }

    let amt = Amount::ONE_PICO;
    let _t = HasAmount {
        xmr_amount: amt,
        some_xmr_amount: Some(amt),
        pico_amount: amt,
        some_pico_amount: Some(amt),
    };

    let amt = SignedAmount::ONE_PICO;
    let _t = HasAmount {
        xmr_amount: amt,
        some_xmr_amount: Some(amt),
        pico_amount: amt,
        some_pico_amount: Some(amt),
    };
}
