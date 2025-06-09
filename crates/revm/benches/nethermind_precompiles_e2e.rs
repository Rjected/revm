use criterion::{criterion_group, criterion_main, Criterion, BenchmarkId};
use revm::{
    Context,
    database::InMemoryDB,
    primitives::{Address, Bytes, U256, B256, TxKind, keccak256},
    context::{BlockEnv, TxEnv},
    state::AccountInfo,
    bytecode::Bytecode,
    MainBuilder,
    ExecuteEvm,
    MainContext,
};
use std::hint::black_box;
use hex;

// All gas limits from Nethermind benchmarks
const GAS_LIMITS: &[u64] = &[30_000_000, 50_000_000, 60_000_000, 80_000_000, 100_000_000, 150_000_000];

// Helper to create contract account with bytecode
fn setup_contract_account(db: &mut InMemoryDB, address: Address, bytecode: Bytes) {
    let code = Bytecode::new_legacy(bytecode.clone());
    let code_hash = keccak256(&bytecode);
    let account = AccountInfo {
        balance: U256::ZERO,
        nonce: 1,
        code_hash,
        code: Some(code),
    };
    db.insert_account_info(address, account);
}

// EcRecover precompile benchmark
fn bench_ecrecover_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/EcRecover");
    
    // Bytecode for calling ecrecover precompile
    // This pattern sets up memory with test data and calls precompile 0x01
    let bytecode = Bytes::from(hex::decode(
        "60806040527f7c80c68a0fc78000fc2d1710b36de78a072c5ab2a15c6e102c723dc12b6e1e6e600052\
         7f7a7a31b2fb039dd993c3f68c500dc6b1f8e9b09b8f0b18a9ae4f1ce03bb2ad2e602052\
         7f000000000000000000000000000000000000000000000000000000000000001b604052\
         7f0101010101010101010101010101010101010101010101010101010101010101606052\
         602060008060806001600019f160005260206000f3"
    ).unwrap());
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, &gas_limit| {
                b.iter(|| {
                    let mut db = InMemoryDB::default();
                    let contract_address = Address::from([0x02; 20]);
                    setup_contract_account(&mut db, contract_address, bytecode.clone());
                    
                    // Add caller account with balance
                    let caller = Address::from([0x01; 20]);
                    db.insert_account_info(caller, AccountInfo {
                        balance: U256::from(10).pow(U256::from(18)), // 1 ETH
                        nonce: 0,
                        code_hash: keccak256(&[]),
                        code: None,
                    });
                    
                    let tx = TxEnv {
                        caller: Address::from([0x01; 20]),
                        gas_limit,
                        gas_price: 0x3b9aca00u128, // 1 gwei
                        gas_priority_fee: Some(1), // 1 wei priority fee
                        kind: TxKind::Call(contract_address),
                        data: Bytes::default(), // Empty calldata
                        value: U256::ZERO,
                        ..Default::default()
                    };
                    
                    let mut evm = Context::mainnet()
                        .with_db(db)
                        .with_block(BlockEnv {
                            number: U256::from(1),
                            beneficiary: Address::from([0x02; 20]),
                            timestamp: U256::from(0x65156995),
                            difficulty: U256::ZERO,
                            prevrandao: Some(B256::from([0x9c; 32])),
                            basefee: 7,
                            gas_limit: 0x5d21dba00u64,
                            ..Default::default()
                        })
                        .build_mainnet();
                    
                    // Pass the transaction environment to transact
                    let result = evm.transact(tx).unwrap();
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

// Identity precompile benchmark
fn bench_identity_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/Identity");
    
    // Bytecode for calling identity precompile in a loop
    // Sets up 1 byte in memory and calls identity precompile repeatedly
    let bytecode = Bytes::from(hex::decode(
        "60016000526001600060006004600019fa5060005260005160005260206000f3"
    ).unwrap());
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, &gas_limit| {
                b.iter(|| {
                    let mut db = InMemoryDB::default();
                    let contract_address = Address::from([0x02; 20]);
                    setup_contract_account(&mut db, contract_address, bytecode.clone());
                    
                    // Add caller account with balance
                    let caller = Address::from([0x01; 20]);
                    db.insert_account_info(caller, AccountInfo {
                        balance: U256::from(10).pow(U256::from(18)), // 1 ETH
                        nonce: 0,
                        code_hash: keccak256(&[]),
                        code: None,
                    });
                    
                    let tx = TxEnv {
                        caller: Address::from([0x01; 20]),
                        gas_limit,
                        gas_price: 0x3b9aca00u128, // 1 gwei
                        gas_priority_fee: Some(1), // 1 wei priority fee
                        kind: TxKind::Call(contract_address),
                        data: Bytes::default(), // Empty calldata
                        value: U256::ZERO,
                        ..Default::default()
                    };
                    
                    let mut evm = Context::mainnet()
                        .with_db(db)
                        .with_block(BlockEnv {
                            number: U256::from(1),
                            beneficiary: Address::from([0x02; 20]),
                            timestamp: U256::from(0x65156995),
                            difficulty: U256::ZERO,
                            prevrandao: Some(B256::from([0x9c; 32])),
                            basefee: 7,
                            gas_limit: 0x5d21dba00u64,
                            ..Default::default()
                        })
                        .build_mainnet();
                    
                    // Pass the transaction environment to transact
                    let result = evm.transact(tx).unwrap();
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

// Keccak256 benchmark (using SHA3 precompile)
fn bench_keccak256_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/Keccak256");
    
    // Bytecode that repeatedly calls KECCAK256 opcode on 32 bytes
    let bytecode = Bytes::from(hex::decode(
        "5b602060005260206000205060005660001c5660055661ffff57"
    ).unwrap());
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, &gas_limit| {
                b.iter(|| {
                    let mut db = InMemoryDB::default();
                    let contract_address = Address::from([0x02; 20]);
                    setup_contract_account(&mut db, contract_address, bytecode.clone());
                    
                    // Add caller account with balance
                    let caller = Address::from([0x01; 20]);
                    db.insert_account_info(caller, AccountInfo {
                        balance: U256::from(10).pow(U256::from(18)), // 1 ETH
                        nonce: 0,
                        code_hash: keccak256(&[]),
                        code: None,
                    });
                    
                    let tx = TxEnv {
                        caller: Address::from([0x01; 20]),
                        gas_limit,
                        gas_price: 0x3b9aca00u128, // 1 gwei
                        gas_priority_fee: Some(1), // 1 wei priority fee
                        kind: TxKind::Call(contract_address),
                        data: Bytes::default(), // Empty calldata
                        value: U256::ZERO,
                        ..Default::default()
                    };
                    
                    let mut evm = Context::mainnet()
                        .with_db(db)
                        .with_block(BlockEnv {
                            number: U256::from(1),
                            beneficiary: Address::from([0x02; 20]),
                            timestamp: U256::from(0x65156995),
                            difficulty: U256::ZERO,
                            prevrandao: Some(B256::from([0x9c; 32])),
                            basefee: 7,
                            gas_limit: 0x5d21dba00u64,
                            ..Default::default()
                        })
                        .build_mainnet();
                    
                    // Pass the transaction environment to transact
                    let result = evm.transact(tx).unwrap();
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

// Modexp precompile benchmark - Minimal gas case
fn bench_modexp_min_gas_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/ModexpMinGas");
    
    // Modexp with minimal parameters: base_len=1, exp_len=1, mod_len=1
    // Input: base=8, exponent=9, modulus=10
    let modexp_input = hex::decode(
        "0000000000000000000000000000000000000000000000000000000000000001\
         0000000000000000000000000000000000000000000000000000000000000001\
         0000000000000000000000000000000000000000000000000000000000000001\
         08\
         09\
         0a"
    ).unwrap();
    
    // Bytecode that calls modexp precompile in a loop
    let mut bytecode_hex = String::new();
    // Store input data in memory
    for (i, chunk) in modexp_input.chunks(32).enumerate() {
        bytecode_hex.push_str(&format!("7f")); // PUSH32
        for byte in chunk {
            bytecode_hex.push_str(&format!("{:02x}", byte));
        }
        // Pad if chunk is less than 32 bytes
        for _ in chunk.len()..32 {
            bytecode_hex.push_str("00");
        }
        bytecode_hex.push_str(&format!("60{:02x}52", i * 32)); // PUSH1 offset, MSTORE
    }
    
    // Loop: JUMPDEST, call modexp, JUMP back
    bytecode_hex.push_str("5b"); // JUMPDEST
    bytecode_hex.push_str("60206000"); // PUSH1 0x20, PUSH1 0x00 (return data)
    bytecode_hex.push_str(&format!("60{:02x}", modexp_input.len())); // PUSH1 input_len
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (input offset)
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (value)
    bytecode_hex.push_str("6005"); // PUSH1 0x05 (modexp address)
    bytecode_hex.push_str("5a"); // GAS
    bytecode_hex.push_str("f1"); // CALL
    bytecode_hex.push_str("50"); // POP result
    bytecode_hex.push_str(&format!("60{:02x}", bytecode_hex.len() / 2)); // PUSH1 jump_dest
    bytecode_hex.push_str("56"); // JUMP
    
    let bytecode = Bytes::from(hex::decode(bytecode_hex).unwrap());
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, &gas_limit| {
                b.iter(|| {
                    let mut db = InMemoryDB::default();
                    let contract_address = Address::from([0x02; 20]);
                    setup_contract_account(&mut db, contract_address, bytecode.clone());
                    
                    // Add caller account with balance
                    let caller = Address::from([0x01; 20]);
                    db.insert_account_info(caller, AccountInfo {
                        balance: U256::from(10).pow(U256::from(18)), // 1 ETH
                        nonce: 0,
                        code_hash: keccak256(&[]),
                        code: None,
                    });
                    
                    let tx = TxEnv {
                        caller: Address::from([0x01; 20]),
                        gas_limit,
                        gas_price: 0x3b9aca00u128, // 1 gwei
                        gas_priority_fee: Some(1), // 1 wei priority fee
                        kind: TxKind::Call(contract_address),
                        data: Bytes::default(), // Empty calldata
                        value: U256::ZERO,
                        ..Default::default()
                    };
                    
                    let mut evm = Context::mainnet()
                        .with_db(db)
                        .with_block(BlockEnv {
                            number: U256::from(1),
                            beneficiary: Address::from([0x02; 20]),
                            timestamp: U256::from(0x65156995),
                            difficulty: U256::ZERO,
                            prevrandao: Some(B256::from([0x9c; 32])),
                            basefee: 7,
                            gas_limit: 0x5d21dba00u64,
                            ..Default::default()
                        })
                        .build_mainnet();
                    
                    // Pass the transaction environment to transact
                    let result = evm.transact(tx).unwrap();
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

// Modexp 208 gas balanced case
fn bench_modexp_208_gas_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/Modexp208GasBalanced");
    
    // Modexp with 208 gas cost parameters
    // Using specific parameters that result in ~208 gas cost
    let modexp_input = hex::decode(
        "0000000000000000000000000000000000000000000000000000000000000020\
         0000000000000000000000000000000000000000000000000000000000000005\
         0000000000000000000000000000000000000000000000000000000000000020\
         e8e77626586f73b955364c7b4bbf0bb7f7685ebd40e852b164633a4acbd3244c\
         1234567890\
         fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd"
    ).unwrap();
    
    // Create bytecode similar to min gas case but with this input
    let mut bytecode_hex = String::new();
    // Store input data in memory
    for (i, chunk) in modexp_input.chunks(32).enumerate() {
        bytecode_hex.push_str(&format!("7f")); // PUSH32
        for byte in chunk {
            bytecode_hex.push_str(&format!("{:02x}", byte));
        }
        // Pad if chunk is less than 32 bytes
        for _ in chunk.len()..32 {
            bytecode_hex.push_str("00");
        }
        bytecode_hex.push_str(&format!("60{:02x}52", i * 32)); // PUSH1 offset, MSTORE
    }
    
    // Loop: JUMPDEST, call modexp, JUMP back
    bytecode_hex.push_str("5b"); // JUMPDEST
    bytecode_hex.push_str("60206000"); // PUSH1 0x20, PUSH1 0x00 (return data)
    bytecode_hex.push_str(&format!("60{:02x}", modexp_input.len())); // PUSH1 input_len
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (input offset)
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (value)
    bytecode_hex.push_str("6005"); // PUSH1 0x05 (modexp address)
    bytecode_hex.push_str("5a"); // GAS
    bytecode_hex.push_str("f1"); // CALL
    bytecode_hex.push_str("50"); // POP result
    bytecode_hex.push_str(&format!("60{:02x}", bytecode_hex.len() / 2)); // PUSH1 jump_dest
    bytecode_hex.push_str("56"); // JUMP
    
    let bytecode = Bytes::from(hex::decode(bytecode_hex).unwrap());
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, &gas_limit| {
                b.iter(|| {
                    let mut db = InMemoryDB::default();
                    let contract_address = Address::from([0x02; 20]);
                    setup_contract_account(&mut db, contract_address, bytecode.clone());
                    
                    // Add caller account with balance
                    let caller = Address::from([0x01; 20]);
                    db.insert_account_info(caller, AccountInfo {
                        balance: U256::from(10).pow(U256::from(18)), // 1 ETH
                        nonce: 0,
                        code_hash: keccak256(&[]),
                        code: None,
                    });
                    
                    let tx = TxEnv {
                        caller: Address::from([0x01; 20]),
                        gas_limit,
                        gas_price: 0x3b9aca00u128, // 1 gwei
                        gas_priority_fee: Some(1), // 1 wei priority fee
                        kind: TxKind::Call(contract_address),
                        data: Bytes::default(), // Empty calldata
                        value: U256::ZERO,
                        ..Default::default()
                    };
                    
                    let mut evm = Context::mainnet()
                        .with_db(db)
                        .with_block(BlockEnv {
                            number: U256::from(1),
                            beneficiary: Address::from([0x02; 20]),
                            timestamp: U256::from(0x65156995),
                            difficulty: U256::ZERO,
                            prevrandao: Some(B256::from([0x9c; 32])),
                            basefee: 7,
                            gas_limit: 0x5d21dba00u64,
                            ..Default::default()
                        })
                        .build_mainnet();
                    
                    // Pass the transaction environment to transact
                    let result = evm.transact(tx).unwrap();
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

// Modexp 215 gas expensive exponent case
fn bench_modexp_215_gas_exp_heavy_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/Modexp215GasExpHeavy");
    
    // Modexp with expensive exponent (215 gas cost)
    // Larger exponent makes it more expensive
    let modexp_input = hex::decode(
        "0000000000000000000000000000000000000000000000000000000000000020\
         0000000000000000000000000000000000000000000000000000000000000020\
         0000000000000000000000000000000000000000000000000000000000000020\
         e8e77626586f73b955364c7b4bbf0bb7f7685ebd40e852b164633a4acbd3244c\
         ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
         fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd"
    ).unwrap();
    
    // Create bytecode similar to previous cases
    let mut bytecode_hex = String::new();
    // Store input data in memory
    for (i, chunk) in modexp_input.chunks(32).enumerate() {
        bytecode_hex.push_str(&format!("7f")); // PUSH32
        for byte in chunk {
            bytecode_hex.push_str(&format!("{:02x}", byte));
        }
        // Pad if chunk is less than 32 bytes
        for _ in chunk.len()..32 {
            bytecode_hex.push_str("00");
        }
        bytecode_hex.push_str(&format!("60{:02x}52", i * 32)); // PUSH1 offset, MSTORE
    }
    
    // Loop: JUMPDEST, call modexp, JUMP back
    bytecode_hex.push_str("5b"); // JUMPDEST
    bytecode_hex.push_str("60206000"); // PUSH1 0x20, PUSH1 0x00 (return data)
    bytecode_hex.push_str(&format!("60{:02x}", modexp_input.len())); // PUSH1 input_len
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (input offset)
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (value)
    bytecode_hex.push_str("6005"); // PUSH1 0x05 (modexp address)
    bytecode_hex.push_str("5a"); // GAS
    bytecode_hex.push_str("f1"); // CALL
    bytecode_hex.push_str("50"); // POP result
    bytecode_hex.push_str(&format!("60{:02x}", bytecode_hex.len() / 2)); // PUSH1 jump_dest
    bytecode_hex.push_str("56"); // JUMP
    
    let bytecode = Bytes::from(hex::decode(bytecode_hex).unwrap());
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, &gas_limit| {
                b.iter(|| {
                    let mut db = InMemoryDB::default();
                    let contract_address = Address::from([0x02; 20]);
                    setup_contract_account(&mut db, contract_address, bytecode.clone());
                    
                    // Add caller account with balance
                    let caller = Address::from([0x01; 20]);
                    db.insert_account_info(caller, AccountInfo {
                        balance: U256::from(10).pow(U256::from(18)), // 1 ETH
                        nonce: 0,
                        code_hash: keccak256(&[]),
                        code: None,
                    });
                    
                    let tx = TxEnv {
                        caller: Address::from([0x01; 20]),
                        gas_limit,
                        gas_price: 0x3b9aca00u128, // 1 gwei
                        gas_priority_fee: Some(1), // 1 wei priority fee
                        kind: TxKind::Call(contract_address),
                        data: Bytes::default(), // Empty calldata
                        value: U256::ZERO,
                        ..Default::default()
                    };
                    
                    let mut evm = Context::mainnet()
                        .with_db(db)
                        .with_block(BlockEnv {
                            number: U256::from(1),
                            beneficiary: Address::from([0x02; 20]),
                            timestamp: U256::from(0x65156995),
                            difficulty: U256::ZERO,
                            prevrandao: Some(B256::from([0x9c; 32])),
                            basefee: 7,
                            gas_limit: 0x5d21dba00u64,
                            ..Default::default()
                        })
                        .build_mainnet();
                    
                    // Pass the transaction environment to transact
                    let result = evm.transact(tx).unwrap();
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

// Modexp 298 gas expensive case (even larger parameters)
fn bench_modexp_298_gas_exp_heavy_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/Modexp298GasExpHeavy");
    
    // Modexp with very expensive parameters (298 gas cost)
    // Using 64-byte values for base, exp, and modulus
    let modexp_input = hex::decode(
        "0000000000000000000000000000000000000000000000000000000000000040\
         0000000000000000000000000000000000000000000000000000000000000040\
         0000000000000000000000000000000000000000000000000000000000000040\
         e8e77626586f73b955364c7b4bbf0bb7f7685ebd40e852b164633a4acbd3244c\
         f6547b3751e2e4ed86ab2cce601032b7b70dc380c6d21dfb9c3c3f3bf2019ad8\
         ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
         ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
         fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd\
         fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe"
    ).unwrap();
    
    // Create bytecode - need to handle larger input
    let mut bytecode_hex = String::new();
    // Store input data in memory
    for (i, chunk) in modexp_input.chunks(32).enumerate() {
        bytecode_hex.push_str(&format!("7f")); // PUSH32
        for byte in chunk {
            bytecode_hex.push_str(&format!("{:02x}", byte));
        }
        // Pad if chunk is less than 32 bytes
        for _ in chunk.len()..32 {
            bytecode_hex.push_str("00");
        }
        bytecode_hex.push_str(&format!("61{:04x}52", i * 32)); // PUSH2 offset, MSTORE
    }
    
    // Loop: JUMPDEST, call modexp, JUMP back
    bytecode_hex.push_str("5b"); // JUMPDEST
    bytecode_hex.push_str("60406000"); // PUSH1 0x40, PUSH1 0x00 (return data)
    bytecode_hex.push_str(&format!("61{:04x}", modexp_input.len())); // PUSH2 input_len
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (input offset)
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (value)
    bytecode_hex.push_str("6005"); // PUSH1 0x05 (modexp address)
    bytecode_hex.push_str("5a"); // GAS
    bytecode_hex.push_str("f1"); // CALL
    bytecode_hex.push_str("50"); // POP result
    bytecode_hex.push_str(&format!("61{:04x}", bytecode_hex.len() / 2)); // PUSH2 jump_dest
    bytecode_hex.push_str("56"); // JUMP
    
    let bytecode = Bytes::from(hex::decode(bytecode_hex).unwrap());
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, &gas_limit| {
                b.iter(|| {
                    let mut db = InMemoryDB::default();
                    let contract_address = Address::from([0x02; 20]);
                    setup_contract_account(&mut db, contract_address, bytecode.clone());
                    
                    // Add caller account with balance
                    let caller = Address::from([0x01; 20]);
                    db.insert_account_info(caller, AccountInfo {
                        balance: U256::from(10).pow(U256::from(18)), // 1 ETH
                        nonce: 0,
                        code_hash: keccak256(&[]),
                        code: None,
                    });
                    
                    let tx = TxEnv {
                        caller: Address::from([0x01; 20]),
                        gas_limit,
                        gas_price: 0x3b9aca00u128, // 1 gwei
                        gas_priority_fee: Some(1), // 1 wei priority fee
                        kind: TxKind::Call(contract_address),
                        data: Bytes::default(), // Empty calldata
                        value: U256::ZERO,
                        ..Default::default()
                    };
                    
                    let mut evm = Context::mainnet()
                        .with_db(db)
                        .with_block(BlockEnv {
                            number: U256::from(1),
                            beneficiary: Address::from([0x02; 20]),
                            timestamp: U256::from(0x65156995),
                            difficulty: U256::ZERO,
                            prevrandao: Some(B256::from([0x9c; 32])),
                            basefee: 7,
                            gas_limit: 0x5d21dba00u64,
                            ..Default::default()
                        })
                        .build_mainnet();
                    
                    // Pass the transaction environment to transact
                    let result = evm.transact(tx).unwrap();
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

// Modexp minimal gas with expensive exponent
fn bench_modexp_min_gas_exp_heavy_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/ModexpMinGasExpHeavy");
    
    // Minimal size parameters but with expensive exponent pattern
    let modexp_input = hex::decode(
        "0000000000000000000000000000000000000000000000000000000000000001\
         0000000000000000000000000000000000000000000000000000000000000020\
         0000000000000000000000000000000000000000000000000000000000000001\
         08\
         ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
         0a"
    ).unwrap();
    
    // Create bytecode similar to min gas case
    let mut bytecode_hex = String::new();
    // Store input data in memory
    for (i, chunk) in modexp_input.chunks(32).enumerate() {
        bytecode_hex.push_str(&format!("7f")); // PUSH32
        for byte in chunk {
            bytecode_hex.push_str(&format!("{:02x}", byte));
        }
        // Pad if chunk is less than 32 bytes
        for _ in chunk.len()..32 {
            bytecode_hex.push_str("00");
        }
        bytecode_hex.push_str(&format!("60{:02x}52", i * 32)); // PUSH1 offset, MSTORE
    }
    
    // Loop: JUMPDEST, call modexp, JUMP back
    bytecode_hex.push_str("5b"); // JUMPDEST
    bytecode_hex.push_str("60206000"); // PUSH1 0x20, PUSH1 0x00 (return data)
    bytecode_hex.push_str(&format!("60{:02x}", modexp_input.len())); // PUSH1 input_len
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (input offset)
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (value)
    bytecode_hex.push_str("6005"); // PUSH1 0x05 (modexp address)
    bytecode_hex.push_str("5a"); // GAS
    bytecode_hex.push_str("f1"); // CALL
    bytecode_hex.push_str("50"); // POP result
    bytecode_hex.push_str(&format!("60{:02x}", bytecode_hex.len() / 2)); // PUSH1 jump_dest
    bytecode_hex.push_str("56"); // JUMP
    
    let bytecode = Bytes::from(hex::decode(bytecode_hex).unwrap());
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, &gas_limit| {
                b.iter(|| {
                    let mut db = InMemoryDB::default();
                    let contract_address = Address::from([0x02; 20]);
                    setup_contract_account(&mut db, contract_address, bytecode.clone());
                    
                    // Add caller account with balance
                    let caller = Address::from([0x01; 20]);
                    db.insert_account_info(caller, AccountInfo {
                        balance: U256::from(10).pow(U256::from(18)), // 1 ETH
                        nonce: 0,
                        code_hash: keccak256(&[]),
                        code: None,
                    });
                    
                    let tx = TxEnv {
                        caller: Address::from([0x01; 20]),
                        gas_limit,
                        gas_price: 0x3b9aca00u128, // 1 gwei
                        gas_priority_fee: Some(1), // 1 wei priority fee
                        kind: TxKind::Call(contract_address),
                        data: Bytes::default(), // Empty calldata
                        value: U256::ZERO,
                        ..Default::default()
                    };
                    
                    let mut evm = Context::mainnet()
                        .with_db(db)
                        .with_block(BlockEnv {
                            number: U256::from(1),
                            beneficiary: Address::from([0x02; 20]),
                            timestamp: U256::from(0x65156995),
                            difficulty: U256::ZERO,
                            prevrandao: Some(B256::from([0x9c; 32])),
                            basefee: 7,
                            gas_limit: 0x5d21dba00u64,
                            ..Default::default()
                        })
                        .build_mainnet();
                    
                    // Pass the transaction environment to transact
                    let result = evm.transact(tx).unwrap();
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

// Modexp Pawel2 test case
fn bench_modexp_pawel2_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/ModexpPawel2");
    
    // Pawel2 specific test case - edge case testing
    let modexp_input = hex::decode(
        "0000000000000000000000000000000000000000000000000000000000000080\
         0000000000000000000000000000000000000000000000000000000000000001\
         0000000000000000000000000000000000000000000000000000000000000080\
         7878787878787878787878787878787878787878787878787878787878787878\
         7878787878787878787878787878787878787878787878787878787878787878\
         7878787878787878787878787878787878787878787878787878787878787878\
         7878787878787878787878787878787878787878787878787878787878787878\
         02\
         7878787878787878787878787878787878787878787878787878787878787878\
         7878787878787878787878787878787878787878787878787878787878787878\
         7878787878787878787878787878787878787878787878787878787878787878\
         7878787878787878787878787878787878787878787878787878787878787878"
    ).unwrap();
    
    // Create bytecode for larger input
    let mut bytecode_hex = String::new();
    // Store input data in memory
    for (i, chunk) in modexp_input.chunks(32).enumerate() {
        bytecode_hex.push_str(&format!("7f")); // PUSH32
        for byte in chunk {
            bytecode_hex.push_str(&format!("{:02x}", byte));
        }
        // Pad if chunk is less than 32 bytes
        for _ in chunk.len()..32 {
            bytecode_hex.push_str("00");
        }
        bytecode_hex.push_str(&format!("61{:04x}52", i * 32)); // PUSH2 offset, MSTORE
    }
    
    // Loop: JUMPDEST, call modexp, JUMP back
    bytecode_hex.push_str("5b"); // JUMPDEST
    bytecode_hex.push_str("60806000"); // PUSH1 0x80, PUSH1 0x00 (return data)
    bytecode_hex.push_str(&format!("61{:04x}", modexp_input.len())); // PUSH2 input_len
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (input offset)
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (value)
    bytecode_hex.push_str("6005"); // PUSH1 0x05 (modexp address)
    bytecode_hex.push_str("5a"); // GAS
    bytecode_hex.push_str("f1"); // CALL
    bytecode_hex.push_str("50"); // POP result
    bytecode_hex.push_str(&format!("61{:04x}", bytecode_hex.len() / 2)); // PUSH2 jump_dest
    bytecode_hex.push_str("56"); // JUMP
    
    let bytecode = Bytes::from(hex::decode(bytecode_hex).unwrap());
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, &gas_limit| {
                b.iter(|| {
                    let mut db = InMemoryDB::default();
                    let contract_address = Address::from([0x02; 20]);
                    setup_contract_account(&mut db, contract_address, bytecode.clone());
                    
                    // Add caller account with balance
                    let caller = Address::from([0x01; 20]);
                    db.insert_account_info(caller, AccountInfo {
                        balance: U256::from(10).pow(U256::from(18)), // 1 ETH
                        nonce: 0,
                        code_hash: keccak256(&[]),
                        code: None,
                    });
                    
                    let tx = TxEnv {
                        caller: Address::from([0x01; 20]),
                        gas_limit,
                        gas_price: 0x3b9aca00u128, // 1 gwei
                        gas_priority_fee: Some(1), // 1 wei priority fee
                        kind: TxKind::Call(contract_address),
                        data: Bytes::default(), // Empty calldata
                        value: U256::ZERO,
                        ..Default::default()
                    };
                    
                    let mut evm = Context::mainnet()
                        .with_db(db)
                        .with_block(BlockEnv {
                            number: U256::from(1),
                            beneficiary: Address::from([0x02; 20]),
                            timestamp: U256::from(0x65156995),
                            difficulty: U256::ZERO,
                            prevrandao: Some(B256::from([0x9c; 32])),
                            basefee: 7,
                            gas_limit: 0x5d21dba00u64,
                            ..Default::default()
                        })
                        .build_mainnet();
                    
                    // Pass the transaction environment to transact
                    let result = evm.transact(tx).unwrap();
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

// Modexp Vulnerability Pawel1 ExpHeavy test case
fn bench_modexp_vulnerability_pawel1_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/ModexpVulnerabilityPawel1ExpHeavy");
    
    // Vulnerability test case - tests edge cases in modexp
    let modexp_input = hex::decode(
        "0000000000000000000000000000000000000000000000000000000000000001\
         0000000000000000000000000000000000000000000000000000000000000080\
         0000000000000000000000000000000000000000000000000000000000000080\
         ff\
         ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
         ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
         ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
         ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
         8000000000000000000000000000000000000000000000000000000000000000\
         0000000000000000000000000000000000000000000000000000000000000000\
         0000000000000000000000000000000000000000000000000000000000000000\
         0000000000000000000000000000000000000000000000000000000000000001"
    ).unwrap();
    
    // Create bytecode for vulnerability test
    let mut bytecode_hex = String::new();
    // Store input data in memory
    for (i, chunk) in modexp_input.chunks(32).enumerate() {
        bytecode_hex.push_str(&format!("7f")); // PUSH32
        for byte in chunk {
            bytecode_hex.push_str(&format!("{:02x}", byte));
        }
        // Pad if chunk is less than 32 bytes
        for _ in chunk.len()..32 {
            bytecode_hex.push_str("00");
        }
        bytecode_hex.push_str(&format!("61{:04x}52", i * 32)); // PUSH2 offset, MSTORE
    }
    
    // Loop: JUMPDEST, call modexp, JUMP back
    bytecode_hex.push_str("5b"); // JUMPDEST
    bytecode_hex.push_str("60806000"); // PUSH1 0x80, PUSH1 0x00 (return data)
    bytecode_hex.push_str(&format!("61{:04x}", modexp_input.len())); // PUSH2 input_len
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (input offset)
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (value)
    bytecode_hex.push_str("6005"); // PUSH1 0x05 (modexp address)
    bytecode_hex.push_str("5a"); // GAS
    bytecode_hex.push_str("f1"); // CALL
    bytecode_hex.push_str("50"); // POP result
    bytecode_hex.push_str(&format!("61{:04x}", bytecode_hex.len() / 2)); // PUSH2 jump_dest
    bytecode_hex.push_str("56"); // JUMP
    
    let bytecode = Bytes::from(hex::decode(bytecode_hex).unwrap());
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, &gas_limit| {
                b.iter(|| {
                    let mut db = InMemoryDB::default();
                    let contract_address = Address::from([0x02; 20]);
                    setup_contract_account(&mut db, contract_address, bytecode.clone());
                    
                    // Add caller account with balance
                    let caller = Address::from([0x01; 20]);
                    db.insert_account_info(caller, AccountInfo {
                        balance: U256::from(10).pow(U256::from(18)), // 1 ETH
                        nonce: 0,
                        code_hash: keccak256(&[]),
                        code: None,
                    });
                    
                    let tx = TxEnv {
                        caller: Address::from([0x01; 20]),
                        gas_limit,
                        gas_price: 0x3b9aca00u128, // 1 gwei
                        gas_priority_fee: Some(1), // 1 wei priority fee
                        kind: TxKind::Call(contract_address),
                        data: Bytes::default(), // Empty calldata
                        value: U256::ZERO,
                        ..Default::default()
                    };
                    
                    let mut evm = Context::mainnet()
                        .with_db(db)
                        .with_block(BlockEnv {
                            number: U256::from(1),
                            beneficiary: Address::from([0x02; 20]),
                            timestamp: U256::from(0x65156995),
                            difficulty: U256::ZERO,
                            prevrandao: Some(B256::from([0x9c; 32])),
                            basefee: 7,
                            gas_limit: 0x5d21dba00u64,
                            ..Default::default()
                        })
                        .build_mainnet();
                    
                    // Pass the transaction environment to transact
                    let result = evm.transact(tx).unwrap();
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

// Modexp Pawel4 test case
fn bench_modexp_pawel4_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/ModexpPawel4");
    
    // Pawel4 specific test case
    let modexp_input = hex::decode(
        "0000000000000000000000000000000000000000000000000000000000000100\
         0000000000000000000000000000000000000000000000000000000000000001\
         0000000000000000000000000000000000000000000000000000000000000100\
         f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2\
         f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2\
         f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2\
         f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2\
         f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2\
         f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2\
         f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2\
         f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2\
         03\
         e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1\
         e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1\
         e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1\
         e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1\
         e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1\
         e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1\
         e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1\
         e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1"
    ).unwrap();
    
    // Create bytecode for larger input
    let mut bytecode_hex = String::new();
    // Store input data in memory
    for (i, chunk) in modexp_input.chunks(32).enumerate() {
        bytecode_hex.push_str(&format!("7f")); // PUSH32
        for byte in chunk {
            bytecode_hex.push_str(&format!("{:02x}", byte));
        }
        // Pad if chunk is less than 32 bytes
        for _ in chunk.len()..32 {
            bytecode_hex.push_str("00");
        }
        bytecode_hex.push_str(&format!("61{:04x}52", i * 32)); // PUSH2 offset, MSTORE
    }
    
    // Loop: JUMPDEST, call modexp, JUMP back
    bytecode_hex.push_str("5b"); // JUMPDEST
    bytecode_hex.push_str("61010060006101"); // PUSH2 0x0100, PUSH1 0x00, PUSH2 0x01 (return data)
    bytecode_hex.push_str(&format!("{:04x}", modexp_input.len())); // PUSH2 input_len
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (input offset)
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (value)
    bytecode_hex.push_str("6005"); // PUSH1 0x05 (modexp address)
    bytecode_hex.push_str("5a"); // GAS
    bytecode_hex.push_str("f1"); // CALL
    bytecode_hex.push_str("50"); // POP result
    bytecode_hex.push_str(&format!("61{:04x}", bytecode_hex.len() / 2)); // PUSH2 jump_dest
    bytecode_hex.push_str("56"); // JUMP
    
    let bytecode = Bytes::from(hex::decode(bytecode_hex).unwrap());
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, &gas_limit| {
                b.iter(|| {
                    let mut db = InMemoryDB::default();
                    let contract_address = Address::from([0x02; 20]);
                    setup_contract_account(&mut db, contract_address, bytecode.clone());
                    
                    // Add caller account with balance
                    let caller = Address::from([0x01; 20]);
                    db.insert_account_info(caller, AccountInfo {
                        balance: U256::from(10).pow(U256::from(18)), // 1 ETH
                        nonce: 0,
                        code_hash: keccak256(&[]),
                        code: None,
                    });
                    
                    let tx = TxEnv {
                        caller: Address::from([0x01; 20]),
                        gas_limit,
                        gas_price: 0x3b9aca00u128, // 1 gwei
                        gas_priority_fee: Some(1), // 1 wei priority fee
                        kind: TxKind::Call(contract_address),
                        data: Bytes::default(), // Empty calldata
                        value: U256::ZERO,
                        ..Default::default()
                    };
                    
                    let mut evm = Context::mainnet()
                        .with_db(db)
                        .with_block(BlockEnv {
                            number: U256::from(1),
                            beneficiary: Address::from([0x02; 20]),
                            timestamp: U256::from(0x65156995),
                            difficulty: U256::ZERO,
                            prevrandao: Some(B256::from([0x9c; 32])),
                            basefee: 7,
                            gas_limit: 0x5d21dba00u64,
                            ..Default::default()
                        })
                        .build_mainnet();
                    
                    // Pass the transaction environment to transact
                    let result = evm.transact(tx).unwrap();
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

// Modexp Vulnerability Guido4 Even test case
fn bench_modexp_vulnerability_guido4_even_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/ModexpVulnerabilityGuido4Even");
    
    // Guido4 even test case - tests specific edge case
    let modexp_input = hex::decode(
        "0000000000000000000000000000000000000000000000000000000000000080\
         0000000000000000000000000000000000000000000000000000000000000001\
         0000000000000000000000000000000000000000000000000000000000000080\
         0000000000000000000000000000000000000000000000000000000000000000\
         0000000000000000000000000000000000000000000000000000000000000000\
         0000000000000000000000000000000000000000000000000000000000000000\
         fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe\
         02\
         fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd\
         fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd\
         fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd\
         fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffd"
    ).unwrap();
    
    // Create bytecode
    let mut bytecode_hex = String::new();
    // Store input data in memory
    for (i, chunk) in modexp_input.chunks(32).enumerate() {
        bytecode_hex.push_str(&format!("7f")); // PUSH32
        for byte in chunk {
            bytecode_hex.push_str(&format!("{:02x}", byte));
        }
        // Pad if chunk is less than 32 bytes
        for _ in chunk.len()..32 {
            bytecode_hex.push_str("00");
        }
        bytecode_hex.push_str(&format!("61{:04x}52", i * 32)); // PUSH2 offset, MSTORE
    }
    
    // Loop: JUMPDEST, call modexp, JUMP back
    bytecode_hex.push_str("5b"); // JUMPDEST
    bytecode_hex.push_str("60806000"); // PUSH1 0x80, PUSH1 0x00 (return data)
    bytecode_hex.push_str(&format!("61{:04x}", modexp_input.len())); // PUSH2 input_len
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (input offset)
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (value)
    bytecode_hex.push_str("6005"); // PUSH1 0x05 (modexp address)
    bytecode_hex.push_str("5a"); // GAS
    bytecode_hex.push_str("f1"); // CALL
    bytecode_hex.push_str("50"); // POP result
    bytecode_hex.push_str(&format!("61{:04x}", bytecode_hex.len() / 2)); // PUSH2 jump_dest
    bytecode_hex.push_str("56"); // JUMP
    
    let bytecode = Bytes::from(hex::decode(bytecode_hex).unwrap());
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, &gas_limit| {
                b.iter(|| {
                    let mut db = InMemoryDB::default();
                    let contract_address = Address::from([0x02; 20]);
                    setup_contract_account(&mut db, contract_address, bytecode.clone());
                    
                    // Add caller account with balance
                    let caller = Address::from([0x01; 20]);
                    db.insert_account_info(caller, AccountInfo {
                        balance: U256::from(10).pow(U256::from(18)), // 1 ETH
                        nonce: 0,
                        code_hash: keccak256(&[]),
                        code: None,
                    });
                    
                    let tx = TxEnv {
                        caller: Address::from([0x01; 20]),
                        gas_limit,
                        gas_price: 0x3b9aca00u128, // 1 gwei
                        gas_priority_fee: Some(1), // 1 wei priority fee
                        kind: TxKind::Call(contract_address),
                        data: Bytes::default(), // Empty calldata
                        value: U256::ZERO,
                        ..Default::default()
                    };
                    
                    let mut evm = Context::mainnet()
                        .with_db(db)
                        .with_block(BlockEnv {
                            number: U256::from(1),
                            beneficiary: Address::from([0x02; 20]),
                            timestamp: U256::from(0x65156995),
                            difficulty: U256::ZERO,
                            prevrandao: Some(B256::from([0x9c; 32])),
                            basefee: 7,
                            gas_limit: 0x5d21dba00u64,
                            ..Default::default()
                        })
                        .build_mainnet();
                    
                    // Pass the transaction environment to transact
                    let result = evm.transact(tx).unwrap();
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

// Modexp Vulnerability Pawel2 ExpHeavy test case
fn bench_modexp_vulnerability_pawel2_exp_heavy_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/ModexpVulnerabilityPawel2ExpHeavy");
    
    // Pawel2 ExpHeavy vulnerability test
    let modexp_input = hex::decode(
        "0000000000000000000000000000000000000000000000000000000000000080\
         0000000000000000000000000000000000000000000000000000000000000080\
         0000000000000000000000000000000000000000000000000000000000000080\
         7878787878787878787878787878787878787878787878787878787878787878\
         7878787878787878787878787878787878787878787878787878787878787878\
         7878787878787878787878787878787878787878787878787878787878787878\
         7878787878787878787878787878787878787878787878787878787878787878\
         ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
         ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
         ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
         ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
         7878787878787878787878787878787878787878787878787878787878787878\
         7878787878787878787878787878787878787878787878787878787878787878\
         7878787878787878787878787878787878787878787878787878787878787878\
         7878787878787878787878787878787878787878787878787878787878787878"
    ).unwrap();
    
    // Create bytecode
    let mut bytecode_hex = String::new();
    // Store input data in memory
    for (i, chunk) in modexp_input.chunks(32).enumerate() {
        bytecode_hex.push_str(&format!("7f")); // PUSH32
        for byte in chunk {
            bytecode_hex.push_str(&format!("{:02x}", byte));
        }
        // Pad if chunk is less than 32 bytes
        for _ in chunk.len()..32 {
            bytecode_hex.push_str("00");
        }
        bytecode_hex.push_str(&format!("61{:04x}52", i * 32)); // PUSH2 offset, MSTORE
    }
    
    // Loop: JUMPDEST, call modexp, JUMP back
    bytecode_hex.push_str("5b"); // JUMPDEST
    bytecode_hex.push_str("60806000"); // PUSH1 0x80, PUSH1 0x00 (return data)
    bytecode_hex.push_str(&format!("61{:04x}", modexp_input.len())); // PUSH2 input_len
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (input offset)
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (value)
    bytecode_hex.push_str("6005"); // PUSH1 0x05 (modexp address)
    bytecode_hex.push_str("5a"); // GAS
    bytecode_hex.push_str("f1"); // CALL
    bytecode_hex.push_str("50"); // POP result
    bytecode_hex.push_str(&format!("61{:04x}", bytecode_hex.len() / 2)); // PUSH2 jump_dest
    bytecode_hex.push_str("56"); // JUMP
    
    let bytecode = Bytes::from(hex::decode(bytecode_hex).unwrap());
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, &gas_limit| {
                b.iter(|| {
                    let mut db = InMemoryDB::default();
                    let contract_address = Address::from([0x02; 20]);
                    setup_contract_account(&mut db, contract_address, bytecode.clone());
                    
                    // Add caller account with balance
                    let caller = Address::from([0x01; 20]);
                    db.insert_account_info(caller, AccountInfo {
                        balance: U256::from(10).pow(U256::from(18)), // 1 ETH
                        nonce: 0,
                        code_hash: keccak256(&[]),
                        code: None,
                    });
                    
                    let tx = TxEnv {
                        caller: Address::from([0x01; 20]),
                        gas_limit,
                        gas_price: 0x3b9aca00u128, // 1 gwei
                        gas_priority_fee: Some(1), // 1 wei priority fee
                        kind: TxKind::Call(contract_address),
                        data: Bytes::default(), // Empty calldata
                        value: U256::ZERO,
                        ..Default::default()
                    };
                    
                    let mut evm = Context::mainnet()
                        .with_db(db)
                        .with_block(BlockEnv {
                            number: U256::from(1),
                            beneficiary: Address::from([0x02; 20]),
                            timestamp: U256::from(0x65156995),
                            difficulty: U256::ZERO,
                            prevrandao: Some(B256::from([0x9c; 32])),
                            basefee: 7,
                            gas_limit: 0x5d21dba00u64,
                            ..Default::default()
                        })
                        .build_mainnet();
                    
                    // Pass the transaction environment to transact
                    let result = evm.transact(tx).unwrap();
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

// Modexp Vulnerability Pawel3 ExpHeavy test case
fn bench_modexp_vulnerability_pawel3_exp_heavy_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/ModexpVulnerabilityPawel3ExpHeavy");
    
    // Pawel3 ExpHeavy vulnerability test
    let modexp_input = hex::decode(
        "0000000000000000000000000000000000000000000000000000000000000001\
         00000000000000000000000000000000000000000000000000000000000000ff\
         0000000000000000000000000000000000000000000000000000000000000080\
         ff\
         ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
         ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
         ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
         ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
         ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
         ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
         ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
         ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00\
         7878787878787878787878787878787878787878787878787878787878787878\
         7878787878787878787878787878787878787878787878787878787878787878\
         7878787878787878787878787878787878787878787878787878787878787878\
         7878787878787878787878787878787878787878787878787878787878787878"
    ).unwrap();
    
    // Create bytecode
    let mut bytecode_hex = String::new();
    // Store input data in memory
    for (i, chunk) in modexp_input.chunks(32).enumerate() {
        bytecode_hex.push_str(&format!("7f")); // PUSH32
        for byte in chunk {
            bytecode_hex.push_str(&format!("{:02x}", byte));
        }
        // Pad if chunk is less than 32 bytes
        for _ in chunk.len()..32 {
            bytecode_hex.push_str("00");
        }
        bytecode_hex.push_str(&format!("61{:04x}52", i * 32)); // PUSH2 offset, MSTORE
    }
    
    // Loop: JUMPDEST, call modexp, JUMP back
    bytecode_hex.push_str("5b"); // JUMPDEST
    bytecode_hex.push_str("60806000"); // PUSH1 0x80, PUSH1 0x00 (return data)
    bytecode_hex.push_str(&format!("61{:04x}", modexp_input.len())); // PUSH2 input_len
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (input offset)
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (value)
    bytecode_hex.push_str("6005"); // PUSH1 0x05 (modexp address)
    bytecode_hex.push_str("5a"); // GAS
    bytecode_hex.push_str("f1"); // CALL
    bytecode_hex.push_str("50"); // POP result
    bytecode_hex.push_str(&format!("61{:04x}", bytecode_hex.len() / 2)); // PUSH2 jump_dest
    bytecode_hex.push_str("56"); // JUMP
    
    let bytecode = Bytes::from(hex::decode(bytecode_hex).unwrap());
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, &gas_limit| {
                b.iter(|| {
                    let mut db = InMemoryDB::default();
                    let contract_address = Address::from([0x02; 20]);
                    setup_contract_account(&mut db, contract_address, bytecode.clone());
                    
                    // Add caller account with balance
                    let caller = Address::from([0x01; 20]);
                    db.insert_account_info(caller, AccountInfo {
                        balance: U256::from(10).pow(U256::from(18)), // 1 ETH
                        nonce: 0,
                        code_hash: keccak256(&[]),
                        code: None,
                    });
                    
                    let tx = TxEnv {
                        caller: Address::from([0x01; 20]),
                        gas_limit,
                        gas_price: 0x3b9aca00u128, // 1 gwei
                        gas_priority_fee: Some(1), // 1 wei priority fee
                        kind: TxKind::Call(contract_address),
                        data: Bytes::default(), // Empty calldata
                        value: U256::ZERO,
                        ..Default::default()
                    };
                    
                    let mut evm = Context::mainnet()
                        .with_db(db)
                        .with_block(BlockEnv {
                            number: U256::from(1),
                            beneficiary: Address::from([0x02; 20]),
                            timestamp: U256::from(0x65156995),
                            difficulty: U256::ZERO,
                            prevrandao: Some(B256::from([0x9c; 32])),
                            basefee: 7,
                            gas_limit: 0x5d21dba00u64,
                            ..Default::default()
                        })
                        .build_mainnet();
                    
                    // Pass the transaction environment to transact
                    let result = evm.transact(tx).unwrap();
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

// Modexp Vulnerability Pawel4 ExpHeavy test case
fn bench_modexp_vulnerability_pawel4_exp_heavy_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/ModexpVulnerabilityPawel4ExpHeavy");
    
    // Pawel4 ExpHeavy vulnerability test
    let modexp_input = hex::decode(
        "0000000000000000000000000000000000000000000000000000000000000100\
         00000000000000000000000000000000000000000000000000000000000000ff\
         0000000000000000000000000000000000000000000000000000000000000100\
         f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2\
         f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2\
         f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2\
         f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2\
         f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2\
         f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2\
         f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2\
         f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2f2\
         ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
         ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
         ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
         ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
         ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
         ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
         ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff\
         ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff00\
         e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1\
         e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1\
         e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1\
         e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1\
         e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1\
         e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1\
         e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1\
         e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1e1"
    ).unwrap();
    
    // Create bytecode
    let mut bytecode_hex = String::new();
    // Store input data in memory
    for (i, chunk) in modexp_input.chunks(32).enumerate() {
        bytecode_hex.push_str(&format!("7f")); // PUSH32
        for byte in chunk {
            bytecode_hex.push_str(&format!("{:02x}", byte));
        }
        // Pad if chunk is less than 32 bytes
        for _ in chunk.len()..32 {
            bytecode_hex.push_str("00");
        }
        bytecode_hex.push_str(&format!("61{:04x}52", i * 32)); // PUSH2 offset, MSTORE
    }
    
    // Loop: JUMPDEST, call modexp, JUMP back
    bytecode_hex.push_str("5b"); // JUMPDEST
    bytecode_hex.push_str("61010060006101"); // PUSH2 0x0100, PUSH1 0x00, PUSH2 0x01 (return data)
    bytecode_hex.push_str(&format!("{:04x}", modexp_input.len())); // PUSH2 input_len
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (input offset)
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (value)
    bytecode_hex.push_str("6005"); // PUSH1 0x05 (modexp address)
    bytecode_hex.push_str("5a"); // GAS
    bytecode_hex.push_str("f1"); // CALL
    bytecode_hex.push_str("50"); // POP result
    bytecode_hex.push_str(&format!("61{:04x}", bytecode_hex.len() / 2)); // PUSH2 jump_dest
    bytecode_hex.push_str("56"); // JUMP
    
    let bytecode = Bytes::from(hex::decode(bytecode_hex).unwrap());
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, &gas_limit| {
                b.iter(|| {
                    let mut db = InMemoryDB::default();
                    let contract_address = Address::from([0x02; 20]);
                    setup_contract_account(&mut db, contract_address, bytecode.clone());
                    
                    // Add caller account with balance
                    let caller = Address::from([0x01; 20]);
                    db.insert_account_info(caller, AccountInfo {
                        balance: U256::from(10).pow(U256::from(18)), // 1 ETH
                        nonce: 0,
                        code_hash: keccak256(&[]),
                        code: None,
                    });
                    
                    let tx = TxEnv {
                        caller: Address::from([0x01; 20]),
                        gas_limit,
                        gas_price: 0x3b9aca00u128, // 1 gwei
                        gas_priority_fee: Some(1), // 1 wei priority fee
                        kind: TxKind::Call(contract_address),
                        data: Bytes::default(), // Empty calldata
                        value: U256::ZERO,
                        ..Default::default()
                    };
                    
                    let mut evm = Context::mainnet()
                        .with_db(db)
                        .with_block(BlockEnv {
                            number: U256::from(1),
                            beneficiary: Address::from([0x02; 20]),
                            timestamp: U256::from(0x65156995),
                            difficulty: U256::ZERO,
                            prevrandao: Some(B256::from([0x9c; 32])),
                            basefee: 7,
                            gas_limit: 0x5d21dba00u64,
                            ..Default::default()
                        })
                        .build_mainnet();
                    
                    // Pass the transaction environment to transact
                    let result = evm.transact(tx).unwrap();
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

// Blake2f precompile benchmark - 1K rounds
fn bench_blake2f_1k_rounds_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/Blake1KRounds");
    
    // Blake2f input format: rounds (4 bytes) + h (64 bytes) + m (128 bytes) + t (16 bytes) + f (1 byte) = 213 bytes
    // 1K rounds = 1000 = 0x03E8
    let blake2f_input = hex::decode(
        "000003e8\
         48c9bdf267e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5\
         d182e6ad7f520e511f6c3e2b8c68059b6bbd41fbabd9831f79217e1319cde05b\
         6162630000000000000000000000000000000000000000000000000000000000\
         0000000000000000000000000000000000000000000000000000000000000000\
         0000000000000000000000000000000000000000000000000000000000000000\
         0000000000000000000000000000000000000000000000000000000000000000\
         0300000000000000\
         0000000000000000\
         01"
    ).unwrap();
    
    // Create bytecode
    let mut bytecode_hex = String::new();
    // Store input data in memory
    for (i, chunk) in blake2f_input.chunks(32).enumerate() {
        bytecode_hex.push_str(&format!("7f")); // PUSH32
        for byte in chunk {
            bytecode_hex.push_str(&format!("{:02x}", byte));
        }
        // Pad if chunk is less than 32 bytes
        for _ in chunk.len()..32 {
            bytecode_hex.push_str("00");
        }
        bytecode_hex.push_str(&format!("60{:02x}52", i * 32)); // PUSH1 offset, MSTORE
    }
    
    // Loop: JUMPDEST, call blake2f, JUMP back
    let jumpdest_pos = bytecode_hex.len() / 2;
    bytecode_hex.push_str("5b"); // JUMPDEST
    bytecode_hex.push_str("60406000"); // PUSH1 0x40, PUSH1 0x00 (return data)
    bytecode_hex.push_str(&format!("60{:02x}", blake2f_input.len())); // PUSH1 input_len
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (input offset)
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (value)
    bytecode_hex.push_str("6009"); // PUSH1 0x09 (blake2f address)
    bytecode_hex.push_str("5a"); // GAS
    bytecode_hex.push_str("f1"); // CALL
    bytecode_hex.push_str("50"); // POP result
    bytecode_hex.push_str(&format!("60{:02x}", jumpdest_pos)); // PUSH1 jump_dest
    bytecode_hex.push_str("56"); // JUMP
    
    let bytecode = Bytes::from(hex::decode(bytecode_hex).unwrap());
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, &gas_limit| {
                b.iter(|| {
                    let mut db = InMemoryDB::default();
                    let contract_address = Address::from([0x02; 20]);
                    setup_contract_account(&mut db, contract_address, bytecode.clone());
                    
                    // Add caller account with balance
                    let caller = Address::from([0x01; 20]);
                    db.insert_account_info(caller, AccountInfo {
                        balance: U256::from(10).pow(U256::from(18)), // 1 ETH
                        nonce: 0,
                        code_hash: keccak256(&[]),
                        code: None,
                    });
                    
                    let tx = TxEnv {
                        caller: Address::from([0x01; 20]),
                        gas_limit,
                        gas_price: 0x3b9aca00u128, // 1 gwei
                        gas_priority_fee: Some(1), // 1 wei priority fee
                        kind: TxKind::Call(contract_address),
                        data: Bytes::default(), // Empty calldata
                        value: U256::ZERO,
                        ..Default::default()
                    };
                    
                    let mut evm = Context::mainnet()
                        .with_db(db)
                        .with_block(BlockEnv {
                            number: U256::from(1),
                            beneficiary: Address::from([0x02; 20]),
                            timestamp: U256::from(0x65156995),
                            difficulty: U256::ZERO,
                            prevrandao: Some(B256::from([0x9c; 32])),
                            basefee: 7,
                            gas_limit: 0x5d21dba00u64,
                            ..Default::default()
                        })
                        .build_mainnet();
                    
                    // Pass the transaction environment to transact
                    let result = evm.transact(tx).unwrap();
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

// Blake2f precompile benchmark - 1M rounds
fn bench_blake2f_1m_rounds_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/Blake1MRounds");
    
    // Blake2f input format: rounds (4 bytes) + h (64 bytes) + m (128 bytes) + t (16 bytes) + f (1 byte) = 213 bytes
    // 1M rounds = 1000000 = 0x0F4240
    let blake2f_input = hex::decode(
        "000f4240\
         48c9bdf267e6096a3ba7ca8485ae67bb2bf894fe72f36e3cf1361d5f3af54fa5\
         d182e6ad7f520e511f6c3e2b8c68059b6bbd41fbabd9831f79217e1319cde05b\
         6162630000000000000000000000000000000000000000000000000000000000\
         0000000000000000000000000000000000000000000000000000000000000000\
         0000000000000000000000000000000000000000000000000000000000000000\
         0000000000000000000000000000000000000000000000000000000000000000\
         0300000000000000\
         0000000000000000\
         01"
    ).unwrap();
    
    // Create bytecode
    let mut bytecode_hex = String::new();
    // Store input data in memory
    for (i, chunk) in blake2f_input.chunks(32).enumerate() {
        bytecode_hex.push_str(&format!("7f")); // PUSH32
        for byte in chunk {
            bytecode_hex.push_str(&format!("{:02x}", byte));
        }
        // Pad if chunk is less than 32 bytes
        for _ in chunk.len()..32 {
            bytecode_hex.push_str("00");
        }
        bytecode_hex.push_str(&format!("60{:02x}52", i * 32)); // PUSH1 offset, MSTORE
    }
    
    // Loop: JUMPDEST, call blake2f, JUMP back
    let jumpdest_pos = bytecode_hex.len() / 2;
    bytecode_hex.push_str("5b"); // JUMPDEST
    bytecode_hex.push_str("60406000"); // PUSH1 0x40, PUSH1 0x00 (return data)
    bytecode_hex.push_str(&format!("60{:02x}", blake2f_input.len())); // PUSH1 input_len
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (input offset)
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (value)
    bytecode_hex.push_str("6009"); // PUSH1 0x09 (blake2f address)
    bytecode_hex.push_str("5a"); // GAS
    bytecode_hex.push_str("f1"); // CALL
    bytecode_hex.push_str("50"); // POP result
    bytecode_hex.push_str(&format!("60{:02x}", jumpdest_pos)); // PUSH1 jump_dest
    bytecode_hex.push_str("56"); // JUMP
    
    let bytecode = Bytes::from(hex::decode(bytecode_hex).unwrap());
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, &gas_limit| {
                b.iter(|| {
                    let mut db = InMemoryDB::default();
                    let contract_address = Address::from([0x02; 20]);
                    setup_contract_account(&mut db, contract_address, bytecode.clone());
                    
                    // Add caller account with balance
                    let caller = Address::from([0x01; 20]);
                    db.insert_account_info(caller, AccountInfo {
                        balance: U256::from(10).pow(U256::from(18)), // 1 ETH
                        nonce: 0,
                        code_hash: keccak256(&[]),
                        code: None,
                    });
                    
                    let tx = TxEnv {
                        caller: Address::from([0x01; 20]),
                        gas_limit,
                        gas_price: 0x3b9aca00u128, // 1 gwei
                        gas_priority_fee: Some(1), // 1 wei priority fee
                        kind: TxKind::Call(contract_address),
                        data: Bytes::default(), // Empty calldata
                        value: U256::ZERO,
                        ..Default::default()
                    };
                    
                    let mut evm = Context::mainnet()
                        .with_db(db)
                        .with_block(BlockEnv {
                            number: U256::from(1),
                            beneficiary: Address::from([0x02; 20]),
                            timestamp: U256::from(0x65156995),
                            difficulty: U256::ZERO,
                            prevrandao: Some(B256::from([0x9c; 32])),
                            basefee: 7,
                            gas_limit: 0x5d21dba00u64,
                            ..Default::default()
                        })
                        .build_mainnet();
                    
                    // Pass the transaction environment to transact
                    let result = evm.transact(tx).unwrap();
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

// EcAdd precompile benchmark - 12 byte coordinates
fn bench_ecadd_12_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/EcAdd12");
    
    // EcAdd input: two points (x1, y1, x2, y2) - using 12-byte values padded to 32 bytes
    let ecadd_input = hex::decode(
        "0000000000000000000000000000000000000000000000000000000000000001\
         0000000000000000000000000000000000000000000000000000000000000002\
         0000000000000000000000000000000000000000000000000000000000000001\
         0000000000000000000000000000000000000000000000000000000000000002"
    ).unwrap();
    
    // Create bytecode
    let mut bytecode_hex = String::new();
    // Store input data in memory
    for (i, chunk) in ecadd_input.chunks(32).enumerate() {
        bytecode_hex.push_str(&format!("7f")); // PUSH32
        for byte in chunk {
            bytecode_hex.push_str(&format!("{:02x}", byte));
        }
        bytecode_hex.push_str(&format!("60{:02x}52", i * 32)); // PUSH1 offset, MSTORE
    }
    
    // Loop: JUMPDEST, call ecadd, JUMP back
    bytecode_hex.push_str("5b"); // JUMPDEST
    bytecode_hex.push_str("60406000"); // PUSH1 0x40, PUSH1 0x00 (return data)
    bytecode_hex.push_str(&format!("60{:02x}", ecadd_input.len())); // PUSH1 input_len
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (input offset)
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (value)
    bytecode_hex.push_str("6006"); // PUSH1 0x06 (ecadd address)
    bytecode_hex.push_str("5a"); // GAS
    bytecode_hex.push_str("f1"); // CALL
    bytecode_hex.push_str("50"); // POP result
    bytecode_hex.push_str(&format!("60{:02x}", bytecode_hex.len() / 2)); // PUSH1 jump_dest
    bytecode_hex.push_str("56"); // JUMP
    
    let bytecode = Bytes::from(hex::decode(bytecode_hex).unwrap());
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, &gas_limit| {
                b.iter(|| {
                    let mut db = InMemoryDB::default();
                    let contract_address = Address::from([0x02; 20]);
                    setup_contract_account(&mut db, contract_address, bytecode.clone());
                    
                    // Add caller account with balance
                    let caller = Address::from([0x01; 20]);
                    db.insert_account_info(caller, AccountInfo {
                        balance: U256::from(10).pow(U256::from(18)), // 1 ETH
                        nonce: 0,
                        code_hash: keccak256(&[]),
                        code: None,
                    });
                    
                    let tx = TxEnv {
                        caller: Address::from([0x01; 20]),
                        gas_limit,
                        gas_price: 0x3b9aca00u128, // 1 gwei
                        gas_priority_fee: Some(1), // 1 wei priority fee
                        kind: TxKind::Call(contract_address),
                        data: Bytes::default(), // Empty calldata
                        value: U256::ZERO,
                        ..Default::default()
                    };
                    
                    let mut evm = Context::mainnet()
                        .with_db(db)
                        .with_block(BlockEnv {
                            number: U256::from(1),
                            beneficiary: Address::from([0x02; 20]),
                            timestamp: U256::from(0x65156995),
                            difficulty: U256::ZERO,
                            prevrandao: Some(B256::from([0x9c; 32])),
                            basefee: 7,
                            gas_limit: 0x5d21dba00u64,
                            ..Default::default()
                        })
                        .build_mainnet();
                    
                    // Pass the transaction environment to transact
                    let result = evm.transact(tx).unwrap();
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

// EcAdd precompile benchmark - 32 byte coordinates
fn bench_ecadd_32_byte_coordinates_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/EcAdd32ByteCoordinates");
    
    // EcAdd input: two points (x1, y1, x2, y2) - full 32-byte coordinates
    let ecadd_input = hex::decode(
        "1c76476f4def4bb94541d57ebba1193381ffa7aa76ada664dd31c16024c43f59\
         3034dd2920f673e204fee2811c678745fc819b55d3e9d294e45c9b03a76aef41\
         209dd15ebff5d46c4bd888e51a93cf99a7329636c63514396b4a452003a35bf7\
         04bf11ca01483bfa8b34b43561848d28905960114c8ac04049af4b6315a41678"
    ).unwrap();
    
    // Create bytecode
    let mut bytecode_hex = String::new();
    // Store input data in memory
    for (i, chunk) in ecadd_input.chunks(32).enumerate() {
        bytecode_hex.push_str(&format!("7f")); // PUSH32
        for byte in chunk {
            bytecode_hex.push_str(&format!("{:02x}", byte));
        }
        bytecode_hex.push_str(&format!("60{:02x}52", i * 32)); // PUSH1 offset, MSTORE
    }
    
    // Loop: JUMPDEST, call ecadd, JUMP back
    bytecode_hex.push_str("5b"); // JUMPDEST
    bytecode_hex.push_str("60406000"); // PUSH1 0x40, PUSH1 0x00 (return data)
    bytecode_hex.push_str(&format!("60{:02x}", ecadd_input.len())); // PUSH1 input_len
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (input offset)
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (value)
    bytecode_hex.push_str("6006"); // PUSH1 0x06 (ecadd address)
    bytecode_hex.push_str("5a"); // GAS
    bytecode_hex.push_str("f1"); // CALL
    bytecode_hex.push_str("50"); // POP result
    bytecode_hex.push_str(&format!("60{:02x}", bytecode_hex.len() / 2)); // PUSH1 jump_dest
    bytecode_hex.push_str("56"); // JUMP
    
    let bytecode = Bytes::from(hex::decode(bytecode_hex).unwrap());
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, &gas_limit| {
                b.iter(|| {
                    let mut db = InMemoryDB::default();
                    let contract_address = Address::from([0x02; 20]);
                    setup_contract_account(&mut db, contract_address, bytecode.clone());
                    
                    // Add caller account with balance
                    let caller = Address::from([0x01; 20]);
                    db.insert_account_info(caller, AccountInfo {
                        balance: U256::from(10).pow(U256::from(18)), // 1 ETH
                        nonce: 0,
                        code_hash: keccak256(&[]),
                        code: None,
                    });
                    
                    let tx = TxEnv {
                        caller: Address::from([0x01; 20]),
                        gas_limit,
                        gas_price: 0x3b9aca00u128, // 1 gwei
                        gas_priority_fee: Some(1), // 1 wei priority fee
                        kind: TxKind::Call(contract_address),
                        data: Bytes::default(), // Empty calldata
                        value: U256::ZERO,
                        ..Default::default()
                    };
                    
                    let mut evm = Context::mainnet()
                        .with_db(db)
                        .with_block(BlockEnv {
                            number: U256::from(1),
                            beneficiary: Address::from([0x02; 20]),
                            timestamp: U256::from(0x65156995),
                            difficulty: U256::ZERO,
                            prevrandao: Some(B256::from([0x9c; 32])),
                            basefee: 7,
                            gas_limit: 0x5d21dba00u64,
                            ..Default::default()
                        })
                        .build_mainnet();
                    
                    // Pass the transaction environment to transact
                    let result = evm.transact(tx).unwrap();
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

// EcMul precompile benchmark - 12 byte coordinates and 32 byte scalar
fn bench_ecmul_12_and_32_byte_scalar_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/EcMul12And32ByteScalar");
    
    // EcMul input: point (x, y) and scalar k - 12-byte coords, 32-byte scalar
    let ecmul_input = hex::decode(
        "0000000000000000000000000000000000000000000000000000000000000001\
         0000000000000000000000000000000000000000000000000000000000000002\
         ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
    ).unwrap();
    
    // Create bytecode
    let mut bytecode_hex = String::new();
    // Store input data in memory
    for (i, chunk) in ecmul_input.chunks(32).enumerate() {
        bytecode_hex.push_str(&format!("7f")); // PUSH32
        for byte in chunk {
            bytecode_hex.push_str(&format!("{:02x}", byte));
        }
        bytecode_hex.push_str(&format!("60{:02x}52", i * 32)); // PUSH1 offset, MSTORE
    }
    
    // Loop: JUMPDEST, call ecmul, JUMP back
    bytecode_hex.push_str("5b"); // JUMPDEST
    bytecode_hex.push_str("60406000"); // PUSH1 0x40, PUSH1 0x00 (return data)
    bytecode_hex.push_str(&format!("60{:02x}", ecmul_input.len())); // PUSH1 input_len
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (input offset)
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (value)
    bytecode_hex.push_str("6007"); // PUSH1 0x07 (ecmul address)
    bytecode_hex.push_str("5a"); // GAS
    bytecode_hex.push_str("f1"); // CALL
    bytecode_hex.push_str("50"); // POP result
    bytecode_hex.push_str(&format!("60{:02x}", bytecode_hex.len() / 2)); // PUSH1 jump_dest
    bytecode_hex.push_str("56"); // JUMP
    
    let bytecode = Bytes::from(hex::decode(bytecode_hex).unwrap());
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, &gas_limit| {
                b.iter(|| {
                    let mut db = InMemoryDB::default();
                    let contract_address = Address::from([0x02; 20]);
                    setup_contract_account(&mut db, contract_address, bytecode.clone());
                    
                    // Add caller account with balance
                    let caller = Address::from([0x01; 20]);
                    db.insert_account_info(caller, AccountInfo {
                        balance: U256::from(10).pow(U256::from(18)), // 1 ETH
                        nonce: 0,
                        code_hash: keccak256(&[]),
                        code: None,
                    });
                    
                    let tx = TxEnv {
                        caller: Address::from([0x01; 20]),
                        gas_limit,
                        gas_price: 0x3b9aca00u128, // 1 gwei
                        gas_priority_fee: Some(1), // 1 wei priority fee
                        kind: TxKind::Call(contract_address),
                        data: Bytes::default(), // Empty calldata
                        value: U256::ZERO,
                        ..Default::default()
                    };
                    
                    let mut evm = Context::mainnet()
                        .with_db(db)
                        .with_block(BlockEnv {
                            number: U256::from(1),
                            beneficiary: Address::from([0x02; 20]),
                            timestamp: U256::from(0x65156995),
                            difficulty: U256::ZERO,
                            prevrandao: Some(B256::from([0x9c; 32])),
                            basefee: 7,
                            gas_limit: 0x5d21dba00u64,
                            ..Default::default()
                        })
                        .build_mainnet();
                    
                    // Pass the transaction environment to transact
                    let result = evm.transact(tx).unwrap();
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

// EcMul precompile benchmark - 32 byte coordinates and 32 byte scalar
fn bench_ecmul_32_byte_coordinates_32_byte_scalar_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/EcMul32ByteCoordinates32ByteScalar");
    
    // EcMul input: point (x, y) and scalar k - full 32-byte values
    let ecmul_input = hex::decode(
        "1c76476f4def4bb94541d57ebba1193381ffa7aa76ada664dd31c16024c43f59\
         3034dd2920f673e204fee2811c678745fc819b55d3e9d294e45c9b03a76aef41\
         ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
    ).unwrap();
    
    // Create bytecode
    let mut bytecode_hex = String::new();
    // Store input data in memory
    for (i, chunk) in ecmul_input.chunks(32).enumerate() {
        bytecode_hex.push_str(&format!("7f")); // PUSH32
        for byte in chunk {
            bytecode_hex.push_str(&format!("{:02x}", byte));
        }
        bytecode_hex.push_str(&format!("60{:02x}52", i * 32)); // PUSH1 offset, MSTORE
    }
    
    // Loop: JUMPDEST, call ecmul, JUMP back
    bytecode_hex.push_str("5b"); // JUMPDEST
    bytecode_hex.push_str("60406000"); // PUSH1 0x40, PUSH1 0x00 (return data)
    bytecode_hex.push_str(&format!("60{:02x}", ecmul_input.len())); // PUSH1 input_len
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (input offset)
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (value)
    bytecode_hex.push_str("6007"); // PUSH1 0x07 (ecmul address)
    bytecode_hex.push_str("5a"); // GAS
    bytecode_hex.push_str("f1"); // CALL
    bytecode_hex.push_str("50"); // POP result
    bytecode_hex.push_str(&format!("60{:02x}", bytecode_hex.len() / 2)); // PUSH1 jump_dest
    bytecode_hex.push_str("56"); // JUMP
    
    let bytecode = Bytes::from(hex::decode(bytecode_hex).unwrap());
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, &gas_limit| {
                b.iter(|| {
                    let mut db = InMemoryDB::default();
                    let contract_address = Address::from([0x02; 20]);
                    setup_contract_account(&mut db, contract_address, bytecode.clone());
                    
                    // Add caller account with balance
                    let caller = Address::from([0x01; 20]);
                    db.insert_account_info(caller, AccountInfo {
                        balance: U256::from(10).pow(U256::from(18)), // 1 ETH
                        nonce: 0,
                        code_hash: keccak256(&[]),
                        code: None,
                    });
                    
                    let tx = TxEnv {
                        caller: Address::from([0x01; 20]),
                        gas_limit,
                        gas_price: 0x3b9aca00u128, // 1 gwei
                        gas_priority_fee: Some(1), // 1 wei priority fee
                        kind: TxKind::Call(contract_address),
                        data: Bytes::default(), // Empty calldata
                        value: U256::ZERO,
                        ..Default::default()
                    };
                    
                    let mut evm = Context::mainnet()
                        .with_db(db)
                        .with_block(BlockEnv {
                            number: U256::from(1),
                            beneficiary: Address::from([0x02; 20]),
                            timestamp: U256::from(0x65156995),
                            difficulty: U256::ZERO,
                            prevrandao: Some(B256::from([0x9c; 32])),
                            basefee: 7,
                            gas_limit: 0x5d21dba00u64,
                            ..Default::default()
                        })
                        .build_mainnet();
                    
                    // Pass the transaction environment to transact
                    let result = evm.transact(tx).unwrap();
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

// EcPairing precompile benchmark - 2 sets
fn bench_ecpairing_2_sets_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/EcPairing2Sets");
    
    // EcPairing input: 2 pairs of (G1, G2) points
    // Each G1 point is 64 bytes (x, y), each G2 point is 128 bytes (x_im, x_re, y_im, y_re)
    // Total per pair: 192 bytes, 2 pairs = 384 bytes
    let ecpairing_input = hex::decode(
        "1c76476f4def4bb94541d57ebba1193381ffa7aa76ada664dd31c16024c43f59\
         3034dd2920f673e204fee2811c678745fc819b55d3e9d294e45c9b03a76aef41\
         0fb43f5ffb074c2e75823d15e7f7389e5ffee2beafd4bbdefb5ae5a85ce82b68\
         2f8615e5d8ca89a7ca4f2eb3fe9527f3e5c737339ffc466ec600de2165c34dc8\
         0a3cbecf6c6bdf1308ad8d9ea1ca4821c33a5913666375403d61fb2ad7a16fa9\
         1f77e29b03b76a92dc5e2e8d3df088c37f4db3325c7eb2655af68ad86aeef9a2\
         209dd15ebff5d46c4bd888e51a93cf99a7329636c63514396b4a452003a35bf7\
         04bf11ca01483bfa8b34b43561848d28905960114c8ac04049af4b6315a41678\
         11e0b079dea56f52a5194d5916da4e87d27f10f576dc4aa4e54e19373d587fd0\
         066368b893c53861cc6b305c176b327f1f6db8001853de54b3fe4acad8c47a1f\
         003730de140344e023bb8821f90b923bd0238a64e473cd0195d8a5b2109ae2f6\
         08ac3b6855b47d8790ed8569ae5dad22cdee45b8d32908e592cf172c613bc2d5"
    ).unwrap();
    
    // Create bytecode for larger input
    let mut bytecode_hex = String::new();
    // Store input data in memory
    for (i, chunk) in ecpairing_input.chunks(32).enumerate() {
        bytecode_hex.push_str(&format!("7f")); // PUSH32
        for byte in chunk {
            bytecode_hex.push_str(&format!("{:02x}", byte));
        }
        bytecode_hex.push_str(&format!("61{:04x}52", i * 32)); // PUSH2 offset, MSTORE
    }
    
    // Loop: JUMPDEST, call ecpairing, JUMP back
    bytecode_hex.push_str("5b"); // JUMPDEST
    bytecode_hex.push_str("60206000"); // PUSH1 0x20, PUSH1 0x00 (return data)
    bytecode_hex.push_str(&format!("61{:04x}", ecpairing_input.len())); // PUSH2 input_len
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (input offset)
    bytecode_hex.push_str("6000"); // PUSH1 0x00 (value)
    bytecode_hex.push_str("6008"); // PUSH1 0x08 (ecpairing address)
    bytecode_hex.push_str("5a"); // GAS
    bytecode_hex.push_str("f1"); // CALL
    bytecode_hex.push_str("50"); // POP result
    bytecode_hex.push_str(&format!("61{:04x}", bytecode_hex.len() / 2)); // PUSH2 jump_dest
    bytecode_hex.push_str("56"); // JUMP
    
    let bytecode = Bytes::from(hex::decode(bytecode_hex).unwrap());
    
    for &gas_limit in GAS_LIMITS {
        group.bench_with_input(
            BenchmarkId::from_parameter(format!("{}M", gas_limit / 1_000_000)),
            &gas_limit,
            |b, &gas_limit| {
                b.iter(|| {
                    let mut db = InMemoryDB::default();
                    let contract_address = Address::from([0x02; 20]);
                    setup_contract_account(&mut db, contract_address, bytecode.clone());
                    
                    // Add caller account with balance
                    let caller = Address::from([0x01; 20]);
                    db.insert_account_info(caller, AccountInfo {
                        balance: U256::from(10).pow(U256::from(18)), // 1 ETH
                        nonce: 0,
                        code_hash: keccak256(&[]),
                        code: None,
                    });
                    
                    let tx = TxEnv {
                        caller: Address::from([0x01; 20]),
                        gas_limit,
                        gas_price: 0x3b9aca00u128, // 1 gwei
                        gas_priority_fee: Some(1), // 1 wei priority fee
                        kind: TxKind::Call(contract_address),
                        data: Bytes::default(), // Empty calldata
                        value: U256::ZERO,
                        ..Default::default()
                    };
                    
                    let mut evm = Context::mainnet()
                        .with_db(db)
                        .with_block(BlockEnv {
                            number: U256::from(1),
                            beneficiary: Address::from([0x02; 20]),
                            timestamp: U256::from(0x65156995),
                            difficulty: U256::ZERO,
                            prevrandao: Some(B256::from([0x9c; 32])),
                            basefee: 7,
                            gas_limit: 0x5d21dba00u64,
                            ..Default::default()
                        })
                        .build_mainnet();
                    
                    // Pass the transaction environment to transact
                    let result = evm.transact(tx).unwrap();
                    black_box(result);
                });
            },
        );
    }
    group.finish();
}

criterion_group! {
    name = nethermind_precompiles_e2e_benches;
    config = Criterion::default().sample_size(10);
    targets = 
        bench_ecrecover_e2e,
        bench_identity_e2e,
        bench_keccak256_e2e,
        bench_modexp_min_gas_e2e,
        bench_modexp_208_gas_e2e,
        bench_modexp_215_gas_exp_heavy_e2e,
        bench_modexp_298_gas_exp_heavy_e2e,
        bench_modexp_min_gas_exp_heavy_e2e,
        bench_modexp_pawel2_e2e,
        bench_modexp_vulnerability_pawel1_e2e,
        bench_modexp_pawel4_e2e,
        bench_modexp_vulnerability_guido4_even_e2e,
        bench_modexp_vulnerability_pawel2_exp_heavy_e2e,
        bench_modexp_vulnerability_pawel3_exp_heavy_e2e,
        bench_modexp_vulnerability_pawel4_exp_heavy_e2e,
        bench_blake2f_1k_rounds_e2e,
        bench_blake2f_1m_rounds_e2e,
        bench_ecadd_12_e2e,
        bench_ecadd_32_byte_coordinates_e2e,
        bench_ecmul_12_and_32_byte_scalar_e2e,
        bench_ecmul_32_byte_coordinates_32_byte_scalar_e2e,
        bench_ecpairing_2_sets_e2e
}

criterion_main!(nethermind_precompiles_e2e_benches);