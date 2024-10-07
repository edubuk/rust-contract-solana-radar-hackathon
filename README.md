# Solana-Radar-Hackathon
## Rust-Smart-Contract deployed on Solana Blockchain (Devnet)

### *EduBukSealer: A Solana Program for Certificate Management*

EduBukSealer is a smart contract deployed on the Solana blockchain that enables educational institutes to register themselves and issue certificates to students in a decentralized manner. This program is built using the Anchor framework for Solana smart contracts and provides functionalities to manage institutions, update witnesses, and securely issue certificates.

### *Overview*
- EduBukSealer allows educational institutions to:
- Register Certificates (Academic and WOrk-Experience) on the Solana blockchain.
- Designate witnesses for issuing certificates.
- Post certificates for students securely with metadata (e.g., student name, certificate type, issuer, etc.).
- The contract manages state using on-chain accounts and employs Solana's native security model to ensure that only authorized entities can register institutes, update witnesses, and issue certificates.

### *Getting Started*
To deploy and interact with this program, you need:

- Rust and the Rust toolchain installed.
- Anchor framework for building and deploying the Solana programs.
- A Solana CLI installation to interact with the blockchain.
- A Solana wallet to cover transaction fees and to store program accounts.

### *Program Architecture*
- The program consists of several instructions that allow interaction with on-chain accounts representing state, institutes, and certificates. The key elements include:
- ContractState: Holds the global state of the program, including the owner and the institute ID counter.
- Institute: Represents a registered educational institute with fields such as institute name, acronym, and the current witness.
- Certificate: Represents a student's certificate issued by an institute.

### *Instruction Functions*
#### *1. initialize*
Sets up the contract by creating the State account and assigning the admin.
- Parameters:
ctx: Context with the state and admin accounts.
- Usage:
Should be called once by the contract deployer to initialize the program.

#### *2. register_institute*
Registers a new educational institute. Only the admin can call this function.
##### Parameters:
- ctx: Context with state and admin accounts.
- _institute_name: Name of the institute.
- _acronym: Acronym of the institute.
- _witness: Public key of the witness (institute's address).
##### Usage:
- Appends the new institute to the state's list of institutes.

#### *3. post_certificate*
Allows a registered institute to upload a certificate for a student.
##### Parameters:
- ctx: Context with state and institute accounts.
- student_name, student_address, uri, hash, certificate_type, issuer_name: Certificate metadata.
##### Usage:
- Adds the certificate to the state's certificates list.
- Updates or creates the student's details in the state's student array.

#### *4. get_institute_details*
Retrieves details of an institute. Can be called by the admin or the institute itself.
##### Parameters:
- ctx: Context with state and institute accounts.
##### Usage:
- Emits an event with the institute's details.

#### *5. get_student_details*
Allows a student to retrieve their certificates. Only the student can call this function.
##### Parameters:
- ctx: Context with state and student accounts.
##### Usage:
- Emits an event for each certificate owned by the student.

#### *6. list_institutes*
Allows the admin to retrieve a list of all registered institutes.
##### Parameters:
- ctx: Context with state and admin accounts.
##### Usage:
- Emits an event containing the list of all institutes.

#### *7. bulk_upload*
Enables an institute to upload multiple certificates in one transaction.
##### Parameters:
- ctx: Context with state and institute accounts.
- data: Vector of BulkUploadData containing the certificate information.
- issuer_name: Name of the issuer.
##### Usage:
- Adds multiple certificates to the state's certificate list.
- Updates student details and emits events for successful uploads.
- Emits a separate event for failed uploads if any.

### *Account Structures*
#### 1. State
Holds the global state of the program.
##### Fields:
- admin: Public key of the admin.
- institutes: Vector of registered Institutes.
- certificates: Vector of issued Certificates.
- students: Vector of Student details.

#### 2. Institute
Stores information about an institute.
##### Fields:
- name: Name of the institute.
- acronym: Acronym.
- institute_address: Public key of the institute.

#### 3. Certificate
Represents a certificate issued to a student.
##### Fields:
- student_name,
- student_address,
- college_name,
- hash,
- url,
- certificate_type,
- issuer_name,
- witness_address,
- timestamp.

#### 4. Student
Stores details of a student.
##### Fields:
- name: Name of the student.
- address: Public key of the student.
- institute_names: Vector of institutes where the student received certificates.
- uris: Vector of URIs to the certificates.

### *Events*
- InstitutesListed: Emits the list of all registered institutes.
- InstituteDetailsRetrieved: Emits the details of a specific institute.
- StudentDetailsRetrieved: Emits details of a student's certificates.
- CertificatePosted: Emits when a certificate is successfully uploaded.
- BulkUploadFailed: Emits if any certificate upload fails during bulk upload.

#### *Error Codes*
- Unauthorized: Action is not authorized.
- InstituteNotFound: Institute is not registered.
- CertificateNotFound: No certificate found for the student.
- TupleSizeExceeded: Bulk upload size exceeds the limit.

#### *License*
This project is licensed under the MIT License. See the LICENSE file for details.
