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
- `Consign{ item_id, consignee}` - consigns the item having given `item_id` to another consignment contract having address `consignee`.

#### Query
`TerminalConsignee { item_id }` - returns the terminal consignee of the item having given `item_id`.

### Callble Points
Callable points is called via other consignment contracts for consigning and tracing items.

- `get_consigned() -> Result<u32, ContractError>` - receives a consigned item and returns a minted id for it.
- `trace_terminal_consignee(u32) -> Result<u32, ContractError>` - returns the terminal consignee (who has the item now) of the item of given `u32` id.
