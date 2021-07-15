use solana_sdk::pubkey::{ Pubkey, read_pubkey_file, write_pubkey_file };
use solana_sdk::instruction::{ Instruction };
use solana_sdk::transaction::{ Transaction };
use solana_sdk::signer::{ Signer };
use solana_sdk::signer::keypair::{ Keypair, read_keypair_file };
use solana_sdk::program_pack::{ Pack };
use spl_token::state::{ Mint, Account };

use solana_client::rpc_client::{ RpcClient };


const SOLANA_CLIENT_URL: &'static str = "https://api.devnet.solana.com";
const WALLET_FILE_PATH: &'static str = "/XXXXX/id.json";

const SPECIAL_TOKEN_FILENAME: &'static str = "special_token.pubkey";
const TOKEN_ACCOUNT_FILENAME: &'static str = "token_account.pubkey";

fn main() {

    let client = RpcClient::new(SOLANA_CLIENT_URL.to_string());

    let wallet_keypair = read_keypair_file(WALLET_FILE_PATH).unwrap();
    let wallet_pubkey: Pubkey = wallet_keypair.pubkey();
    println!("Wallet Pubkey: {}", wallet_pubkey);
    println!("Wallet Balance: {}", client.get_balance(&wallet_pubkey).unwrap());

    let special_token_pubkey: Pubkey =
        if let Ok(pubkey) = read_pubkey_file(SPECIAL_TOKEN_FILENAME) {
            pubkey
        } else {
            // Create new Mint
            let mint_account: Keypair = Keypair::new();
            let mint_account_pubkey = mint_account.pubkey();
            println!("Special Token Mint: {}", mint_account_pubkey);

            let minimum_balance_for_rent_exemption = client.get_minimum_balance_for_rent_exemption(Mint::LEN).unwrap();

            let create_account_instruction: Instruction =
                solana_sdk::system_instruction::create_account(
                    &wallet_pubkey,
                    &mint_account_pubkey,
                    minimum_balance_for_rent_exemption,
                    Mint::LEN as u64,
                    &spl_token::id(),
                );
            let initialize_mint_instruction: Instruction =
                spl_token::instruction::initialize_mint(
                    &spl_token::id(),
                    &mint_account_pubkey,
                    &wallet_pubkey,
                    None,
                    9,
                ).unwrap();

            let (recent_blockhash, _fee_calculator) = client.get_recent_blockhash().unwrap();

            let transaction: Transaction =
                Transaction::new_signed_with_payer(
                    &vec![
                        create_account_instruction,
                        initialize_mint_instruction,
                    ],
                    Some(&wallet_pubkey),
                    &[
                        &mint_account,
                        &wallet_keypair,
                    ],
                    recent_blockhash,
                );
            
            let result = client.send_transaction(&transaction);
            println!("'Create Account & Init Mint' Transaction Result: {:?}", result);

            if result.is_ok() {
                write_pubkey_file(SPECIAL_TOKEN_FILENAME,mint_account_pubkey).unwrap();
            }
            return;

            // mint_account_pubkey
        };

    let token_account_pubkey: Pubkey =
        if let Ok(pubkey) = read_pubkey_file(TOKEN_ACCOUNT_FILENAME) {
            pubkey
        } else {
            // If don't have Token Account mint to then create it
            let account_mint_to: Keypair = Keypair::new();
            let account_mint_to_pubkey: Pubkey = account_mint_to.pubkey();
            println!("New Account Mint To: {}", account_mint_to_pubkey);

            let create_account_instruction: Instruction =
                solana_sdk::system_instruction::create_account(
                    &wallet_pubkey,
                    &account_mint_to_pubkey,
                    client.get_minimum_balance_for_rent_exemption(Account::LEN).unwrap(),
                    Account::LEN as u64,
                    &spl_token::id(),
                );
            let initialize_account2_instruction: Instruction =
                spl_token::instruction::initialize_account2(
                    &spl_token::id(),
                    &account_mint_to_pubkey,
                    &special_token_pubkey,
                    &wallet_pubkey,
                ).unwrap();

            let (recent_blockhash, _fee_calculator) = client.get_recent_blockhash().unwrap();

            let transaction: Transaction =
                Transaction::new_signed_with_payer(
                    &vec![
                        create_account_instruction,
                        initialize_account2_instruction,
                    ],
                    Some(&wallet_pubkey),
                    &[
                        &wallet_keypair,
                        &account_mint_to,
                    ],
                    recent_blockhash,
                );

            let result = client.send_transaction(&transaction);
            println!("'Create Account' Transaction Result: {:?}", result);

            if result.is_ok() {
                write_pubkey_file(TOKEN_ACCOUNT_FILENAME,account_mint_to_pubkey).unwrap();
            }
            return;

            // account_mint_to_pubkey
        };
        
    // Mint some tokens
    let mint_to_instruction: Instruction =
        spl_token::instruction::mint_to(
            &spl_token::id(),
            &special_token_pubkey,
            &token_account_pubkey,
            &wallet_pubkey,
            &[&wallet_pubkey],
            2000,
        ).unwrap();
    let _mint_to_checked_instruction: Instruction =
        spl_token::instruction::mint_to_checked(
            &spl_token::id(),
            &special_token_pubkey,
            &token_account_pubkey,
            &wallet_pubkey,
            &[&wallet_pubkey],
            1000,
            9,
        ).unwrap();

    let (recent_blockhash, _fee_calculator) = client.get_recent_blockhash().unwrap();

    let transaction: Transaction =
        Transaction::new_signed_with_payer(
            &vec![
                mint_to_instruction,
            ],
            Some(&wallet_pubkey),
            &[
                &wallet_keypair,
            ],
            recent_blockhash,
        );

    let result = client.send_transaction(&transaction);
    println!("'Mint To' Transaction Result: {:?}", result);

}
