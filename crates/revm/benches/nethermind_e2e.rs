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

// ADDRESS opcode benchmark
fn bench_address_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/Address");
    
    // Bytecode pattern from Nethermind: 5b3050600556
    // This creates an infinite loop:
    // 5b    - JUMPDEST (position 0)
    // 30    - ADDRESS
    // 50    - POP
    // 6005  - PUSH1 0x05
    // 56    - JUMP (back to position 0)
    let bytecode = Bytes::from(hex::decode("5b3050600556").unwrap());
    
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

// CALLER opcode benchmark
fn bench_caller_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/Caller");
    
    // Bytecode pattern: 5b3350600556
    // 5b    - JUMPDEST
    // 33    - CALLER
    // 50    - POP
    // 6005  - PUSH1 0x05
    // 56    - JUMP
    let bytecode = Bytes::from(hex::decode("5b3350600556").unwrap());
    
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

// ORIGIN opcode benchmark
fn bench_origin_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/Origin");
    
    // Bytecode pattern: 5b3250600556
    // 5b    - JUMPDEST
    // 32    - ORIGIN
    // 50    - POP
    // 6005  - PUSH1 0x05
    // 56    - JUMP
    let bytecode = Bytes::from(hex::decode("5b3250600556").unwrap());
    
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

// BASEFEE opcode benchmark
fn bench_basefee_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/BaseFee");
    
    // Bytecode pattern: 5b4850600556
    // 5b    - JUMPDEST
    // 48    - BASEFEE
    // 50    - POP
    // 6005  - PUSH1 0x05
    // 56    - JUMP
    let bytecode = Bytes::from(hex::decode("5b4850600556").unwrap());
    
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

// CHAINID opcode benchmark
fn bench_chainid_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/ChainId");
    
    // Bytecode pattern: 5b4650600556
    // 5b    - JUMPDEST
    // 46    - CHAINID
    // 50    - POP
    // 6005  - PUSH1 0x05
    // 56    - JUMP
    let bytecode = Bytes::from(hex::decode("5b4650600556").unwrap());
    
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

// COINBASE opcode benchmark
fn bench_coinbase_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/CoinBase");
    
    // Bytecode pattern: 5b4150600556
    // 5b    - JUMPDEST
    // 41    - COINBASE
    // 50    - POP
    // 6005  - PUSH1 0x05
    // 56    - JUMP
    let bytecode = Bytes::from(hex::decode("5b4150600556").unwrap());
    
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

// GAS opcode benchmark
fn bench_gas_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/Gas");
    
    // Bytecode pattern: 5b5a50600556
    // 5b    - JUMPDEST
    // 5a    - GAS
    // 50    - POP
    // 6005  - PUSH1 0x05
    // 56    - JUMP
    let bytecode = Bytes::from(hex::decode("5b5a50600556").unwrap());
    
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

// GASLIMIT opcode benchmark
fn bench_gaslimit_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/GasLimit");
    
    // Bytecode pattern: 5b4550600556
    // 5b    - JUMPDEST
    // 45    - GASLIMIT
    // 50    - POP
    // 6005  - PUSH1 0x05
    // 56    - JUMP
    let bytecode = Bytes::from(hex::decode("5b4550600556").unwrap());
    
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

// NUMBER opcode benchmark
fn bench_number_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/Number");
    
    // Bytecode pattern: 5b4350600556
    // 5b    - JUMPDEST
    // 43    - NUMBER
    // 50    - POP
    // 6005  - PUSH1 0x05
    // 56    - JUMP
    let bytecode = Bytes::from(hex::decode("5b4350600556").unwrap());
    
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

// TIMESTAMP opcode benchmark
fn bench_timestamp_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/Timestamp");
    
    // Bytecode pattern: 5b4250600556
    // 5b    - JUMPDEST
    // 42    - TIMESTAMP
    // 50    - POP
    // 6005  - PUSH1 0x05
    // 56    - JUMP
    let bytecode = Bytes::from(hex::decode("5b4250600556").unwrap());
    
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

// BLOBBASEFEE opcode benchmark
fn bench_blobbasefee_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/BlobBaseFee");
    
    // Bytecode pattern: 5b4a50600556
    // 5b    - JUMPDEST
    // 4a    - BLOBBASEFEE
    // 50    - POP
    // 6005  - PUSH1 0x05
    // 56    - JUMP
    let bytecode = Bytes::from(hex::decode("5b4a50600556").unwrap());
    
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

// PREVRANDAO opcode benchmark
fn bench_prevrandao_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/PrevRandao");
    
    // Bytecode pattern: 5b4450600556
    // 5b    - JUMPDEST
    // 44    - PREVRANDAO (was DIFFICULTY)
    // 50    - POP
    // 6005  - PUSH1 0x05
    // 56    - JUMP
    let bytecode = Bytes::from(hex::decode("5b4450600556").unwrap());
    
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

// SELFBALANCE opcode benchmark
fn bench_selfbalance_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/SelfBalance");
    
    // Bytecode pattern: 5b4750600556
    // 5b    - JUMPDEST
    // 47    - SELFBALANCE
    // 50    - POP
    // 6005  - PUSH1 0x05
    // 56    - JUMP
    let bytecode = Bytes::from(hex::decode("5b4750600556").unwrap());
    
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

// PUSH0 opcode benchmark
fn bench_push0_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/Push0");
    
    // Bytecode pattern: 5b5f50600556
    // 5b    - JUMPDEST
    // 5f    - PUSH0
    // 50    - POP
    // 6005  - PUSH1 0x05
    // 56    - JUMP
    let bytecode = Bytes::from(hex::decode("5b5f50600556").unwrap());
    
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

// MSIZE opcode benchmark
fn bench_msize_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/MSize");
    
    // Bytecode pattern: 5b5950600556
    // 5b    - JUMPDEST
    // 59    - MSIZE
    // 50    - POP
    // 6005  - PUSH1 0x05
    // 56    - JUMP
    let bytecode = Bytes::from(hex::decode("5b5950600556").unwrap());
    
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

// BLOBHASH opcode benchmark
fn bench_blobhash_e2e(c: &mut Criterion) {
    let mut group = c.benchmark_group("e2e/BlobHash");
    
    // Bytecode pattern: 5b60004950600656
    // 5b    - JUMPDEST
    // 6000  - PUSH1 0x00 (index 0)
    // 49    - BLOBHASH
    // 50    - POP
    // 6006  - PUSH1 0x06
    // 56    - JUMP
    let bytecode = Bytes::from(hex::decode("5b60004950600656").unwrap());
    
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
    name = nethermind_e2e_benches;
    config = Criterion::default().sample_size(10);
    targets = 
        bench_address_e2e,
        bench_caller_e2e,
        bench_origin_e2e,
        bench_basefee_e2e,
        bench_chainid_e2e,
        bench_coinbase_e2e,
        bench_gas_e2e,
        bench_gaslimit_e2e,
        bench_number_e2e,
        bench_timestamp_e2e,
        bench_blobbasefee_e2e,
        bench_prevrandao_e2e,
        bench_selfbalance_e2e,
        bench_push0_e2e,
        bench_msize_e2e,
        bench_blobhash_e2e
}

criterion_main!(nethermind_e2e_benches);