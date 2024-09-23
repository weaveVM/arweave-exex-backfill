<p align="center">
  <a href="https://wvm.dev">
    <img src="https://raw.githubusercontent.com/weaveVM/.github/main/profile/bg.png">
  </a>
</p>

## About & How it Works
The `arweave-exex-backfill` is a backfill extension for the `WeaveVM-ExEx` data protocol on Arweave. Its purpose is to scan Arweave for potentially missing WeaveVM blocks that were not archived, fetch the missing block data, encode it in borsh-brotli, and then publish it to Arweave following the `WeaveVM-ExEx` data protocol, with the addition of a `WeaveVM:Backfill: true` tag.

If you're running an [Arweave Upload ExEx](https://github.com/weaveVM/wvm-reth/tree/dev/wvm-apps/wvm-exexed/crates/arweave-upload) on your Reth node, you can use this backfill extension to fill any potential data gaps on Arweave.

## WeaveVM-ExEx Data Protocol: Important Note

Once you run an `arweave-exex-backfill` extension instance for your Arweave Upload ExEx, you should internally whitelist the backfill data upload address (backfiller) in your ExEx data protocol on Arweave.

## WeaveVM Backfill Server

- Server Endpoint: https://arweave-exex-backfill.shuttleapp.rs
- Backfill address (backfiller): [F8XVrMQzsHiWfn1CaKtUPxAgUkATXQjXULWw3oVXCiFV](https://viewblock.io/arweave/address/F8XVrMQzsHiWfn1CaKtUPxAgUkATXQjXULWw3oVXCiFV?tab=items)

### Get an archived block by blockNumber

```bash
GET /block/:id
```
## License
This project is licensed under the [MIT License](./LICENSE)