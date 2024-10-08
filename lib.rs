use anchor_lang::prelude::*;

declare_id!("HH18xCQFFpoMscFJdTQEoRHqpmnUbb2Jxnr2BWbBqhCD");

#[program]
pub mod certificate_verification {
 use super::*;

 // Initialize the contract and set the admin
 pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
 let state = &mut ctx.accounts.state;
 state.admin = *ctx.accounts.admin.key;
 Ok(())
 }

 // Admin registers an institute
 pub fn register_institute(
 ctx: Context<RegisterInstitute>, 
 _institute_name: String, 
 _acronym: String, 
 _witness: Pubkey
 ) -> Result<()> {
 let state = &mut ctx.accounts.state;
 require!(ctx.accounts.admin.key == &state.admin, CustomError::Unauthorized);
 
 let institute = Institute {
 name: _institute_name,
 acronym: _acronym,
 institute_address: _witness,
 };
 state.institutes.push(institute);
 Ok(())
 }

 // Registered institutes upload certificates
pub fn post_certificate(
 ctx: Context<PostCertificate>, 
 student_name: String, 
 student_address: Pubkey, 
 uri: String, 
 hash: String, 
 certificate_type: String, 
 issuer_name: String
) -> Result<()> {
 let state = &mut ctx.accounts.state;

 // Find the institute and clone necessary data
 let institute = state
 .institutes
 .iter()
 .find(|i| i.institute_address == *ctx.accounts.institute.key)
 .ok_or(CustomError::Unauthorized)?;
 let institute_name = institute.name.clone();
 let institute_address = institute.institute_address;

 // Get the current block timestamp
 let clock = Clock::get().unwrap();
 let current_timestamp = clock.unix_timestamp;

 // Create the certificate
 let certificate = Certificate {
 student_name: student_name.clone(),
 student_address,
 college_name: institute_name.clone(),
 hash,
 url: uri.clone(),
 certificate_type,
 issuer_name: issuer_name.clone(),
 witness_address: institute_address,
 timestamp: current_timestamp,
 };

 // Add the certificate to the state's certificate array
 state.certificates.push(certificate);

 // Update or create student details
 if let Some(student) = state.students.iter_mut().find(|s| s.address == student_address) {
 student.institute_names.push(institute_name);
 student.uris.push(uri);
 } else {
 let new_student = Student {
 name: student_name,
 address: student_address,
 institute_names: vec![institute_name],
 uris: vec![uri],
 };
 state.students.push(new_student);
 }

 Ok(())
}
 // Admin or Institute can retrieve institute details
 pub fn get_institute_details(
 ctx: Context<GetInstituteDetails>,
 ) -> Result<()> {
 let state = &ctx.accounts.state;

 // Either the admin or the institute can fetch the details
 let is_admin = ctx.accounts.admin.key == &state.admin;
 let institute = state.institutes
 .iter()
 .find(|i| i.institute_address == *ctx.accounts.institute.key || is_admin);

 // Ensure the institute exists
 require!(institute.is_some(), CustomError::InstituteNotFound);

 // Emit an event with the institute details
 emit!(InstituteDetailsRetrieved {
 name: institute.unwrap().name.clone(),
 acronym: institute.unwrap().acronym.clone(),
 institute_address: institute.unwrap().institute_address,
 });

 Ok(())
 }

 // Student can retrieve their certificate details by signing with their address
 pub fn get_student_details(
 ctx: Context<GetStudentCertificateDetails>,
 ) -> Result<()> {
 let state = &ctx.accounts.state;

 // Find certificates associated with the student's address
 let certificates: Vec<&Certificate> = state.certificates
 .iter()
 .filter(|c| c.student_address == *ctx.accounts.student.key)
 .collect();

 // Ensure the student has certificates
 require!(!certificates.is_empty(), CustomError::CertificateNotFound);

 // Emit an event with the student's details and certificates
 for certificate in certificates {
 emit!(StudentDetailsRetrieved {
 student_name: certificate.student_name.clone(),
 student_address: certificate.student_address,
 college_name: certificate.college_name.clone(),
 hash: certificate.hash.clone(),
 url: certificate.url.clone(),
 certificate_type: certificate.certificate_type.clone(),
 issuer_name: certificate.issuer_name.clone(), // Include issuer name in event
 });
 }

 Ok(())
 }

 // Admin retrieves a list of all registered institutes
 pub fn list_institutes(ctx: Context<ListInstitutes>) -> Result<()> {
 let state = &ctx.accounts.state;
 require!(ctx.accounts.admin.key == &state.admin, CustomError::Unauthorized);

 // Emit an event containing the list of institutes
 emit!(InstitutesListed {
 institutes: state.institutes.clone(),
 });

 Ok(())
 }
 pub fn bulk_upload(
 ctx: Context<BulkUpload>,
 data: Vec<BulkUploadData>,
 issuer_name: String
) -> Result<()> {
 let state = &mut ctx.accounts.state;

 // Find the institute
 let institute = state
 .institutes
 .iter()
 .find(|i| i.institute_address == *ctx.accounts.institute.key)
 .ok_or(CustomError::Unauthorized)?;
 let institute_name = institute.name.clone();

 // Limit the number of certificates in a single bulk upload
 require!(data.len() <= 100, CustomError::TupleSizeExceeded);

 let mut failed_uploads = Vec::new();

 for item in data.iter() {
 // Check if the certificate already exists
 if state.certificates.iter().any(|c| c.hash == item.hash) {
 failed_uploads.push(item.student_name.clone());
 continue;
 }

 // Get the current block timestamp
 let clock = Clock::get().unwrap();
 let current_timestamp = clock.unix_timestamp;

 // Create the certificate
 let certificate = Certificate {
 student_name: item.student_name.clone(),
 student_address: item.student_address,
 college_name: institute_name.clone(),
 hash: item.hash.clone(),
 url: item.uri.clone(),
 certificate_type: item.certificate_type.clone(),
 issuer_name: issuer_name.clone(),
 witness_address: *ctx.accounts.institute.key,
 timestamp: current_timestamp,
 };

 // Add the certificate to the state's certificate array
 state.certificates.push(certificate);

 // Update or create student details
 if let Some(student) = state.students.iter_mut().find(|s| s.address == item.student_address) {
 student.institute_names.push(institute_name.clone());
 student.uris.push(item.uri.clone());
 } else {
 let new_student = Student {
 name: item.student_name.clone(),
 address: item.student_address,
 institute_names: vec![institute_name.clone()],
 uris: vec![item.uri.clone()],
 };
 state.students.push(new_student);
 }

 // Emit an event for successful certificate upload
 emit!(CertificatePosted {
 hash: item.hash.clone(),
 institute_address: *ctx.accounts.institute.key,
 student_name: item.student_name.clone(),
 issuer_name: issuer_name.clone(),
 });
 }

 // Emit an event for failed uploads if there are any
 if !failed_uploads.is_empty() {
 emit!(BulkUploadFailed {
 failed_uploads: failed_uploads.clone(), // Clone here
 failed_count: failed_uploads.len() as u64,
 });
 }

 Ok(())
}
}

// State to hold admin, institutes, certificates, and students
#[account]
pub struct State {
 pub admin: Pubkey,
 pub institutes: Vec<Institute>,
 pub certificates: Vec<Certificate>,
 pub students: Vec<Student>, // New array for students
}

// Institute structure
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Institute {
 pub name: String,
 pub acronym: String,
 pub institute_address: Pubkey,
}

// Certificate structure
#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Certificate {
 pub student_name: String,
 pub student_address: Pubkey, 
 pub college_name: String,
 pub hash: String,
 pub url: String,
 pub certificate_type: String,
 pub issuer_name: String, // Issuer name field
 pub witness_address: Pubkey, // Witness (institute) address field
 pub timestamp: i64, // Block timestamp
}

// Student structure

#[derive(Clone, AnchorSerialize, AnchorDeserialize)]
pub struct Student {
 pub name: String,
 pub address: Pubkey,
 pub institute_names: Vec<String>, // Array of institute names
 pub uris: Vec<String>, // Array of URIs for the certificates
}

// Initialize contract
#[derive(Accounts)]
pub struct Initialize<'info> {
 #[account(init, payer = admin, space = 8 + 32 + (100 * 64))]
 pub state: Account<'info, State>,
 #[account(mut)]
 pub admin: Signer<'info>,
 pub system_program: Program<'info, System>,
}

// Register institute
#[derive(Accounts)]
pub struct RegisterInstitute<'info> {
 #[account(mut)]
 pub state: Account<'info, State>,
 pub admin: Signer<'info>,
}

// Post certificate
#[derive(Accounts)]
pub struct PostCertificate<'info> {
 #[account(mut)]
 pub state: Account<'info, State>,
 pub institute: Signer<'info>, // Institute's signature required
}

// Get institute details
#[derive(Accounts)]
pub struct GetInstituteDetails<'info> {
 pub state: Account<'info, State>,
 pub institute: Signer<'info>, // Institute's signature required
 #[account(mut)]
 pub admin: Signer<'info>, // Admin can also fetch details
}

// Get student details
#[derive(Accounts)]
pub struct GetStudentCertificateDetails<'info> {
 pub state: Account<'info, State>,
 pub student: Signer<'info>, // Student's signature required
}

// List institutes
#[derive(Accounts)]
pub struct ListInstitutes<'info> {
 #[account(mut)]
 pub state: Account<'info, State>,
 pub admin: Signer<'info>,
}

// Event for listing institutes
#[event]
pub struct InstitutesListed {
 pub institutes: Vec<Institute>,
}

// Event for institute details retrieval
#[event]
pub struct InstituteDetailsRetrieved {
 pub name: String,
 pub acronym: String,
 pub institute_address: Pubkey,
}

// Event for student details retrieval
#[event]
pub struct StudentDetailsRetrieved {
 pub student_name: String,
 pub student_address: Pubkey,
 pub college_name: String,
 pub hash: String,
 pub url: String,
 pub certificate_type: String,
 pub issuer_name: String, // Include issuer name in event
}
#[derive(Accounts)]
pub struct BulkUpload<'info> {
 #[account(mut)]
 pub state: Account<'info, State>,
 pub institute: Signer<'info>, // Institute's signature required
}

// Bulk upload data structure
#[derive(AnchorSerialize, AnchorDeserialize, Clone)]
pub struct BulkUploadData {
 pub student_name: String,
 pub student_address: Pubkey,
 pub hash: String,
 pub uri: String,
 pub certificate_type: String,
}

// Event for successful certificate upload
#[event]
pub struct CertificatePosted {
 pub hash: String,
 pub institute_address: Pubkey,
 pub student_name: String,
 pub issuer_name: String,
}

// Event for failed bulk uploads
#[event]
pub struct BulkUploadFailed {
 pub failed_uploads: Vec<String>,
 pub failed_count: u64,
}
// Error codes
#[error_code]
pub enum CustomError {
 Unauthorized,
 InstituteNotFound,
 CertificateNotFound,
 TupleSizeExceeded,
}
