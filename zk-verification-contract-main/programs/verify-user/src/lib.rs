use anchor_lang::prelude::*;
use ark_ff::bytes::{FromBytes, ToBytes};
use groth16_solana::groth16::{Groth16Verifier, Groth16Verifyingkey};
use std::ops::Neg;
pub mod errors;
use errors::CustomError;

declare_id!("27hymAk2PCtdYHhMiwwEYiBvs2gbYZeyjNA7x1nT54Hw");

pub const VERIFYINGKEY: Groth16Verifyingkey = Groth16Verifyingkey {
    nr_pubinputs: 3,

    vk_alpha_g1: [
        45, 77, 154, 167, 227, 2, 217, 223, 65, 116, 157, 85, 7, 148, 157, 5, 219, 234, 51, 251,
        177, 108, 100, 59, 34, 245, 153, 162, 190, 109, 242, 226, 20, 190, 221, 80, 60, 55, 206,
        176, 97, 216, 236, 96, 32, 159, 227, 69, 206, 137, 131, 10, 25, 35, 3, 1, 240, 118, 202,
        255, 0, 77, 25, 38,
    ],

    vk_beta_g2: [
        9, 103, 3, 47, 203, 247, 118, 209, 175, 201, 133, 248, 136, 119, 241, 130, 211, 132, 128,
        166, 83, 242, 222, 202, 169, 121, 76, 188, 59, 243, 6, 12, 14, 24, 120, 71, 173, 76, 121,
        131, 116, 208, 214, 115, 43, 245, 1, 132, 125, 214, 139, 192, 224, 113, 36, 30, 2, 19, 188,
        127, 193, 61, 183, 171, 48, 76, 251, 209, 224, 138, 112, 74, 153, 245, 232, 71, 217, 63,
        140, 60, 170, 253, 222, 196, 107, 122, 13, 55, 157, 166, 154, 77, 17, 35, 70, 167, 23, 57,
        193, 177, 164, 87, 168, 199, 49, 49, 35, 210, 77, 47, 145, 146, 248, 150, 183, 198, 62,
        234, 5, 169, 213, 127, 6, 84, 122, 208, 206, 200,
    ],

    vk_gamme_g2: [
        25, 142, 147, 147, 146, 13, 72, 58, 114, 96, 191, 183, 49, 251, 93, 37, 241, 170, 73, 51,
        53, 169, 231, 18, 151, 228, 133, 183, 174, 243, 18, 194, 24, 0, 222, 239, 18, 31, 30, 118,
        66, 106, 0, 102, 94, 92, 68, 121, 103, 67, 34, 212, 247, 94, 218, 221, 70, 222, 189, 92,
        217, 146, 246, 237, 9, 6, 137, 208, 88, 95, 240, 117, 236, 158, 153, 173, 105, 12, 51, 149,
        188, 75, 49, 51, 112, 179, 142, 243, 85, 172, 218, 220, 209, 34, 151, 91, 18, 200, 94, 165,
        219, 140, 109, 235, 74, 171, 113, 128, 141, 203, 64, 143, 227, 209, 231, 105, 12, 67, 211,
        123, 76, 230, 204, 1, 102, 250, 125, 170,
    ],

    vk_delta_g2: [
        2, 133, 88, 175, 107, 158, 196, 4, 8, 152, 87, 171, 1, 49, 152, 46, 17, 142, 150, 43, 242,
        196, 11, 60, 117, 214, 136, 86, 236, 194, 166, 69, 0, 128, 249, 28, 246, 67, 157, 48, 72,
        121, 172, 142, 86, 113, 150, 192, 16, 38, 251, 72, 118, 246, 135, 39, 126, 207, 103, 53,
        239, 192, 172, 79, 4, 166, 166, 238, 197, 202, 137, 144, 211, 3, 102, 173, 83, 86, 84, 1,
        253, 223, 38, 208, 74, 156, 125, 150, 50, 61, 50, 163, 128, 175, 54, 194, 31, 188, 6, 144,
        210, 71, 113, 64, 174, 220, 248, 73, 143, 162, 168, 57, 9, 126, 210, 192, 81, 221, 222,
        227, 225, 16, 28, 100, 23, 211, 201, 22,
    ],

    vk_ic: &[
        [
            11, 229, 231, 98, 125, 167, 192, 162, 213, 195, 70, 127, 180, 182, 126, 53, 78, 168,
            70, 6, 73, 16, 100, 93, 245, 27, 174, 126, 111, 35, 40, 91, 34, 236, 28, 78, 0, 59, 8,
            212, 240, 82, 102, 144, 226, 88, 142, 201, 130, 216, 189, 224, 106, 203, 99, 75, 130,
            4, 68, 196, 190, 40, 6, 186,
        ],
        [
            40, 136, 56, 37, 224, 232, 168, 142, 229, 118, 118, 80, 212, 180, 246, 38, 19, 147,
            119, 77, 80, 17, 229, 183, 115, 204, 77, 210, 52, 158, 48, 109, 21, 75, 192, 25, 187,
            222, 245, 12, 71, 211, 15, 42, 178, 63, 170, 252, 85, 82, 181, 79, 172, 87, 194, 51, 3,
            24, 141, 108, 238, 118, 147, 235,
        ],
        [
            8, 86, 193, 69, 239, 92, 209, 218, 130, 154, 203, 67, 116, 227, 63, 1, 201, 84, 151,
            100, 136, 124, 100, 122, 130, 253, 11, 223, 181, 149, 78, 127, 16, 35, 66, 162, 102,
            13, 47, 6, 59, 58, 213, 170, 0, 31, 159, 101, 248, 193, 176, 223, 148, 174, 197, 140,
            141, 113, 8, 155, 83, 42, 209, 215,
        ],
    ],
};
#[program]
pub mod zk_user_verification {
    use super::*;

    pub fn initialize_main_info(
        ctx: Context<InitializeMainInfo>,
        access_type: String,
        file_name: String,
    ) -> Result<()> {
        let main_info = &mut ctx.accounts.main_info;
        main_info.admin = ctx.accounts.admin.key(); // Set the admin's public key
        main_info.access_type = access_type;
        main_info.file_name = file_name;

        // Set default allowed time range (e.g., 6 PM to 10 PM UTC)
        main_info.start_hour = 18;
        main_info.end_hour = 22;

        Ok(())
    }

    pub fn set_allowed_time_range(
        ctx: Context<SetAllowedTimeRange>,
        start_hour: u8,
        end_hour: u8,
    ) -> Result<()> {
        let main_info = &mut ctx.accounts.main_info;

        // Validate the time range
        require!(
            start_hour < 24 && end_hour <= 24 && start_hour < end_hour,
            CustomError::InvalidTimeRange
        );

        main_info.start_hour = start_hour;
        main_info.end_hour = end_hour;

        Ok(())
    }

    pub fn verify_user(
        ctx: Context<VerifyUser>,
        proof: UserProof,
        access_type: String,
        file_name: String,
    ) -> Result<()> {
        let main_info = &mut ctx.accounts.main_info;

        // Validate the file name
        require!(
            main_info.file_name == file_name,
            CustomError::InvalidFileName
        );

        // Validate the access type
        require!(
            main_info.access_type == access_type,
            CustomError::InvalidAccessType
        );

        // Get the current timestamp
        let clock = Clock::get()?;
        let current_timestamp = clock.unix_timestamp;

        // Convert the timestamp to hours (assuming UTC)
        let current_hour  = ((current_timestamp % 86400) / 3600) as u8; // 86400 seconds in a day, 3600 seconds in an hour

        // Define allowed hours (6 PM to 10 PM in UTC)
        let start_hour = main_info.start_hour; // 6 PM
        let end_hour = main_info.end_hour; // 10 PM

        // Check if the current hour is within the allowed range
        require!(
            current_hour >= start_hour && current_hour < end_hour,
            CustomError::InvalidTime
        );

        // Verify the User proof
        require!(main_info.verify_user(proof), CustomError::VerificationFailed);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeMainInfo<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + 32 + 32 + 1 + 1 + 1,
        seeds = [b"maininfo", admin.key().as_ref()],
        bump
    )]
    pub main_info: Account<'info, MainInfo>,
    #[account(mut)]
    pub admin: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct SetAllowedTimeRange<'info> {
    #[account(
        mut,
        seeds = [b"maininfo", admin.key().as_ref()],
        bump,
        has_one = admin // Ensure the admin matches the stored admin in the main_info account
    )]
    pub main_info: Account<'info, MainInfo>,
    /// CHECK:
    #[account(signer)]
    pub admin: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct VerifyUser<'info> {
    #[account(
        mut,
        seeds = [b"maininfo", main_info.admin.as_ref()],
        bump
    )]
    pub main_info: Account<'info, MainInfo>,
    pub user: Signer<'info>,
}

#[account]
pub struct MainInfo {
    admin: Pubkey, // Admin's public key,
    access_type : String, // Access type (e.g., "read", "write")
    // The file name to be verified
    // This should be a unique identifier for the file
    file_name: String,
    start_hour: u8, // Allowed start hour (in UTC)
    end_hour: u8,   // Allowed end hour (in UTC)
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub struct UserProof {
    proof_a: [u8; 64],
    proof_b: [u8; 128],
    proof_c: [u8; 64],
    public_signals: [u8; 64], // 3 public signals, each 256 bytes
}

impl MainInfo {
    pub fn verify_user(&self, proof: UserProof) -> bool {
        type G1 = ark_ec::short_weierstrass_jacobian::GroupAffine<ark_bn254::g1::Parameters>;

        fn change_endianness(bytes: &[u8]) -> Vec<u8> {
            let mut vec = Vec::new();
            for b in bytes.chunks(32) {
                for byte in b.iter().rev() {
                    vec.push(*byte);
                }
            }
            vec
        }

        let proof_a_neg_g1: G1 = <G1 as FromBytes>::read(
            &*[&change_endianness(proof.proof_a.as_slice())[..], &[0u8][..]].concat(),
        )
        .unwrap();
        let mut proof_a_neg = [0u8; 65];
        <G1 as ToBytes>::write(&proof_a_neg_g1.neg(), &mut proof_a_neg[..]).unwrap();
        let proof_a_neg = change_endianness(&proof_a_neg[..64]).try_into().unwrap();

        let mut public_inputs_vec = Vec::new();
        for input in proof.public_signals.chunks(32) {
            public_inputs_vec.push(input);
        }

        // Convert the byte slice into a Groth16Verifyingkey
        let mut verifier = Groth16Verifier::new(
            &proof_a_neg,
            &proof.proof_b,
            &proof.proof_c,
            &public_inputs_vec.as_slice(),
            &VERIFYINGKEY,
        )
        .unwrap();
        verifier.verify().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anchor_lang::prelude::*;
    use anchor_lang::solana_program::pubkey::Pubkey;

    #[test]
    fn test_initialize_main_info() {
        // Simulate the admin's public key
        let admin_pubkey = Pubkey::new_unique();

        // Simulate the PDA for the main_info account
        let (main_info_pda, _bump) =
            Pubkey::find_program_address(&[b"maininfo", admin_pubkey.as_ref()], &crate::ID);

        // Create a new main_info instance
        let mut main_info = MainInfo {
            admin: admin_pubkey,
            access_type: String::from("read"),
            file_name: String::new(),
            start_hour: 18,
            end_hour: 22,
        };

        // Simulate the initialization
        let file_name = String::from("REG123456789");
        let user_public_key = Pubkey::new_unique();
        main_info.file_name = file_name.clone();

        // Assertions
        assert_eq!(main_info.admin, admin_pubkey);
        assert_eq!(main_info.file_name, file_name);
        assert_eq!(main_info.start_hour, 18);
        assert_eq!(main_info.end_hour, 22);
    }

    #[test]
    fn test_set_allowed_time_range() {
        // Simulate the admin's public key
        let admin_pubkey = Pubkey::new_unique();

        // Simulate the PDA for the main_info account
        let (main_info_pda, _bump) =
            Pubkey::find_program_address(&[b"maininfo", admin_pubkey.as_ref()], &crate::ID);

        // Create a new main_info instance
        let mut main_info = MainInfo {
            admin: admin_pubkey,
            access_type: String::from("read"),
            file_name: String::new(),
            start_hour: 18,
            end_hour: 22,
        };

        // Simulate setting a new allowed time range
        let new_start_hour = 10;
        let new_end_hour = 20;

        // Validate the time range
        assert!(new_start_hour < 24 && new_end_hour <= 24 && new_start_hour < new_end_hour);

        main_info.start_hour = new_start_hour;
        main_info.end_hour = new_end_hour;

        // Assertions
        assert_eq!(main_info.start_hour, new_start_hour);
        assert_eq!(main_info.end_hour, new_end_hour);
    }

    #[test]
    fn test_verify_user() {
        // Simulate the admin's public key
        let admin_pubkey = Pubkey::new_unique();

        // Simulate the PDA for the main_info account
        let (main_info_pda, _bump) =
            Pubkey::find_program_address(&[b"maininfo", admin_pubkey.as_ref()], &crate::ID);

        // Create a new main_info instance
        let mut main_info = MainInfo {
            admin: admin_pubkey,
            access_type: String::from("read"),
            file_name: String::from("REG123456789"),
            start_hour: 18,
            end_hour: 22,
        };

        // Mock UserProof
        let proof = UserProof {
            proof_a: [
                46, 64, 147, 27, 134, 254, 129, 25, 207, 225, 244, 224, 54, 71, 146, 36, 5, 153,
                227, 39, 157, 89, 89, 229, 227, 99, 183, 238, 175, 136, 220, 232, 37, 50, 49, 16,
                55, 81, 54, 196, 45, 219, 195, 208, 101, 122, 106, 18, 245, 122, 72, 145, 117, 147,
                179, 172, 15, 99, 61, 148, 62, 204, 184, 148,
            ],
            proof_b: [
                0, 227, 221, 17, 11, 47, 182, 18, 121, 146, 232, 44, 178, 147, 37, 119, 83, 198,
                24, 200, 239, 208, 127, 145, 153, 123, 231, 194, 159, 210, 22, 248, 43, 206, 2,
                134, 7, 81, 130, 75, 150, 213, 132, 27, 161, 151, 2, 130, 201, 28, 231, 155, 148,
                167, 151, 207, 197, 251, 79, 209, 28, 232, 65, 221, 4, 5, 108, 101, 218, 84, 254,
                227, 67, 193, 154, 63, 31, 136, 98, 221, 227, 239, 38, 166, 32, 43, 220, 14, 199,
                80, 188, 75, 221, 21, 95, 235, 31, 95, 181, 228, 72, 14, 120, 253, 201, 202, 40,
                80, 180, 65, 118, 36, 96, 220, 179, 242, 181, 44, 23, 9, 15, 25, 36, 165, 204, 185,
                158, 97,
            ],
            proof_c: [
                36, 41, 23, 168, 87, 235, 67, 54, 246, 52, 34, 108, 130, 117, 81, 155, 199, 5, 131,
                93, 219, 139, 130, 73, 105, 248, 89, 192, 236, 17, 223, 112, 30, 29, 200, 157, 156,
                200, 16, 200, 236, 168, 56, 56, 85, 222, 215, 55, 244, 152, 74, 238, 54, 143, 107,
                203, 150, 177, 212, 148, 47, 56, 171, 15,
            ],
            public_signals: [
                26, 243, 92, 199, 103, 37, 2, 134, 123, 28, 203, 47, 101, 42, 42, 208, 90, 112, 79,
                72, 106, 216, 234, 136, 249, 132, 188, 208, 205, 118, 167, 122, 6, 225, 175, 42,
                21, 213, 158, 237, 35, 232, 14, 139, 151, 132, 118, 4, 218, 34, 79, 86, 226, 189,
                105, 200, 7, 16, 168, 106, 131, 198, 21, 141,
            ],
        };

        // Simulate the current time within the allowed range
        let current_hour = 19; // 7 PM UTC
        assert!(
            current_hour >= main_info.start_hour as i64 && current_hour < main_info.end_hour as i64
        );

        // Simulate verification with the correct registration number
        let file_name = String::from("REG123456789");
        assert_eq!(
            main_info.file_name, file_name,
            "Registration number does not match"
        );

        // Simulate verification
        let result = main_info.verify_user(proof);

        // Assertions
        assert!(result, "User verification failed");
    }
}
