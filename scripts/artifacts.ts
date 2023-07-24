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
      address:
        "osmo1lnfl26u2g6zct0yt9ze0ykp0wtyjn86alffte8kjp0rs8078zypsf6u82k",
      codeId: 2295,
      digest:
        "83bee2eede36470812330ed4c359aa3be611d7b0b974a140ff06e60c9252caf9",
    },
    hpl_igp_core: {
      address:
        "osmo13535apg9jt6h95l7tq3sn3tjjxt7qkknpcem9n7tuue5tqefdfds2w338d",
      codeId: 2296,
      digest:
        "1eb6b6c2c5054c684a53bab7c1bdee469248c40536c5a0f411ff1577ac4d98b3",
    },
    hpl_igp_gas_oracle: {
      address:
        "osmo1nflgqcyrm2g5g2hzjwvznrmf74th6pdwh6va850plnlr3lrk7wxqsg3dnm",
      codeId: 2297,
      digest:
        "004b8366f738d28496f83783072c621bb26940c5ffd8360c757a6ef14bf0e984",
    },
    hpl_ism_multisig: {
      address:
        "osmo1zyscr9xkkma507gc0g9d34jjaw4un0gykl8nc33hf7vxtwp6vr9q874s2d",
      codeId: 2298,
      digest:
        "fcbecc3ffa2171e7925e18734d5b3dfaed9eed4a2304fef5c5079c48f51cad2d",
    },
    hpl_ism_routing: {
      address:
        "osmo1fgdfmw5yxj7qu3wtgfgss7kj6zawpyfr038qzglksakymhtwrkgsw22wdz",
      codeId: 2299,
      digest:
        "1829479923b36863f8f17f70d1ad79f9e9a449424e3fa48e77c1dcb621c1ef68",
    },
    hpl_mailbox: {
      address:
        "osmo1470mne23gkwx4szuhmqezw5tctrfv97ray05557mmrwr26460uvs4z6qht",
      codeId: 2300,
      digest:
        "678fcdda376e0caf7aa3b11afd697829cd2ab1a26f1e462140f4b46dafa4e0e4",
    },
    hpl_multicall: {
      address:
        "osmo1apcykrvqzhg5xeqxgm23rc6v5zflq3ctta25zge6w0fj4pu6dy3qxxlng3",
      codeId: 2301,
      digest:
        "db54e81af9750b461c52a22f8c7587c9e8c2992aef10c9582bc15bd388addc1e",
    },
    hpl_validator_announce: {
      address:
        "osmo16fhchx8qt5evpqux3235fu0t3q46wkxh0gl6veyx42mr7t398dqsme3y4z",
      codeId: 2302,
      digest:
        "4b0accbad42896c9ddb6e85df88e7b8301ab17cad1f4ce64cd38955a12af3e31",
    },
  },
  address: "osmo109ns4u04l44kqdkvp876hukd3hxz8zzm7809el",
  latestMigration: "mailbox_related",
};
