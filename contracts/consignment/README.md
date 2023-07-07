# Consignment
This contract models an actor of a consignment system.
This contract can hold multiple item and consign own items to another consign contract.
All item's consignment history are traceable and users can query terminal consigee; who has the item now.
In this contract, consignee does not save consigner's information and any other metadata for simplifying this example.

## How to build
See https://github.com/Finschia/fnsa-contracts/README.md.

## Interface
### Message / Query
#### Instantiate
`InstantiateMsg{}` - Instantiating the contract.

#### Execute
- `Mint{}` - mints a new item and issues its' information via an event.
- `Consign{ item_id, consignee}` - consigns the item having given `item_id` to another consignment contract having address `consignee`. Only the owner of the contract can execute it.

#### Query
`TerminalOwner { item_id }` - returns the terminal consignment's owner (who has the item now) of the item having given `item_id`.

### Callble Points
Callable points is called via other consignment contracts for consigning and tracing items.

- `get_consigned() -> Result<u32, ContractError>` - receives a consigned item and returns a minted id for it.
- `trace_terminal_owner(u32) -> Result<u32, ContractError>` - returns the terminal owner (who has the item now) of the item of given `u32` id.

## Note
`trace_terminal_owner` has two problems, but it will be not fixed because this contract is just a sample.

- Dynamic link cannot call the same contract in one callstack.
  This means some items cannot be traced; for example, an item minted in `A` and it is consigned to `B`, then be consigned back to `A`.
  In this example, this contract tries to trace calling `A -> B -> A`, but `B` cannot call `A`.
- Dynamic link cannot call too deep callstack; default max length is 5.
  So, long consignment chain cannot be traced with a contract call.

These can be solved using traditional query without gas problems.
