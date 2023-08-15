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
      address: "osmo1yjgl5erq8v7u3u3gd8ysmzq85hclle8t8mu55h8x0yplz22w3dfq2cd07v",
      codeId: 3047,
      digest: "83bee2eede36470812330ed4c359aa3be611d7b0b974a140ff06e60c9252caf9"
    },
    hpl_igp_core: {
      address: "osmo1ln9e8tx9kfd5p4wvwglx4x3j9p8dnf2kgm6ghu5hpc3hhu2u80jsrwvmww",
      codeId: 3048,
      digest: "1eb6b6c2c5054c684a53bab7c1bdee469248c40536c5a0f411ff1577ac4d98b3"
    },
    hpl_igp_gas_oracle: {
      address: "osmo1hzg8e93knvlqmm0428lf2yv20uq9qqgq8x0zs2s2p7frtd35vcvq479749",
      codeId: 3049,
      digest: "004b8366f738d28496f83783072c621bb26940c5ffd8360c757a6ef14bf0e984"
    },
    hpl_ism_multisig: {
      address: "osmo1uxeqqfzfx0mhfmff2vug2myjfyj0s5a6tyuw4a83737ugrzm6mzspzv53k",
      codeId: 3050,
      digest: "fcbecc3ffa2171e7925e18734d5b3dfaed9eed4a2304fef5c5079c48f51cad2d"
    },
    hpl_ism_routing: {
      address: "osmo1k2qvs6nttn2yta2w5pd625duedjmur7p35gzq4pwxr6w965fu7pq720dh4",
      codeId: 3051,
      digest: "1829479923b36863f8f17f70d1ad79f9e9a449424e3fa48e77c1dcb621c1ef68"
    },
    hpl_mailbox: {
      address: "osmo1cmmk5j7nkv05xpl3r5qeczksru5uj9qv5qaeu58ucrsgkgx4hnssslrf0j",
      codeId: 3052,
      digest: "bba3d7d4ae9d831c1fa156705325ffb84e113abb8cf7c046b4a34175e23794c8"
    },
    hpl_multicall: {
      address: "osmo164uv2tcrlfrnfjrv33s6nrj3dms3r299yhnfhcjw5xc5ze0q9rwqvg2zpm",
      codeId: 3053,
      digest: "db54e81af9750b461c52a22f8c7587c9e8c2992aef10c9582bc15bd388addc1e"
    },
    hpl_validator_announce: {
      address: "osmo1cwrsx2sw8wajjvnusfx5pjdlg3hqlx4fnymuy0jmrrpe8qmnj90s6gshs0",
      codeId: 3054,
      digest: "4b0accbad42896c9ddb6e85df88e7b8301ab17cad1f4ce64cd38955a12af3e31"
    }
  },
  latestMigration: "mailbox_related",
  address: "osmo109ns4u04l44kqdkvp876hukd3hxz8zzm7809el"
}
