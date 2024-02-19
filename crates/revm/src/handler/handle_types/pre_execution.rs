// Includes.
use crate::{
    handler::mainnet,
    primitives::{db::Database, EVMError, EVMResultGeneric, Spec},
    Context,
};
use alloc::sync::Arc;
use revm_precompile::Precompiles;

/// Loads precompiles into Evm
pub type LoadPrecompilesHandle<'a> = Arc<dyn Fn() -> Precompiles + 'a>;

/// Load access list accounts and beneficiary.
/// There is no need to load Caller as it is assumed that
/// it will be loaded in DeductCallerHandle.
pub type LoadAccountsHandle<'a, EXT, DB> =
    Arc<dyn Fn(&mut Context<EXT, DB>) -> Result<(), EVMError<<DB as Database>::Error>> + 'a>;

/// Deduct the caller to its limit.
pub type DeductCallerHandle<'a, EXT, DB> =
    Arc<dyn Fn(&mut Context<EXT, DB>) -> EVMResultGeneric<(), <DB as Database>::Error> + 'a>;

/// Handles related to pre execution before the stack loop is started.
pub struct PreExecutionHandler<'a, EXT, DB: Database> {
    /// Load precompiles
    pub load_precompiles: LoadPrecompilesHandle<'a>,
    /// Main load handle
    pub load_accounts: LoadAccountsHandle<'a, EXT, DB>,
    /// Deduct max value from the caller.
    pub deduct_caller: DeductCallerHandle<'a, EXT, DB>,
}

impl<'a, EXT, DB: Database> PreExecutionHandler<'a, EXT, DB> {
    /// Creates mainnet MainHandles.
    pub fn new<'b, SPEC: Spec + 'b, EXT2: 'b, DB2: Database + 'b>(
    ) -> PreExecutionHandler<'b, EXT2, DB2> {
        PreExecutionHandler {
            load_precompiles: Arc::new(mainnet::load_precompiles::<SPEC>),
            load_accounts: Arc::new(mainnet::load_accounts::<SPEC, EXT2, DB2>),
            deduct_caller: Arc::new(mainnet::deduct_caller::<SPEC, EXT2, DB2>),
        }
    }
}

impl<'a, EXT, DB: Database> PreExecutionHandler<'a, EXT, DB> {
    /// Deduct caller to its limit.
    pub fn deduct_caller(&self, context: &mut Context<EXT, DB>) -> Result<(), EVMError<DB::Error>> {
        (self.deduct_caller)(context)
    }

    /// Main load
    pub fn load_accounts(&self, context: &mut Context<EXT, DB>) -> Result<(), EVMError<DB::Error>> {
        (self.load_accounts)(context)
    }

    /// Load precompiles
    pub fn load_precompiles(&self) -> Precompiles {
        (self.load_precompiles)()
    }
}

/// A trait for pre-execution handler methods.
trait PreExecutionHandlerMethods<EXT, DB>
where
    DB: Database,
{
    /// Load precompiles
    fn load_precompiles(&self) -> Precompiles;

    /// Main load
    fn load_accounts(&self, context: &mut Context<EXT, DB>) -> Result<(), EVMError<DB::Error>>;

    fn deduct_caller(&self, context: &mut Context<EXT, DB>) -> EVMResultGeneric<(), DB::Error>;
}

/// Handles related to pre execution before the stack loop is started.
pub struct PreExecutionHandlerTwo<LP, LA, DC> {
    /// Load precompiles
    pub load_precompiles: LP,
    /// Main load handle
    pub load_accounts: LA,
    /// Deduct max value from the caller.
    pub deduct_caller: DC,
}

impl<LP, LA, DC> PreExecutionHandlerTwo<LP, LA, DC> {
    /// Creates mainnet MainHandles.
    pub fn new<DB, EXT>(load_precompiles: LP, load_accounts: LA, deduct_caller: DC) -> Self
    where
        DB: Database,
        LP: Fn() -> Precompiles,
        LA: Fn(&mut Context<EXT, DB>) -> Result<(), EVMError<<DB as Database>::Error>>,
        DC: Fn(&mut Context<EXT, DB>) -> EVMResultGeneric<(), <DB as Database>::Error>,
    {
        PreExecutionHandlerTwo {
            load_precompiles,
            load_accounts,
            deduct_caller,
        }
    }

    /// Return a new [PreExecutionHandlerTwo] a new load precompiles handle.
    pub fn with_load_precompiles<LP2>(
        self,
        load_precompiles: LP2,
    ) -> PreExecutionHandlerTwo<LP2, LA, DC>
    where
        LP2: Fn() -> Precompiles,
    {
        PreExecutionHandlerTwo {
            load_precompiles,
            load_accounts: self.load_accounts,
            deduct_caller: self.deduct_caller,
        }
    }

    /// Return a new [PreExecutionHandlerTwo] a new load accounts handle.
    pub fn with_load_accounts<LA2, DB, EXT>(
        self,
        load_accounts: LA2,
    ) -> PreExecutionHandlerTwo<LP, LA2, DC>
    where
        LA2: Fn(&mut Context<EXT, DB>) -> Result<(), EVMError<<DB as Database>::Error>>,
        DB: Database,
    {
        PreExecutionHandlerTwo {
            load_precompiles: self.load_precompiles,
            load_accounts,
            deduct_caller: self.deduct_caller,
        }
    }

    /// Return a new [PreExecutionHandlerTwo] a new deduct caller handle.
    pub fn with_deduct_caller<DC2, DB, EXT>(
        self,
        deduct_caller: DC2,
    ) -> PreExecutionHandlerTwo<LP, LA, DC2>
    where
        DC2: Fn(&mut Context<EXT, DB>) -> EVMResultGeneric<(), <DB as Database>::Error>,
        DB: Database,
    {
        PreExecutionHandlerTwo {
            load_precompiles: self.load_precompiles,
            load_accounts: self.load_accounts,
            deduct_caller,
        }
    }
}

impl<LP, LA, DC, EXT, DB> PreExecutionHandlerMethods<EXT, DB> for PreExecutionHandlerTwo<LP, LA, DC>
where
    DB: Database,
    LP: Fn() -> Precompiles,
    LA: Fn(&mut Context<EXT, DB>) -> Result<(), EVMError<<DB as Database>::Error>>,
    DC: Fn(&mut Context<EXT, DB>) -> EVMResultGeneric<(), <DB as Database>::Error>,
{
    /// Load precompiles
    fn load_precompiles(&self) -> Precompiles {
        (self.load_precompiles)()
    }

    /// Main load
    fn load_accounts(&self, context: &mut Context<EXT, DB>) -> Result<(), EVMError<DB::Error>> {
        (self.load_accounts)(context)
    }

    fn deduct_caller(&self, context: &mut Context<EXT, DB>) -> EVMResultGeneric<(), DB::Error> {
        (self.deduct_caller)(context)
    }
}
