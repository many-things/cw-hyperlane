export enum CONTRACT_NAMES {
  HPL_HUB = "hpl_hub",
  HPL_IGP_CORE = "hpl_igp_core",
  HPL_IGP_GAS_ORACLE = "hpl_igp_gas_oracle",
  HPL_ISM_MULTISIG = "hpl_ism_multisig",
  HPL_ISM_ROUTING = "hpl_ism_routing",
  HPL_MAILBOX = "hpl_mailbox",
  HPL_MULTICALL = "hpl_multicall",
  HPL_VALIDATOR_ANNOUNCE = "hpl_validator_announce",
}

export const ARTIFACTS: {
  contracts: {
    [name in CONTRACT_NAMES]: {
      address: string;
      codeId: number;
      digest: string;
    };
  };
  address: string;
  latestMigration: string;
} = {
  contracts: {
    hpl_hub: {
      address: "osmo1fzampgfzygfmh7l5vwnfqttx47q94yrpd57ceurs2jt6g46q9wys037jcr",
      codeId: 3152,
      digest: "2824a19f9b8607e25d55576e7324ebf1ed0f2c846eb01839e958c94866e8a0ad"
    },
    hpl_igp_core: {
      address: "osmo18eptphzlxuup2387f3wlewn68pzr4keanz6e67n9tnrtwep7dhqshm9taq",
      codeId: 3153,
      digest: "43af1b82eb8cd9632653e76769ffeb7da1c95ea575869f7f5e34177dbe8bec5e"
    },
    hpl_igp_gas_oracle: {
      address: "osmo1tcwlcmtj3l4720n8cdk0t6l0nts27x2rswt6x0hj9kjc8wq9n8qs6gu3fg",
      codeId: 3154,
      digest: "7e31e76fb15b3332dab4890768a84881d9cd7431723907b7405c8f214c66e6de"
    },
    hpl_ism_multisig: {
      address: "osmo12pyzlwg74p88d3em92ztyvl3mszgn0tc9chrj3hwhuzplrpn2z5ssygfuq",
      codeId: 3155,
      digest: "29f01d5bc172ed671e0d191b85315c484f4e234ae3a1c9a14482206cbb2c0a41"
    },
    hpl_ism_routing: {
      address: "osmo1vdl5fazjl73kp8re23n3aweqeu4yum2ula0pdcf8c25mgk9jmmgsv90g20",
      codeId: 3156,
      digest: "9981335ad3c21e78f4a8e8ba4af4a3aeeed50bdaf26fd6f7074ab6ad604ff5bb"
    },
    hpl_mailbox: {
      address: "osmo1h0h5287kps9aehetgy8zpjnc3c3kdgrgx7m0kghqwx72zxe7wx2qnsqxp3",
      codeId: 3157,
      digest: "48e3725fa5d31f39b4fd63816cf9aadd0367bb4f8869b7adbb01fa64c3058c05"
    },
    hpl_multicall: {
      address: "osmo1dalwcenexewk8d4lnujm5w04mlwa5c7h7tqtfm6uz6gjstntekzqmptf0w",
      codeId: 3158,
      digest: "4f94814a642bce4da88e0c58d04f6c7f01926cc80c0c740d37230ff6fcb8315d"
    },
    hpl_validator_announce: {
      address: "osmo13qlwxa7f59a8gdysvvutsxa9zpqg5dxf3euy82rtyxw5xrenaq9qj5c8vm",
      codeId: 3159,
      digest: "bf66c1eff181e8aef2dbff550cd2f33f73d392a0d9aedac75a61fec85c115c04"
    }
  },
  latestMigration: "mailbox_related",
  address: "osmo109ns4u04l44kqdkvp876hukd3hxz8zzm7809el"
}
