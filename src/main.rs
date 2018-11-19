
//size of block (bytes) after which reward for block calculated using block size - second change, from v5
const CRYPTONOTE_BLOCK_GRANTED_FULL_REWARD_ZONE_V5: u128 = 300_000; /// get_min_block_weight
const DYNAMIC_FEE_REFERENCE_TRANSACTION_WEIGHT: u128 = 3000;

const DYNAMIC_FEE_PER_KB_BASE_BLOCK_REWARD: u128 = 10_000_000_000_000;

const PER_KB_FEE_QUANTIZATION_DECIMALS: u64 = 8;
const CRYPTONOTE_DISPLAY_DECIMAL_POINT: u64 = 9; /// it is 12 in monero

const DYNAMIC_FEE_PER_KB_BASE_FEE_V5 : u64 = 2_000_000_000;


#[allow(dead_code)]
fn get_fee_quantization_mask() -> u64
{
  let mut mask: u64= 1;

    for _i in PER_KB_FEE_QUANTIZATION_DECIMALS..CRYPTONOTE_DISPLAY_DECIMAL_POINT {
        mask *= 10;
    }

  return mask;
}

fn get_dynamic_base_fee_new(block_reward : u64, median_block_weight : u64) -> u64 {

    let mut mul = block_reward as u128 * DYNAMIC_FEE_REFERENCE_TRANSACTION_WEIGHT;
    let min_block_size = CRYPTONOTE_BLOCK_GRANTED_FULL_REWARD_ZONE_V5 as u128;

    println!("mul: {}", mul);

    mul = mul / min_block_size;
    mul = mul / median_block_weight as u128;

    return mul as u64 / 5;
}

fn get_dynamic_base_fee_old(block_reward : u64, median_block_weight : u64) -> u64 {

    let min_block_size = CRYPTONOTE_BLOCK_GRANTED_FULL_REWARD_ZONE_V5 as u64;
    let fee_per_kb_base = DYNAMIC_FEE_PER_KB_BASE_FEE_V5;

    let unscaled_fee_per_kb = fee_per_kb_base * min_block_size / median_block_weight;

    let scaled = unscaled_fee_per_kb as u128 * block_reward as u128;

    let scaled = scaled / DYNAMIC_FEE_PER_KB_BASE_BLOCK_REWARD;

    scaled as u64
}

fn main() {


    let block_reward = 100_000_000_000; /// 100 loki

    let median_block_weight = 300_000 as u64; /// no idea what this is


    println!("fee (hf >= 10): {}/KB", get_dynamic_base_fee_new(block_reward, median_block_weight) * 1024);

    println!("old fee: {}/KB", get_dynamic_base_fee_old(block_reward, median_block_weight));



}
