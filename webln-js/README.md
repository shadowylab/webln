# WebLN - Lightning Web Standard

## Getting started

```sh
npm i @shadowylab/webln
```

```javascript
import { WebLN, loadWasmAsync } from '@shadowylab/webln';

async function webln() {
    let webln = new WebLN();

    // Check if WebLN is enabled
    console.log(await webln.isEnabled());

    // Enable WebLN
    await webln.enable();

    // Get info
    let info = await webln.getInfo();
    console.log(info.alias());
    console.log(info.pubkey());
    console.log(info.color());
    console.log(info.methods());

    // Pay invoice
    await webln.sendPayment("bolt11-invoice");

    // Send payment async (needed for HOLD invoices)
    await webln.sendPaymentAsync("bolt11-invoice");

    let response = await webln.getBalance();
    console.log(response.balance);
}
```

## License

This project is distributed under the MIT software license - see the [LICENSE](LICENSE) file for details

## Donations

⚡ Tips: https://getalby.com/p/yuki

⚡ Lightning Address: yuki@getalby.com
