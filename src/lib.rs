pub mod account_meta_for_swap;
pub mod accounts;
pub mod amm;
pub mod constants;
pub mod pda;

use anchor_lang::prelude::*;

//bounding curve v1
declare_id!("E55ogQGmvvBKtZeGHaNoCASCBAV4rFFedCMzhku2YTqz");

// In production we will be using this bounding curve v1 will be desecrated in few months.
// If it's possible to support both then it will be good to support both.

//bounding curve v2
// declare_id!("Anpp23e2Rwh4tmQxDS9x9DzfyVmJQ1EdrCecCgvXfjPC");