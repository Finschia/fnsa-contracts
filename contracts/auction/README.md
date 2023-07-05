# auction contract

The auction contract runs an auction for an NFT based on cw721. The contract consists of two parts. One is the auction contract, which acts as the caller. The other is a dynamic link version of [cw721-base](https://github.com/CosmWasm/cw-nfts/tree/main/contracts/cw721-base).

## details
When the seller starts the auction, specifying the minimum price, auction time, etc. Bidders can bid on the auction, and at the end of the auction time, the highest bidder pays the funds and takes ownership of the NFT.

Only one auction can be running at a time.

Don't forget that the seller must give contract approval for the NFTs owned by the seller before starting the auction.

## Messages

`StartAuctionMsg {expiration_time, cw721_address, token_id, start_bid}` - Start an auction for the nft specified by `token_id` at `cw721_address`. The auction time is `expiration_time` and the starting price is `start_bid`. Since contract will hold the seller's NFTs for the duration of the auction, you'll need to give contract approval for that NFT(auction item).

`PlaceBidMsg {bid}` - Place a bid for the currently running auction, the bid price is `bid`. The bidder must have more balances than that bid price and must offer a higher price than the current highest bid.

`EndAuctionMsg {}` - Ends the auction. This can only be done by the highest bidder after the auction time has ended. The highest bidder must send the coin more than the bid to contract as `info.funds`. Contract will pass the coin to the seller of the NFT and pass the NFT to the highest bidder to end the auction.

## Queries

`GetHighestBid {}` - Get the highest bid info as `HighestBidResponse{highest_bid, bidder}`.

`GetAuctionItem {}` - Get info about the nft currently being auctioned as `AuctionItemResponse{end_time, cw721_address, token_id, start_bid}`.
    
`GetAuctionHistory {idx}` - Gets the auction history in the form of `AuctionHistoryResponse{end_time, seller, cw721_address, token_id, highest_bid, bidder}`. `idx` starts at 0.
