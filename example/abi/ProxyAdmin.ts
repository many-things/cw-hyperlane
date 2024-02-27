export const ProxyAdmin = {
  abi: [
    {
      type: "function",
      name: "changeProxyAdmin",
      inputs: [
        {
          name: "proxy",
          type: "address",
          internalType: "contract ITransparentUpgradeableProxy",
        },
        { name: "newAdmin", type: "address", internalType: "address" },
      ],
      outputs: [],
      stateMutability: "nonpayable",
    },
    {
      type: "function",
      name: "getProxyAdmin",
      inputs: [
        {
          name: "proxy",
          type: "address",
          internalType: "contract ITransparentUpgradeableProxy",
        },
      ],
      outputs: [{ name: "", type: "address", internalType: "address" }],
      stateMutability: "view",
    },
    {
      type: "function",
      name: "getProxyImplementation",
      inputs: [
        {
          name: "proxy",
          type: "address",
          internalType: "contract ITransparentUpgradeableProxy",
        },
      ],
      outputs: [{ name: "", type: "address", internalType: "address" }],
      stateMutability: "view",
    },
    {
      type: "function",
      name: "owner",
      inputs: [],
      outputs: [{ name: "", type: "address", internalType: "address" }],
      stateMutability: "view",
    },
    {
      type: "function",
      name: "renounceOwnership",
      inputs: [],
      outputs: [],
      stateMutability: "nonpayable",
    },
    {
      type: "function",
      name: "transferOwnership",
      inputs: [{ name: "newOwner", type: "address", internalType: "address" }],
      outputs: [],
      stateMutability: "nonpayable",
    },
    {
      type: "function",
      name: "upgrade",
      inputs: [
        {
          name: "proxy",
          type: "address",
          internalType: "contract ITransparentUpgradeableProxy",
        },
        { name: "implementation", type: "address", internalType: "address" },
      ],
      outputs: [],
      stateMutability: "nonpayable",
    },
    {
      type: "function",
      name: "upgradeAndCall",
      inputs: [
        {
          name: "proxy",
          type: "address",
          internalType: "contract ITransparentUpgradeableProxy",
        },
        { name: "implementation", type: "address", internalType: "address" },
        { name: "data", type: "bytes", internalType: "bytes" },
      ],
      outputs: [],
      stateMutability: "payable",
    },
    {
      type: "event",
      name: "OwnershipTransferred",
      inputs: [
        {
          name: "previousOwner",
          type: "address",
          indexed: true,
          internalType: "address",
        },
        {
          name: "newOwner",
          type: "address",
          indexed: true,
          internalType: "address",
        },
      ],
      anonymous: false,
    },
  ],
  bytecode: {
    object:
      "0x608060405234801561001057600080fd5b5061001a3361001f565b61006f565b600080546001600160a01b038381166001600160a01b0319831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b6108658061007e6000396000f3fe60806040526004361061007b5760003560e01c80639623609d1161004e5780639623609d1461012b57806399a88ec41461013e578063f2fde38b1461015e578063f3b7dead1461017e57600080fd5b8063204e1c7a14610080578063715018a6146100c95780637eff275e146100e05780638da5cb5b14610100575b600080fd5b34801561008c57600080fd5b506100a061009b366004610608565b61019e565b60405173ffffffffffffffffffffffffffffffffffffffff909116815260200160405180910390f35b3480156100d557600080fd5b506100de610255565b005b3480156100ec57600080fd5b506100de6100fb36600461062c565b610269565b34801561010c57600080fd5b5060005473ffffffffffffffffffffffffffffffffffffffff166100a0565b6100de610139366004610694565b6102f7565b34801561014a57600080fd5b506100de61015936600461062c565b61038c565b34801561016a57600080fd5b506100de610179366004610608565b6103e8565b34801561018a57600080fd5b506100a0610199366004610608565b6104a4565b60008060008373ffffffffffffffffffffffffffffffffffffffff166040516101ea907f5c60da1b00000000000000000000000000000000000000000000000000000000815260040190565b600060405180830381855afa9150503d8060008114610225576040519150601f19603f3d011682016040523d82523d6000602084013e61022a565b606091505b50915091508161023957600080fd5b8080602001905181019061024d9190610788565b949350505050565b61025d6104f0565b6102676000610571565b565b6102716104f0565b6040517f8f28397000000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8281166004830152831690638f283970906024015b600060405180830381600087803b1580156102db57600080fd5b505af11580156102ef573d6000803e3d6000fd5b505050505050565b6102ff6104f0565b6040517f4f1ef28600000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff841690634f1ef28690349061035590869086906004016107a5565b6000604051808303818588803b15801561036e57600080fd5b505af1158015610382573d6000803e3d6000fd5b5050505050505050565b6103946104f0565b6040517f3659cfe600000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8281166004830152831690633659cfe6906024016102c1565b6103f06104f0565b73ffffffffffffffffffffffffffffffffffffffff8116610498576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201527f646472657373000000000000000000000000000000000000000000000000000060648201526084015b60405180910390fd5b6104a181610571565b50565b60008060008373ffffffffffffffffffffffffffffffffffffffff166040516101ea907ff851a44000000000000000000000000000000000000000000000000000000000815260040190565b60005473ffffffffffffffffffffffffffffffffffffffff163314610267576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e6572604482015260640161048f565b6000805473ffffffffffffffffffffffffffffffffffffffff8381167fffffffffffffffffffffffff0000000000000000000000000000000000000000831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b73ffffffffffffffffffffffffffffffffffffffff811681146104a157600080fd5b60006020828403121561061a57600080fd5b8135610625816105e6565b9392505050565b6000806040838503121561063f57600080fd5b823561064a816105e6565b9150602083013561065a816105e6565b809150509250929050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b6000806000606084860312156106a957600080fd5b83356106b4816105e6565b925060208401356106c4816105e6565b9150604084013567ffffffffffffffff808211156106e157600080fd5b818601915086601f8301126106f557600080fd5b81358181111561070757610707610665565b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0908116603f0116810190838211818310171561074d5761074d610665565b8160405282815289602084870101111561076657600080fd5b8260208601602083013760006020848301015280955050505050509250925092565b60006020828403121561079a57600080fd5b8151610625816105e6565b73ffffffffffffffffffffffffffffffffffffffff8316815260006020604081840152835180604085015260005b818110156107ef578581018301518582016060015282016107d3565b5060006060828601015260607fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011685010192505050939250505056fea2646970667358221220d102f47386213fff51d4693f0c3b6ba1746ba7ef46cf6ccf410f039609c1be3564736f6c63430008130033",
    sourceMap:
      "450:2409:37:-:0;;;;;;;;;;;;-1:-1:-1;936:32:25;719:10:53;936:18:25;:32::i;:::-;450:2409:37;;2426:187:25;2499:16;2518:6;;-1:-1:-1;;;;;2534:17:25;;;-1:-1:-1;;;;;;2534:17:25;;;;;;2566:40;;2518:6;;;;;;;2566:40;;2499:16;2566:40;2489:124;2426:187;:::o;450:2409:37:-;;;;;;;",
    linkReferences: {},
  },
  deployedBytecode: {
    object:
      "0x60806040526004361061007b5760003560e01c80639623609d1161004e5780639623609d1461012b57806399a88ec41461013e578063f2fde38b1461015e578063f3b7dead1461017e57600080fd5b8063204e1c7a14610080578063715018a6146100c95780637eff275e146100e05780638da5cb5b14610100575b600080fd5b34801561008c57600080fd5b506100a061009b366004610608565b61019e565b60405173ffffffffffffffffffffffffffffffffffffffff909116815260200160405180910390f35b3480156100d557600080fd5b506100de610255565b005b3480156100ec57600080fd5b506100de6100fb36600461062c565b610269565b34801561010c57600080fd5b5060005473ffffffffffffffffffffffffffffffffffffffff166100a0565b6100de610139366004610694565b6102f7565b34801561014a57600080fd5b506100de61015936600461062c565b61038c565b34801561016a57600080fd5b506100de610179366004610608565b6103e8565b34801561018a57600080fd5b506100a0610199366004610608565b6104a4565b60008060008373ffffffffffffffffffffffffffffffffffffffff166040516101ea907f5c60da1b00000000000000000000000000000000000000000000000000000000815260040190565b600060405180830381855afa9150503d8060008114610225576040519150601f19603f3d011682016040523d82523d6000602084013e61022a565b606091505b50915091508161023957600080fd5b8080602001905181019061024d9190610788565b949350505050565b61025d6104f0565b6102676000610571565b565b6102716104f0565b6040517f8f28397000000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8281166004830152831690638f283970906024015b600060405180830381600087803b1580156102db57600080fd5b505af11580156102ef573d6000803e3d6000fd5b505050505050565b6102ff6104f0565b6040517f4f1ef28600000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff841690634f1ef28690349061035590869086906004016107a5565b6000604051808303818588803b15801561036e57600080fd5b505af1158015610382573d6000803e3d6000fd5b5050505050505050565b6103946104f0565b6040517f3659cfe600000000000000000000000000000000000000000000000000000000815273ffffffffffffffffffffffffffffffffffffffff8281166004830152831690633659cfe6906024016102c1565b6103f06104f0565b73ffffffffffffffffffffffffffffffffffffffff8116610498576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602660248201527f4f776e61626c653a206e6577206f776e657220697320746865207a65726f206160448201527f646472657373000000000000000000000000000000000000000000000000000060648201526084015b60405180910390fd5b6104a181610571565b50565b60008060008373ffffffffffffffffffffffffffffffffffffffff166040516101ea907ff851a44000000000000000000000000000000000000000000000000000000000815260040190565b60005473ffffffffffffffffffffffffffffffffffffffff163314610267576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820181905260248201527f4f776e61626c653a2063616c6c6572206973206e6f7420746865206f776e6572604482015260640161048f565b6000805473ffffffffffffffffffffffffffffffffffffffff8381167fffffffffffffffffffffffff0000000000000000000000000000000000000000831681178455604051919092169283917f8be0079c531659141344cd1fd0a4f28419497f9722a3daafe3b4186f6b6457e09190a35050565b73ffffffffffffffffffffffffffffffffffffffff811681146104a157600080fd5b60006020828403121561061a57600080fd5b8135610625816105e6565b9392505050565b6000806040838503121561063f57600080fd5b823561064a816105e6565b9150602083013561065a816105e6565b809150509250929050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b6000806000606084860312156106a957600080fd5b83356106b4816105e6565b925060208401356106c4816105e6565b9150604084013567ffffffffffffffff808211156106e157600080fd5b818601915086601f8301126106f557600080fd5b81358181111561070757610707610665565b604051601f82017fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0908116603f0116810190838211818310171561074d5761074d610665565b8160405282815289602084870101111561076657600080fd5b8260208601602083013760006020848301015280955050505050509250925092565b60006020828403121561079a57600080fd5b8151610625816105e6565b73ffffffffffffffffffffffffffffffffffffffff8316815260006020604081840152835180604085015260005b818110156107ef578581018301518582016060015282016107d3565b5060006060828601015260607fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0601f83011685010192505050939250505056fea2646970667358221220d102f47386213fff51d4693f0c3b6ba1746ba7ef46cf6ccf410f039609c1be3564736f6c63430008130033",
    sourceMap:
      "450:2409:37:-:0;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;650:438;;;;;;;;;;-1:-1:-1;650:438:37;;;;;:::i;:::-;;:::i;:::-;;;698:42:282;686:55;;;668:74;;656:2;641:18;650:438:37;;;;;;;1824:101:25;;;;;;;;;;;;;:::i;:::-;;1842:149:37;;;;;;;;;;-1:-1:-1;1842:149:37;;;;;:::i;:::-;;:::i;1201:85:25:-;;;;;;;;;;-1:-1:-1;1247:7:25;1273:6;;;1201:85;;2612:245:37;;;;;;:::i;:::-;;:::i;2196:150::-;;;;;;;;;;-1:-1:-1;2196:150:37;;;;;:::i;:::-;;:::i;2074:198:25:-;;;;;;;;;;-1:-1:-1;2074:198:25;;;;;:::i;:::-;;:::i;1248:420:37:-;;;;;;;;;;-1:-1:-1;1248:420:37;;;;;:::i;:::-;;:::i;650:438::-;747:7;924:12;938:23;973:5;965:25;;:40;;;;3285:66:282;3273:79;;3377:1;3368:11;;3072:313;965:40:37;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;923:82;;;;1023:7;1015:16;;;;;;1059:10;1048:33;;;;;;;;;;;;:::i;:::-;1041:40;650:438;-1:-1:-1;;;;650:438:37:o;1824:101:25:-;1094:13;:11;:13::i;:::-;1888:30:::1;1915:1;1888:18;:30::i;:::-;1824:101::o:0;1842:149:37:-;1094:13:25;:11;:13::i;:::-;1957:27:37::1;::::0;;;;:17:::1;686:55:282::0;;;1957:27:37::1;::::0;::::1;668:74:282::0;1957:17:37;::::1;::::0;::::1;::::0;641:18:282;;1957:27:37::1;;;;;;;;;;;;;;;;;;::::0;::::1;;;;;;;;;;;;::::0;::::1;;;;;;;;;1842:149:::0;;:::o;2612:245::-;1094:13:25;:11;:13::i;:::-;2788:62:37::1;::::0;;;;:22:::1;::::0;::::1;::::0;::::1;::::0;2818:9:::1;::::0;2788:62:::1;::::0;2829:14;;2845:4;;2788:62:::1;;;:::i;:::-;;;;;;;;;;;;;;;;;;::::0;::::1;;;;;;;;;;;;::::0;::::1;;;;;;;;;;2612:245:::0;;;:::o;2196:150::-;1094:13:25;:11;:13::i;:::-;2308:31:37::1;::::0;;;;:15:::1;686:55:282::0;;;2308:31:37::1;::::0;::::1;668:74:282::0;2308:15:37;::::1;::::0;::::1;::::0;641:18:282;;2308:31:37::1;522:226:282::0;2074:198:25;1094:13;:11;:13::i;:::-;2162:22:::1;::::0;::::1;2154:73;;;::::0;::::1;::::0;;4616:2:282;2154:73:25::1;::::0;::::1;4598:21:282::0;4655:2;4635:18;;;4628:30;4694:34;4674:18;;;4667:62;4765:8;4745:18;;;4738:36;4791:19;;2154:73:25::1;;;;;;;;;2237:28;2256:8;2237:18;:28::i;:::-;2074:198:::0;:::o;1248:420:37:-;1336:7;1504:12;1518:23;1553:5;1545:25;;:40;;;;5034:66:282;5022:79;;5126:1;5117:11;;4821:313;1359:130:25;1247:7;1273:6;1422:23;1273:6;719:10:53;1422:23:25;1414:68;;;;;;;5341:2:282;1414:68:25;;;5323:21:282;;;5360:18;;;5353:30;5419:34;5399:18;;;5392:62;5471:18;;1414:68:25;5139:356:282;2426:187:25;2499:16;2518:6;;;2534:17;;;;;;;;;;2566:40;;2518:6;;;;;;;2566:40;;2499:16;2566:40;2489:124;2426:187;:::o;14:184:282:-;130:42;123:5;119:54;112:5;109:65;99:93;;188:1;185;178:12;203:314;299:6;352:2;340:9;331:7;327:23;323:32;320:52;;;368:1;365;358:12;320:52;407:9;394:23;426:61;481:5;426:61;:::i;:::-;506:5;203:314;-1:-1:-1;;;203:314:282:o;753:485::-;858:6;866;919:2;907:9;898:7;894:23;890:32;887:52;;;935:1;932;925:12;887:52;974:9;961:23;993:61;1048:5;993:61;:::i;:::-;1073:5;-1:-1:-1;1130:2:282;1115:18;;1102:32;1143:63;1102:32;1143:63;:::i;:::-;1225:7;1215:17;;;753:485;;;;;:::o;1243:184::-;1295:77;1292:1;1285:88;1392:4;1389:1;1382:15;1416:4;1413:1;1406:15;1432:1353;1555:6;1563;1571;1624:2;1612:9;1603:7;1599:23;1595:32;1592:52;;;1640:1;1637;1630:12;1592:52;1679:9;1666:23;1698:61;1753:5;1698:61;:::i;:::-;1778:5;-1:-1:-1;1835:2:282;1820:18;;1807:32;1848:63;1807:32;1848:63;:::i;:::-;1930:7;-1:-1:-1;1988:2:282;1973:18;;1960:32;2011:18;2041:14;;;2038:34;;;2068:1;2065;2058:12;2038:34;2106:6;2095:9;2091:22;2081:32;;2151:7;2144:4;2140:2;2136:13;2132:27;2122:55;;2173:1;2170;2163:12;2122:55;2209:2;2196:16;2231:2;2227;2224:10;2221:36;;;2237:18;;:::i;:::-;2371:2;2365:9;2433:4;2425:13;;2276:66;2421:22;;;2445:2;2417:31;2413:40;2401:53;;;2469:18;;;2489:22;;;2466:46;2463:72;;;2515:18;;:::i;:::-;2555:10;2551:2;2544:22;2590:2;2582:6;2575:18;2630:7;2625:2;2620;2616;2612:11;2608:20;2605:33;2602:53;;;2651:1;2648;2641:12;2602:53;2707:2;2702;2698;2694:11;2689:2;2681:6;2677:15;2664:46;2752:1;2747:2;2742;2734:6;2730:15;2726:24;2719:35;2773:6;2763:16;;;;;;;1432:1353;;;;;:::o;3390:289::-;3468:6;3521:2;3509:9;3500:7;3496:23;3492:32;3489:52;;;3537:1;3534;3527:12;3489:52;3569:9;3563:16;3588:61;3643:5;3588:61;:::i;3684:725::-;3871:42;3863:6;3859:55;3848:9;3841:74;3822:4;3934:2;3972;3967;3956:9;3952:18;3945:30;4004:6;3998:13;4047:6;4042:2;4031:9;4027:18;4020:34;4072:1;4082:140;4096:6;4093:1;4090:13;4082:140;;;4191:14;;;4187:23;;4181:30;4157:17;;;4176:2;4153:26;4146:66;4111:10;;4082:140;;;4086:3;4271:1;4266:2;4257:6;4246:9;4242:22;4238:31;4231:42;4400:2;4330:66;4325:2;4317:6;4313:15;4309:88;4298:9;4294:104;4290:113;4282:121;;;;3684:725;;;;;:::o",
    linkReferences: {},
  },
  methodIdentifiers: {
    "changeProxyAdmin(address,address)": "7eff275e",
    "getProxyAdmin(address)": "f3b7dead",
    "getProxyImplementation(address)": "204e1c7a",
    "owner()": "8da5cb5b",
    "renounceOwnership()": "715018a6",
    "transferOwnership(address)": "f2fde38b",
    "upgrade(address,address)": "99a88ec4",
    "upgradeAndCall(address,address,bytes)": "9623609d",
  },
  rawMetadata:
    '{"compiler":{"version":"0.8.19+commit.7dd6d404"},"language":"Solidity","output":{"abi":[{"anonymous":false,"inputs":[{"indexed":true,"internalType":"address","name":"previousOwner","type":"address"},{"indexed":true,"internalType":"address","name":"newOwner","type":"address"}],"name":"OwnershipTransferred","type":"event"},{"inputs":[{"internalType":"contract ITransparentUpgradeableProxy","name":"proxy","type":"address"},{"internalType":"address","name":"newAdmin","type":"address"}],"name":"changeProxyAdmin","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"contract ITransparentUpgradeableProxy","name":"proxy","type":"address"}],"name":"getProxyAdmin","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[{"internalType":"contract ITransparentUpgradeableProxy","name":"proxy","type":"address"}],"name":"getProxyImplementation","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"owner","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"renounceOwnership","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address","name":"newOwner","type":"address"}],"name":"transferOwnership","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"contract ITransparentUpgradeableProxy","name":"proxy","type":"address"},{"internalType":"address","name":"implementation","type":"address"}],"name":"upgrade","outputs":[],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"contract ITransparentUpgradeableProxy","name":"proxy","type":"address"},{"internalType":"address","name":"implementation","type":"address"},{"internalType":"bytes","name":"data","type":"bytes"}],"name":"upgradeAndCall","outputs":[],"stateMutability":"payable","type":"function"}],"devdoc":{"details":"This is an auxiliary contract meant to be assigned as the admin of a {TransparentUpgradeableProxy}. For an explanation of why you would want to use this see the documentation for {TransparentUpgradeableProxy}.","kind":"dev","methods":{"changeProxyAdmin(address,address)":{"details":"Changes the admin of `proxy` to `newAdmin`. Requirements: - This contract must be the current admin of `proxy`."},"getProxyAdmin(address)":{"details":"Returns the current admin of `proxy`. Requirements: - This contract must be the admin of `proxy`."},"getProxyImplementation(address)":{"details":"Returns the current implementation of `proxy`. Requirements: - This contract must be the admin of `proxy`."},"owner()":{"details":"Returns the address of the current owner."},"renounceOwnership()":{"details":"Leaves the contract without owner. It will not be possible to call `onlyOwner` functions. Can only be called by the current owner. NOTE: Renouncing ownership will leave the contract without an owner, thereby disabling any functionality that is only available to the owner."},"transferOwnership(address)":{"details":"Transfers ownership of the contract to a new account (`newOwner`). Can only be called by the current owner."},"upgrade(address,address)":{"details":"Upgrades `proxy` to `implementation`. See {TransparentUpgradeableProxy-upgradeTo}. Requirements: - This contract must be the admin of `proxy`."},"upgradeAndCall(address,address,bytes)":{"details":"Upgrades `proxy` to `implementation` and calls a function on the new implementation. See {TransparentUpgradeableProxy-upgradeToAndCall}. Requirements: - This contract must be the admin of `proxy`."}},"version":1},"userdoc":{"kind":"user","methods":{},"version":1}},"settings":{"compilationTarget":{"../node_modules/@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol":"ProxyAdmin"},"evmVersion":"paris","libraries":{},"metadata":{"bytecodeHash":"ipfs"},"optimizer":{"enabled":true,"runs":999999},"remappings":[":@eth-optimism/=../node_modules/@eth-optimism/",":@openzeppelin/=../node_modules/@openzeppelin/",":ds-test/=lib/forge-std/lib/ds-test/src/",":forge-std/=lib/forge-std/src/"]},"sources":{"../node_modules/@openzeppelin/contracts/access/Ownable.sol":{"keccak256":"0xba43b97fba0d32eb4254f6a5a297b39a19a247082a02d6e69349e071e2946218","license":"MIT","urls":["bzz-raw://fc980984badf3984b6303b377711220e067722bbd6a135b24669ff5069ef9f32","dweb:/ipfs/QmPHXMSXj99XjSVM21YsY6aNtLLjLVXDbyN76J5HQYvvrz"]},"../node_modules/@openzeppelin/contracts/interfaces/IERC1967.sol":{"keccak256":"0x3cbef5ebc24b415252e2f8c0c9254555d30d9f085603b4b80d9b5ed20ab87e90","license":"MIT","urls":["bzz-raw://e8fa670c3bdce78e642cc6ae11c4cb38b133499cdce5e1990a9979d424703263","dweb:/ipfs/QmVxeCUk4jL2pXQyhsoNJwyU874wRufS2WvGe8TgPKPqhE"]},"../node_modules/@openzeppelin/contracts/interfaces/draft-IERC1822.sol":{"keccak256":"0x1d4afe6cb24200cc4545eed814ecf5847277dfe5d613a1707aad5fceecebcfff","license":"MIT","urls":["bzz-raw://383fb7b8181016ac5ccf07bc9cdb7c1b5045ea36e2cc4df52bcbf20396fc7688","dweb:/ipfs/QmYJ7Cg4WmE3rR8KGQxjUCXFfTH6TcwZ2Z1f6tPrq7jHFr"]},"../node_modules/@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol":{"keccak256":"0xa2b22da3032e50b55f95ec1d13336102d675f341167aa76db571ef7f8bb7975d","license":"MIT","urls":["bzz-raw://96b6d77a20bebd4eb06b801d3d020c7e82be13bd535cb0d0a6b7181c51dab5d5","dweb:/ipfs/QmPUR9Cv9jNFdQX6PtBfaBW1ZCnKwiu65R2VD5kbdanDyn"]},"../node_modules/@openzeppelin/contracts/proxy/ERC1967/ERC1967Upgrade.sol":{"keccak256":"0x3b21ae06bf5957f73fa16754b0669c77b7abd8ba6c072d35c3281d446fdb86c2","license":"MIT","urls":["bzz-raw://2db8e18505e86e02526847005d7287a33e397ed7fb9eaba3fd4a4a197add16e2","dweb:/ipfs/QmW9BSuKTzHWHBNSHF4L8XfVuU1uJrP2vLg84YtBd8mL82"]},"../node_modules/@openzeppelin/contracts/proxy/Proxy.sol":{"keccak256":"0xc130fe33f1b2132158531a87734153293f6d07bc263ff4ac90e85da9c82c0e27","license":"MIT","urls":["bzz-raw://8831721b6f4cc26534d190f9f1631c3f59c9ff38efdd911f85e0882b8e360472","dweb:/ipfs/QmQZnLErZNStirSQ13ZNWQgvEYUtGE5tXYwn4QUPaVUfPN"]},"../node_modules/@openzeppelin/contracts/proxy/beacon/IBeacon.sol":{"keccak256":"0xd50a3421ac379ccb1be435fa646d66a65c986b4924f0849839f08692f39dde61","license":"MIT","urls":["bzz-raw://ada1e030c0231db8d143b44ce92b4d1158eedb087880cad6d8cc7bd7ebe7b354","dweb:/ipfs/QmWZ2NHZweRpz1U9GF6R1h65ri76dnX7fNxLBeM2t5N5Ce"]},"../node_modules/@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol":{"keccak256":"0x8e99882a991853dc446278576c8cb9b3a5ded84642e9bcc917b1677807c2f18c","license":"MIT","urls":["bzz-raw://310153c1a4c739002ffbc1351ed1dd7488a0d20f5dd816353332fc2c1d81e0a3","dweb:/ipfs/QmcvwXQVUBRTEAoNcvwSVFmhpHUXQ21s2Hfj79hq2uQNVM"]},"../node_modules/@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol":{"keccak256":"0x168e36d7e616bd41f6abab4a83009da64513ae9e638aa6d5980066e2a92db689","license":"MIT","urls":["bzz-raw://a45c64b97311fabbcbe8dad7e94fa89e06a7f96060d5565326ef706f5f239017","dweb:/ipfs/QmeU2jiBGbHhz9DqRotjbpAx5s2xExDSRQtSD5ENjuHzDq"]},"../node_modules/@openzeppelin/contracts/utils/Address.sol":{"keccak256":"0x006dd67219697fe68d7fbfdea512e7c4cb64a43565ed86171d67e844982da6fa","license":"MIT","urls":["bzz-raw://2455248c8ddd9cc6a7af76a13973cddf222072427e7b0e2a7d1aff345145e931","dweb:/ipfs/QmfYjnjRbWqYpuxurqveE6HtzsY1Xx323J428AKQgtBJZm"]},"../node_modules/@openzeppelin/contracts/utils/Context.sol":{"keccak256":"0xe2e337e6dde9ef6b680e07338c493ebea1b5fd09b43424112868e9cc1706bca7","license":"MIT","urls":["bzz-raw://6df0ddf21ce9f58271bdfaa85cde98b200ef242a05a3f85c2bc10a8294800a92","dweb:/ipfs/QmRK2Y5Yc6BK7tGKkgsgn3aJEQGi5aakeSPZvS65PV8Xp3"]},"../node_modules/@openzeppelin/contracts/utils/StorageSlot.sol":{"keccak256":"0xf09e68aa0dc6722a25bc46490e8d48ed864466d17313b8a0b254c36b54e49899","license":"MIT","urls":["bzz-raw://e26daf81e2252dc1fe1ce0e4b55c2eb7c6d1ee84ae6558d1a9554432ea1d32da","dweb:/ipfs/Qmb1UANWiWq5pCKbmHSu772hd4nt374dVaghGmwSVNuk8Q"]}},"version":1}',
  metadata: {
    compiler: { version: "0.8.19+commit.7dd6d404" },
    language: "Solidity",
    output: {
      abi: [
        {
          inputs: [
            {
              internalType: "address",
              name: "previousOwner",
              type: "address",
              indexed: true,
            },
            {
              internalType: "address",
              name: "newOwner",
              type: "address",
              indexed: true,
            },
          ],
          type: "event",
          name: "OwnershipTransferred",
          anonymous: false,
        },
        {
          inputs: [
            {
              internalType: "contract ITransparentUpgradeableProxy",
              name: "proxy",
              type: "address",
            },
            { internalType: "address", name: "newAdmin", type: "address" },
          ],
          stateMutability: "nonpayable",
          type: "function",
          name: "changeProxyAdmin",
        },
        {
          inputs: [
            {
              internalType: "contract ITransparentUpgradeableProxy",
              name: "proxy",
              type: "address",
            },
          ],
          stateMutability: "view",
          type: "function",
          name: "getProxyAdmin",
          outputs: [{ internalType: "address", name: "", type: "address" }],
        },
        {
          inputs: [
            {
              internalType: "contract ITransparentUpgradeableProxy",
              name: "proxy",
              type: "address",
            },
          ],
          stateMutability: "view",
          type: "function",
          name: "getProxyImplementation",
          outputs: [{ internalType: "address", name: "", type: "address" }],
        },
        {
          inputs: [],
          stateMutability: "view",
          type: "function",
          name: "owner",
          outputs: [{ internalType: "address", name: "", type: "address" }],
        },
        {
          inputs: [],
          stateMutability: "nonpayable",
          type: "function",
          name: "renounceOwnership",
        },
        {
          inputs: [
            { internalType: "address", name: "newOwner", type: "address" },
          ],
          stateMutability: "nonpayable",
          type: "function",
          name: "transferOwnership",
        },
        {
          inputs: [
            {
              internalType: "contract ITransparentUpgradeableProxy",
              name: "proxy",
              type: "address",
            },
            {
              internalType: "address",
              name: "implementation",
              type: "address",
            },
          ],
          stateMutability: "nonpayable",
          type: "function",
          name: "upgrade",
        },
        {
          inputs: [
            {
              internalType: "contract ITransparentUpgradeableProxy",
              name: "proxy",
              type: "address",
            },
            {
              internalType: "address",
              name: "implementation",
              type: "address",
            },
            { internalType: "bytes", name: "data", type: "bytes" },
          ],
          stateMutability: "payable",
          type: "function",
          name: "upgradeAndCall",
        },
      ],
      devdoc: {
        kind: "dev",
        methods: {
          "changeProxyAdmin(address,address)": {
            details:
              "Changes the admin of `proxy` to `newAdmin`. Requirements: - This contract must be the current admin of `proxy`.",
          },
          "getProxyAdmin(address)": {
            details:
              "Returns the current admin of `proxy`. Requirements: - This contract must be the admin of `proxy`.",
          },
          "getProxyImplementation(address)": {
            details:
              "Returns the current implementation of `proxy`. Requirements: - This contract must be the admin of `proxy`.",
          },
          "owner()": { details: "Returns the address of the current owner." },
          "renounceOwnership()": {
            details:
              "Leaves the contract without owner. It will not be possible to call `onlyOwner` functions. Can only be called by the current owner. NOTE: Renouncing ownership will leave the contract without an owner, thereby disabling any functionality that is only available to the owner.",
          },
          "transferOwnership(address)": {
            details:
              "Transfers ownership of the contract to a new account (`newOwner`). Can only be called by the current owner.",
          },
          "upgrade(address,address)": {
            details:
              "Upgrades `proxy` to `implementation`. See {TransparentUpgradeableProxy-upgradeTo}. Requirements: - This contract must be the admin of `proxy`.",
          },
          "upgradeAndCall(address,address,bytes)": {
            details:
              "Upgrades `proxy` to `implementation` and calls a function on the new implementation. See {TransparentUpgradeableProxy-upgradeToAndCall}. Requirements: - This contract must be the admin of `proxy`.",
          },
        },
        version: 1,
      },
      userdoc: { kind: "user", methods: {}, version: 1 },
    },
    settings: {
      remappings: [
        "@eth-optimism/=../node_modules/@eth-optimism/",
        "@openzeppelin/=../node_modules/@openzeppelin/",
        "ds-test/=lib/forge-std/lib/ds-test/src/",
        "forge-std/=lib/forge-std/src/",
      ],
      optimizer: { enabled: true, runs: 999999 },
      metadata: { bytecodeHash: "ipfs" },
      compilationTarget: {
        "../node_modules/@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol":
          "ProxyAdmin",
      },
      evmVersion: "paris",
      libraries: {},
    },
    sources: {
      "../node_modules/@openzeppelin/contracts/access/Ownable.sol": {
        keccak256:
          "0xba43b97fba0d32eb4254f6a5a297b39a19a247082a02d6e69349e071e2946218",
        urls: [
          "bzz-raw://fc980984badf3984b6303b377711220e067722bbd6a135b24669ff5069ef9f32",
          "dweb:/ipfs/QmPHXMSXj99XjSVM21YsY6aNtLLjLVXDbyN76J5HQYvvrz",
        ],
        license: "MIT",
      },
      "../node_modules/@openzeppelin/contracts/interfaces/IERC1967.sol": {
        keccak256:
          "0x3cbef5ebc24b415252e2f8c0c9254555d30d9f085603b4b80d9b5ed20ab87e90",
        urls: [
          "bzz-raw://e8fa670c3bdce78e642cc6ae11c4cb38b133499cdce5e1990a9979d424703263",
          "dweb:/ipfs/QmVxeCUk4jL2pXQyhsoNJwyU874wRufS2WvGe8TgPKPqhE",
        ],
        license: "MIT",
      },
      "../node_modules/@openzeppelin/contracts/interfaces/draft-IERC1822.sol": {
        keccak256:
          "0x1d4afe6cb24200cc4545eed814ecf5847277dfe5d613a1707aad5fceecebcfff",
        urls: [
          "bzz-raw://383fb7b8181016ac5ccf07bc9cdb7c1b5045ea36e2cc4df52bcbf20396fc7688",
          "dweb:/ipfs/QmYJ7Cg4WmE3rR8KGQxjUCXFfTH6TcwZ2Z1f6tPrq7jHFr",
        ],
        license: "MIT",
      },
      "../node_modules/@openzeppelin/contracts/proxy/ERC1967/ERC1967Proxy.sol":
        {
          keccak256:
            "0xa2b22da3032e50b55f95ec1d13336102d675f341167aa76db571ef7f8bb7975d",
          urls: [
            "bzz-raw://96b6d77a20bebd4eb06b801d3d020c7e82be13bd535cb0d0a6b7181c51dab5d5",
            "dweb:/ipfs/QmPUR9Cv9jNFdQX6PtBfaBW1ZCnKwiu65R2VD5kbdanDyn",
          ],
          license: "MIT",
        },
      "../node_modules/@openzeppelin/contracts/proxy/ERC1967/ERC1967Upgrade.sol":
        {
          keccak256:
            "0x3b21ae06bf5957f73fa16754b0669c77b7abd8ba6c072d35c3281d446fdb86c2",
          urls: [
            "bzz-raw://2db8e18505e86e02526847005d7287a33e397ed7fb9eaba3fd4a4a197add16e2",
            "dweb:/ipfs/QmW9BSuKTzHWHBNSHF4L8XfVuU1uJrP2vLg84YtBd8mL82",
          ],
          license: "MIT",
        },
      "../node_modules/@openzeppelin/contracts/proxy/Proxy.sol": {
        keccak256:
          "0xc130fe33f1b2132158531a87734153293f6d07bc263ff4ac90e85da9c82c0e27",
        urls: [
          "bzz-raw://8831721b6f4cc26534d190f9f1631c3f59c9ff38efdd911f85e0882b8e360472",
          "dweb:/ipfs/QmQZnLErZNStirSQ13ZNWQgvEYUtGE5tXYwn4QUPaVUfPN",
        ],
        license: "MIT",
      },
      "../node_modules/@openzeppelin/contracts/proxy/beacon/IBeacon.sol": {
        keccak256:
          "0xd50a3421ac379ccb1be435fa646d66a65c986b4924f0849839f08692f39dde61",
        urls: [
          "bzz-raw://ada1e030c0231db8d143b44ce92b4d1158eedb087880cad6d8cc7bd7ebe7b354",
          "dweb:/ipfs/QmWZ2NHZweRpz1U9GF6R1h65ri76dnX7fNxLBeM2t5N5Ce",
        ],
        license: "MIT",
      },
      "../node_modules/@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol":
        {
          keccak256:
            "0x8e99882a991853dc446278576c8cb9b3a5ded84642e9bcc917b1677807c2f18c",
          urls: [
            "bzz-raw://310153c1a4c739002ffbc1351ed1dd7488a0d20f5dd816353332fc2c1d81e0a3",
            "dweb:/ipfs/QmcvwXQVUBRTEAoNcvwSVFmhpHUXQ21s2Hfj79hq2uQNVM",
          ],
          license: "MIT",
        },
      "../node_modules/@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol":
        {
          keccak256:
            "0x168e36d7e616bd41f6abab4a83009da64513ae9e638aa6d5980066e2a92db689",
          urls: [
            "bzz-raw://a45c64b97311fabbcbe8dad7e94fa89e06a7f96060d5565326ef706f5f239017",
            "dweb:/ipfs/QmeU2jiBGbHhz9DqRotjbpAx5s2xExDSRQtSD5ENjuHzDq",
          ],
          license: "MIT",
        },
      "../node_modules/@openzeppelin/contracts/utils/Address.sol": {
        keccak256:
          "0x006dd67219697fe68d7fbfdea512e7c4cb64a43565ed86171d67e844982da6fa",
        urls: [
          "bzz-raw://2455248c8ddd9cc6a7af76a13973cddf222072427e7b0e2a7d1aff345145e931",
          "dweb:/ipfs/QmfYjnjRbWqYpuxurqveE6HtzsY1Xx323J428AKQgtBJZm",
        ],
        license: "MIT",
      },
      "../node_modules/@openzeppelin/contracts/utils/Context.sol": {
        keccak256:
          "0xe2e337e6dde9ef6b680e07338c493ebea1b5fd09b43424112868e9cc1706bca7",
        urls: [
          "bzz-raw://6df0ddf21ce9f58271bdfaa85cde98b200ef242a05a3f85c2bc10a8294800a92",
          "dweb:/ipfs/QmRK2Y5Yc6BK7tGKkgsgn3aJEQGi5aakeSPZvS65PV8Xp3",
        ],
        license: "MIT",
      },
      "../node_modules/@openzeppelin/contracts/utils/StorageSlot.sol": {
        keccak256:
          "0xf09e68aa0dc6722a25bc46490e8d48ed864466d17313b8a0b254c36b54e49899",
        urls: [
          "bzz-raw://e26daf81e2252dc1fe1ce0e4b55c2eb7c6d1ee84ae6558d1a9554432ea1d32da",
          "dweb:/ipfs/Qmb1UANWiWq5pCKbmHSu772hd4nt374dVaghGmwSVNuk8Q",
        ],
        license: "MIT",
      },
    },
    version: 1,
  },
  ast: {
    absolutePath:
      "../node_modules/@openzeppelin/contracts/proxy/transparent/ProxyAdmin.sol",
    id: 3866,
    exportedSymbols: {
      Address: [6155],
      Context: [80709],
      ERC1967Proxy: [81197],
      ERC1967Upgrade: [84616],
      IBeacon: [84736],
      IERC1822Proxiable: [84726],
      IERC1967: [84716],
      ITransparentUpgradeableProxy: [3899],
      Ownable: [2723],
      Proxy: [84668],
      ProxyAdmin: [3865],
      StorageSlot: [84846],
      TransparentUpgradeableProxy: [4188],
    },
    nodeType: "SourceUnit",
    src: "116:2744:37",
    nodes: [
      {
        id: 3731,
        nodeType: "PragmaDirective",
        src: "116:23:37",
        nodes: [],
        literals: ["solidity", "^", "0.8", ".0"],
      },
      {
        id: 3732,
        nodeType: "ImportDirective",
        src: "141:43:37",
        nodes: [],
        absolutePath:
          "../node_modules/@openzeppelin/contracts/proxy/transparent/TransparentUpgradeableProxy.sol",
        file: "./TransparentUpgradeableProxy.sol",
        nameLocation: "-1:-1:-1",
        scope: 3866,
        sourceUnit: 4189,
        symbolAliases: [],
        unitAlias: "",
      },
      {
        id: 3733,
        nodeType: "ImportDirective",
        src: "185:34:37",
        nodes: [],
        absolutePath:
          "../node_modules/@openzeppelin/contracts/access/Ownable.sol",
        file: "../../access/Ownable.sol",
        nameLocation: "-1:-1:-1",
        scope: 3866,
        sourceUnit: 2724,
        symbolAliases: [],
        unitAlias: "",
      },
      {
        id: 3865,
        nodeType: "ContractDefinition",
        src: "450:2409:37",
        nodes: [
          {
            id: 3770,
            nodeType: "FunctionDefinition",
            src: "650:438:37",
            nodes: [],
            body: {
              id: 3769,
              nodeType: "Block",
              src: "756:332:37",
              nodes: [],
              statements: [
                {
                  assignments: [3746, 3748],
                  declarations: [
                    {
                      constant: false,
                      id: 3746,
                      mutability: "mutable",
                      name: "success",
                      nameLocation: "929:7:37",
                      nodeType: "VariableDeclaration",
                      scope: 3769,
                      src: "924:12:37",
                      stateVariable: false,
                      storageLocation: "default",
                      typeDescriptions: {
                        typeIdentifier: "t_bool",
                        typeString: "bool",
                      },
                      typeName: {
                        id: 3745,
                        name: "bool",
                        nodeType: "ElementaryTypeName",
                        src: "924:4:37",
                        typeDescriptions: {
                          typeIdentifier: "t_bool",
                          typeString: "bool",
                        },
                      },
                      visibility: "internal",
                    },
                    {
                      constant: false,
                      id: 3748,
                      mutability: "mutable",
                      name: "returndata",
                      nameLocation: "951:10:37",
                      nodeType: "VariableDeclaration",
                      scope: 3769,
                      src: "938:23:37",
                      stateVariable: false,
                      storageLocation: "memory",
                      typeDescriptions: {
                        typeIdentifier: "t_bytes_memory_ptr",
                        typeString: "bytes",
                      },
                      typeName: {
                        id: 3747,
                        name: "bytes",
                        nodeType: "ElementaryTypeName",
                        src: "938:5:37",
                        typeDescriptions: {
                          typeIdentifier: "t_bytes_storage_ptr",
                          typeString: "bytes",
                        },
                      },
                      visibility: "internal",
                    },
                  ],
                  id: 3756,
                  initialValue: {
                    arguments: [
                      {
                        hexValue: "5c60da1b",
                        id: 3754,
                        isConstant: false,
                        isLValue: false,
                        isPure: true,
                        kind: "hexString",
                        lValueRequested: false,
                        nodeType: "Literal",
                        src: "991:13:37",
                        typeDescriptions: {
                          typeIdentifier:
                            "t_stringliteral_96a4c6be7716f5be15d118c16bd1d464cb27f01187d0b9218993a5d488a47c29",
                          typeString: 'literal_string hex"5c60da1b"',
                        },
                      },
                    ],
                    expression: {
                      argumentTypes: [
                        {
                          typeIdentifier:
                            "t_stringliteral_96a4c6be7716f5be15d118c16bd1d464cb27f01187d0b9218993a5d488a47c29",
                          typeString: 'literal_string hex"5c60da1b"',
                        },
                      ],
                      expression: {
                        arguments: [
                          {
                            id: 3751,
                            name: "proxy",
                            nodeType: "Identifier",
                            overloadedDeclarations: [],
                            referencedDeclaration: 3740,
                            src: "973:5:37",
                            typeDescriptions: {
                              typeIdentifier:
                                "t_contract$_ITransparentUpgradeableProxy_$3899",
                              typeString:
                                "contract ITransparentUpgradeableProxy",
                            },
                          },
                        ],
                        expression: {
                          argumentTypes: [
                            {
                              typeIdentifier:
                                "t_contract$_ITransparentUpgradeableProxy_$3899",
                              typeString:
                                "contract ITransparentUpgradeableProxy",
                            },
                          ],
                          id: 3750,
                          isConstant: false,
                          isLValue: false,
                          isPure: true,
                          lValueRequested: false,
                          nodeType: "ElementaryTypeNameExpression",
                          src: "965:7:37",
                          typeDescriptions: {
                            typeIdentifier: "t_type$_t_address_$",
                            typeString: "type(address)",
                          },
                          typeName: {
                            id: 3749,
                            name: "address",
                            nodeType: "ElementaryTypeName",
                            src: "965:7:37",
                            typeDescriptions: {},
                          },
                        },
                        id: 3752,
                        isConstant: false,
                        isLValue: false,
                        isPure: false,
                        kind: "typeConversion",
                        lValueRequested: false,
                        nameLocations: [],
                        names: [],
                        nodeType: "FunctionCall",
                        src: "965:14:37",
                        tryCall: false,
                        typeDescriptions: {
                          typeIdentifier: "t_address",
                          typeString: "address",
                        },
                      },
                      id: 3753,
                      isConstant: false,
                      isLValue: false,
                      isPure: false,
                      lValueRequested: false,
                      memberLocation: "980:10:37",
                      memberName: "staticcall",
                      nodeType: "MemberAccess",
                      src: "965:25:37",
                      typeDescriptions: {
                        typeIdentifier:
                          "t_function_barestaticcall_view$_t_bytes_memory_ptr_$returns$_t_bool_$_t_bytes_memory_ptr_$",
                        typeString:
                          "function (bytes memory) view returns (bool,bytes memory)",
                      },
                    },
                    id: 3755,
                    isConstant: false,
                    isLValue: false,
                    isPure: false,
                    kind: "functionCall",
                    lValueRequested: false,
                    nameLocations: [],
                    names: [],
                    nodeType: "FunctionCall",
                    src: "965:40:37",
                    tryCall: false,
                    typeDescriptions: {
                      typeIdentifier: "t_tuple$_t_bool_$_t_bytes_memory_ptr_$",
                      typeString: "tuple(bool,bytes memory)",
                    },
                  },
                  nodeType: "VariableDeclarationStatement",
                  src: "923:82:37",
                },
                {
                  expression: {
                    arguments: [
                      {
                        id: 3758,
                        name: "success",
                        nodeType: "Identifier",
                        overloadedDeclarations: [],
                        referencedDeclaration: 3746,
                        src: "1023:7:37",
                        typeDescriptions: {
                          typeIdentifier: "t_bool",
                          typeString: "bool",
                        },
                      },
                    ],
                    expression: {
                      argumentTypes: [
                        { typeIdentifier: "t_bool", typeString: "bool" },
                      ],
                      id: 3757,
                      name: "require",
                      nodeType: "Identifier",
                      overloadedDeclarations: [-18, -18],
                      referencedDeclaration: -18,
                      src: "1015:7:37",
                      typeDescriptions: {
                        typeIdentifier:
                          "t_function_require_pure$_t_bool_$returns$__$",
                        typeString: "function (bool) pure",
                      },
                    },
                    id: 3759,
                    isConstant: false,
                    isLValue: false,
                    isPure: false,
                    kind: "functionCall",
                    lValueRequested: false,
                    nameLocations: [],
                    names: [],
                    nodeType: "FunctionCall",
                    src: "1015:16:37",
                    tryCall: false,
                    typeDescriptions: {
                      typeIdentifier: "t_tuple$__$",
                      typeString: "tuple()",
                    },
                  },
                  id: 3760,
                  nodeType: "ExpressionStatement",
                  src: "1015:16:37",
                },
                {
                  expression: {
                    arguments: [
                      {
                        id: 3763,
                        name: "returndata",
                        nodeType: "Identifier",
                        overloadedDeclarations: [],
                        referencedDeclaration: 3748,
                        src: "1059:10:37",
                        typeDescriptions: {
                          typeIdentifier: "t_bytes_memory_ptr",
                          typeString: "bytes memory",
                        },
                      },
                      {
                        components: [
                          {
                            id: 3765,
                            isConstant: false,
                            isLValue: false,
                            isPure: true,
                            lValueRequested: false,
                            nodeType: "ElementaryTypeNameExpression",
                            src: "1072:7:37",
                            typeDescriptions: {
                              typeIdentifier: "t_type$_t_address_$",
                              typeString: "type(address)",
                            },
                            typeName: {
                              id: 3764,
                              name: "address",
                              nodeType: "ElementaryTypeName",
                              src: "1072:7:37",
                              typeDescriptions: {},
                            },
                          },
                        ],
                        id: 3766,
                        isConstant: false,
                        isInlineArray: false,
                        isLValue: false,
                        isPure: true,
                        lValueRequested: false,
                        nodeType: "TupleExpression",
                        src: "1071:9:37",
                        typeDescriptions: {
                          typeIdentifier: "t_type$_t_address_$",
                          typeString: "type(address)",
                        },
                      },
                    ],
                    expression: {
                      argumentTypes: [
                        {
                          typeIdentifier: "t_bytes_memory_ptr",
                          typeString: "bytes memory",
                        },
                        {
                          typeIdentifier: "t_type$_t_address_$",
                          typeString: "type(address)",
                        },
                      ],
                      expression: {
                        id: 3761,
                        name: "abi",
                        nodeType: "Identifier",
                        overloadedDeclarations: [],
                        referencedDeclaration: -1,
                        src: "1048:3:37",
                        typeDescriptions: {
                          typeIdentifier: "t_magic_abi",
                          typeString: "abi",
                        },
                      },
                      id: 3762,
                      isConstant: false,
                      isLValue: false,
                      isPure: true,
                      lValueRequested: false,
                      memberLocation: "1052:6:37",
                      memberName: "decode",
                      nodeType: "MemberAccess",
                      src: "1048:10:37",
                      typeDescriptions: {
                        typeIdentifier:
                          "t_function_abidecode_pure$__$returns$__$",
                        typeString: "function () pure",
                      },
                    },
                    id: 3767,
                    isConstant: false,
                    isLValue: false,
                    isPure: false,
                    kind: "functionCall",
                    lValueRequested: false,
                    nameLocations: [],
                    names: [],
                    nodeType: "FunctionCall",
                    src: "1048:33:37",
                    tryCall: false,
                    typeDescriptions: {
                      typeIdentifier: "t_address_payable",
                      typeString: "address payable",
                    },
                  },
                  functionReturnParameters: 3744,
                  id: 3768,
                  nodeType: "Return",
                  src: "1041:40:37",
                },
              ],
            },
            documentation: {
              id: 3737,
              nodeType: "StructuredDocumentation",
              src: "487:158:37",
              text: " @dev Returns the current implementation of `proxy`.\n Requirements:\n - This contract must be the admin of `proxy`.",
            },
            functionSelector: "204e1c7a",
            implemented: true,
            kind: "function",
            modifiers: [],
            name: "getProxyImplementation",
            nameLocation: "659:22:37",
            parameters: {
              id: 3741,
              nodeType: "ParameterList",
              parameters: [
                {
                  constant: false,
                  id: 3740,
                  mutability: "mutable",
                  name: "proxy",
                  nameLocation: "711:5:37",
                  nodeType: "VariableDeclaration",
                  scope: 3770,
                  src: "682:34:37",
                  stateVariable: false,
                  storageLocation: "default",
                  typeDescriptions: {
                    typeIdentifier:
                      "t_contract$_ITransparentUpgradeableProxy_$3899",
                    typeString: "contract ITransparentUpgradeableProxy",
                  },
                  typeName: {
                    id: 3739,
                    nodeType: "UserDefinedTypeName",
                    pathNode: {
                      id: 3738,
                      name: "ITransparentUpgradeableProxy",
                      nameLocations: ["682:28:37"],
                      nodeType: "IdentifierPath",
                      referencedDeclaration: 3899,
                      src: "682:28:37",
                    },
                    referencedDeclaration: 3899,
                    src: "682:28:37",
                    typeDescriptions: {
                      typeIdentifier:
                        "t_contract$_ITransparentUpgradeableProxy_$3899",
                      typeString: "contract ITransparentUpgradeableProxy",
                    },
                  },
                  visibility: "internal",
                },
              ],
              src: "681:36:37",
            },
            returnParameters: {
              id: 3744,
              nodeType: "ParameterList",
              parameters: [
                {
                  constant: false,
                  id: 3743,
                  mutability: "mutable",
                  name: "",
                  nameLocation: "-1:-1:-1",
                  nodeType: "VariableDeclaration",
                  scope: 3770,
                  src: "747:7:37",
                  stateVariable: false,
                  storageLocation: "default",
                  typeDescriptions: {
                    typeIdentifier: "t_address",
                    typeString: "address",
                  },
                  typeName: {
                    id: 3742,
                    name: "address",
                    nodeType: "ElementaryTypeName",
                    src: "747:7:37",
                    stateMutability: "nonpayable",
                    typeDescriptions: {
                      typeIdentifier: "t_address",
                      typeString: "address",
                    },
                  },
                  visibility: "internal",
                },
              ],
              src: "746:9:37",
            },
            scope: 3865,
            stateMutability: "view",
            virtual: true,
            visibility: "public",
          },
          {
            id: 3804,
            nodeType: "FunctionDefinition",
            src: "1248:420:37",
            nodes: [],
            body: {
              id: 3803,
              nodeType: "Block",
              src: "1345:323:37",
              nodes: [],
              statements: [
                {
                  assignments: [3780, 3782],
                  declarations: [
                    {
                      constant: false,
                      id: 3780,
                      mutability: "mutable",
                      name: "success",
                      nameLocation: "1509:7:37",
                      nodeType: "VariableDeclaration",
                      scope: 3803,
                      src: "1504:12:37",
                      stateVariable: false,
                      storageLocation: "default",
                      typeDescriptions: {
                        typeIdentifier: "t_bool",
                        typeString: "bool",
                      },
                      typeName: {
                        id: 3779,
                        name: "bool",
                        nodeType: "ElementaryTypeName",
                        src: "1504:4:37",
                        typeDescriptions: {
                          typeIdentifier: "t_bool",
                          typeString: "bool",
                        },
                      },
                      visibility: "internal",
                    },
                    {
                      constant: false,
                      id: 3782,
                      mutability: "mutable",
                      name: "returndata",
                      nameLocation: "1531:10:37",
                      nodeType: "VariableDeclaration",
                      scope: 3803,
                      src: "1518:23:37",
                      stateVariable: false,
                      storageLocation: "memory",
                      typeDescriptions: {
                        typeIdentifier: "t_bytes_memory_ptr",
                        typeString: "bytes",
                      },
                      typeName: {
                        id: 3781,
                        name: "bytes",
                        nodeType: "ElementaryTypeName",
                        src: "1518:5:37",
                        typeDescriptions: {
                          typeIdentifier: "t_bytes_storage_ptr",
                          typeString: "bytes",
                        },
                      },
                      visibility: "internal",
                    },
                  ],
                  id: 3790,
                  initialValue: {
                    arguments: [
                      {
                        hexValue: "f851a440",
                        id: 3788,
                        isConstant: false,
                        isLValue: false,
                        isPure: true,
                        kind: "hexString",
                        lValueRequested: false,
                        nodeType: "Literal",
                        src: "1571:13:37",
                        typeDescriptions: {
                          typeIdentifier:
                            "t_stringliteral_cb23cf6c353ccb16f0d92c8e6b5c5b425654e65dd07e2d295b394de4cf15afb7",
                          typeString: 'literal_string hex"f851a440"',
                        },
                      },
                    ],
                    expression: {
                      argumentTypes: [
                        {
                          typeIdentifier:
                            "t_stringliteral_cb23cf6c353ccb16f0d92c8e6b5c5b425654e65dd07e2d295b394de4cf15afb7",
                          typeString: 'literal_string hex"f851a440"',
                        },
                      ],
                      expression: {
                        arguments: [
                          {
                            id: 3785,
                            name: "proxy",
                            nodeType: "Identifier",
                            overloadedDeclarations: [],
                            referencedDeclaration: 3774,
                            src: "1553:5:37",
                            typeDescriptions: {
                              typeIdentifier:
                                "t_contract$_ITransparentUpgradeableProxy_$3899",
                              typeString:
                                "contract ITransparentUpgradeableProxy",
                            },
                          },
                        ],
                        expression: {
                          argumentTypes: [
                            {
                              typeIdentifier:
                                "t_contract$_ITransparentUpgradeableProxy_$3899",
                              typeString:
                                "contract ITransparentUpgradeableProxy",
                            },
                          ],
                          id: 3784,
                          isConstant: false,
                          isLValue: false,
                          isPure: true,
                          lValueRequested: false,
                          nodeType: "ElementaryTypeNameExpression",
                          src: "1545:7:37",
                          typeDescriptions: {
                            typeIdentifier: "t_type$_t_address_$",
                            typeString: "type(address)",
                          },
                          typeName: {
                            id: 3783,
                            name: "address",
                            nodeType: "ElementaryTypeName",
                            src: "1545:7:37",
                            typeDescriptions: {},
                          },
                        },
                        id: 3786,
                        isConstant: false,
                        isLValue: false,
                        isPure: false,
                        kind: "typeConversion",
                        lValueRequested: false,
                        nameLocations: [],
                        names: [],
                        nodeType: "FunctionCall",
                        src: "1545:14:37",
                        tryCall: false,
                        typeDescriptions: {
                          typeIdentifier: "t_address",
                          typeString: "address",
                        },
                      },
                      id: 3787,
                      isConstant: false,
                      isLValue: false,
                      isPure: false,
                      lValueRequested: false,
                      memberLocation: "1560:10:37",
                      memberName: "staticcall",
                      nodeType: "MemberAccess",
                      src: "1545:25:37",
                      typeDescriptions: {
                        typeIdentifier:
                          "t_function_barestaticcall_view$_t_bytes_memory_ptr_$returns$_t_bool_$_t_bytes_memory_ptr_$",
                        typeString:
                          "function (bytes memory) view returns (bool,bytes memory)",
                      },
                    },
                    id: 3789,
                    isConstant: false,
                    isLValue: false,
                    isPure: false,
                    kind: "functionCall",
                    lValueRequested: false,
                    nameLocations: [],
                    names: [],
                    nodeType: "FunctionCall",
                    src: "1545:40:37",
                    tryCall: false,
                    typeDescriptions: {
                      typeIdentifier: "t_tuple$_t_bool_$_t_bytes_memory_ptr_$",
                      typeString: "tuple(bool,bytes memory)",
                    },
                  },
                  nodeType: "VariableDeclarationStatement",
                  src: "1503:82:37",
                },
                {
                  expression: {
                    arguments: [
                      {
                        id: 3792,
                        name: "success",
                        nodeType: "Identifier",
                        overloadedDeclarations: [],
                        referencedDeclaration: 3780,
                        src: "1603:7:37",
                        typeDescriptions: {
                          typeIdentifier: "t_bool",
                          typeString: "bool",
                        },
                      },
                    ],
                    expression: {
                      argumentTypes: [
                        { typeIdentifier: "t_bool", typeString: "bool" },
                      ],
                      id: 3791,
                      name: "require",
                      nodeType: "Identifier",
                      overloadedDeclarations: [-18, -18],
                      referencedDeclaration: -18,
                      src: "1595:7:37",
                      typeDescriptions: {
                        typeIdentifier:
                          "t_function_require_pure$_t_bool_$returns$__$",
                        typeString: "function (bool) pure",
                      },
                    },
                    id: 3793,
                    isConstant: false,
                    isLValue: false,
                    isPure: false,
                    kind: "functionCall",
                    lValueRequested: false,
                    nameLocations: [],
                    names: [],
                    nodeType: "FunctionCall",
                    src: "1595:16:37",
                    tryCall: false,
                    typeDescriptions: {
                      typeIdentifier: "t_tuple$__$",
                      typeString: "tuple()",
                    },
                  },
                  id: 3794,
                  nodeType: "ExpressionStatement",
                  src: "1595:16:37",
                },
                {
                  expression: {
                    arguments: [
                      {
                        id: 3797,
                        name: "returndata",
                        nodeType: "Identifier",
                        overloadedDeclarations: [],
                        referencedDeclaration: 3782,
                        src: "1639:10:37",
                        typeDescriptions: {
                          typeIdentifier: "t_bytes_memory_ptr",
                          typeString: "bytes memory",
                        },
                      },
                      {
                        components: [
                          {
                            id: 3799,
                            isConstant: false,
                            isLValue: false,
                            isPure: true,
                            lValueRequested: false,
                            nodeType: "ElementaryTypeNameExpression",
                            src: "1652:7:37",
                            typeDescriptions: {
                              typeIdentifier: "t_type$_t_address_$",
                              typeString: "type(address)",
                            },
                            typeName: {
                              id: 3798,
                              name: "address",
                              nodeType: "ElementaryTypeName",
                              src: "1652:7:37",
                              typeDescriptions: {},
                            },
                          },
                        ],
                        id: 3800,
                        isConstant: false,
                        isInlineArray: false,
                        isLValue: false,
                        isPure: true,
                        lValueRequested: false,
                        nodeType: "TupleExpression",
                        src: "1651:9:37",
                        typeDescriptions: {
                          typeIdentifier: "t_type$_t_address_$",
                          typeString: "type(address)",
                        },
                      },
                    ],
                    expression: {
                      argumentTypes: [
                        {
                          typeIdentifier: "t_bytes_memory_ptr",
                          typeString: "bytes memory",
                        },
                        {
                          typeIdentifier: "t_type$_t_address_$",
                          typeString: "type(address)",
                        },
                      ],
                      expression: {
                        id: 3795,
                        name: "abi",
                        nodeType: "Identifier",
                        overloadedDeclarations: [],
                        referencedDeclaration: -1,
                        src: "1628:3:37",
                        typeDescriptions: {
                          typeIdentifier: "t_magic_abi",
                          typeString: "abi",
                        },
                      },
                      id: 3796,
                      isConstant: false,
                      isLValue: false,
                      isPure: true,
                      lValueRequested: false,
                      memberLocation: "1632:6:37",
                      memberName: "decode",
                      nodeType: "MemberAccess",
                      src: "1628:10:37",
                      typeDescriptions: {
                        typeIdentifier:
                          "t_function_abidecode_pure$__$returns$__$",
                        typeString: "function () pure",
                      },
                    },
                    id: 3801,
                    isConstant: false,
                    isLValue: false,
                    isPure: false,
                    kind: "functionCall",
                    lValueRequested: false,
                    nameLocations: [],
                    names: [],
                    nodeType: "FunctionCall",
                    src: "1628:33:37",
                    tryCall: false,
                    typeDescriptions: {
                      typeIdentifier: "t_address_payable",
                      typeString: "address payable",
                    },
                  },
                  functionReturnParameters: 3778,
                  id: 3802,
                  nodeType: "Return",
                  src: "1621:40:37",
                },
              ],
            },
            documentation: {
              id: 3771,
              nodeType: "StructuredDocumentation",
              src: "1094:149:37",
              text: " @dev Returns the current admin of `proxy`.\n Requirements:\n - This contract must be the admin of `proxy`.",
            },
            functionSelector: "f3b7dead",
            implemented: true,
            kind: "function",
            modifiers: [],
            name: "getProxyAdmin",
            nameLocation: "1257:13:37",
            parameters: {
              id: 3775,
              nodeType: "ParameterList",
              parameters: [
                {
                  constant: false,
                  id: 3774,
                  mutability: "mutable",
                  name: "proxy",
                  nameLocation: "1300:5:37",
                  nodeType: "VariableDeclaration",
                  scope: 3804,
                  src: "1271:34:37",
                  stateVariable: false,
                  storageLocation: "default",
                  typeDescriptions: {
                    typeIdentifier:
                      "t_contract$_ITransparentUpgradeableProxy_$3899",
                    typeString: "contract ITransparentUpgradeableProxy",
                  },
                  typeName: {
                    id: 3773,
                    nodeType: "UserDefinedTypeName",
                    pathNode: {
                      id: 3772,
                      name: "ITransparentUpgradeableProxy",
                      nameLocations: ["1271:28:37"],
                      nodeType: "IdentifierPath",
                      referencedDeclaration: 3899,
                      src: "1271:28:37",
                    },
                    referencedDeclaration: 3899,
                    src: "1271:28:37",
                    typeDescriptions: {
                      typeIdentifier:
                        "t_contract$_ITransparentUpgradeableProxy_$3899",
                      typeString: "contract ITransparentUpgradeableProxy",
                    },
                  },
                  visibility: "internal",
                },
              ],
              src: "1270:36:37",
            },
            returnParameters: {
              id: 3778,
              nodeType: "ParameterList",
              parameters: [
                {
                  constant: false,
                  id: 3777,
                  mutability: "mutable",
                  name: "",
                  nameLocation: "-1:-1:-1",
                  nodeType: "VariableDeclaration",
                  scope: 3804,
                  src: "1336:7:37",
                  stateVariable: false,
                  storageLocation: "default",
                  typeDescriptions: {
                    typeIdentifier: "t_address",
                    typeString: "address",
                  },
                  typeName: {
                    id: 3776,
                    name: "address",
                    nodeType: "ElementaryTypeName",
                    src: "1336:7:37",
                    stateMutability: "nonpayable",
                    typeDescriptions: {
                      typeIdentifier: "t_address",
                      typeString: "address",
                    },
                  },
                  visibility: "internal",
                },
              ],
              src: "1335:9:37",
            },
            scope: 3865,
            stateMutability: "view",
            virtual: true,
            visibility: "public",
          },
          {
            id: 3822,
            nodeType: "FunctionDefinition",
            src: "1842:149:37",
            nodes: [],
            body: {
              id: 3821,
              nodeType: "Block",
              src: "1947:44:37",
              nodes: [],
              statements: [
                {
                  expression: {
                    arguments: [
                      {
                        id: 3818,
                        name: "newAdmin",
                        nodeType: "Identifier",
                        overloadedDeclarations: [],
                        referencedDeclaration: 3810,
                        src: "1975:8:37",
                        typeDescriptions: {
                          typeIdentifier: "t_address",
                          typeString: "address",
                        },
                      },
                    ],
                    expression: {
                      argumentTypes: [
                        { typeIdentifier: "t_address", typeString: "address" },
                      ],
                      expression: {
                        id: 3815,
                        name: "proxy",
                        nodeType: "Identifier",
                        overloadedDeclarations: [],
                        referencedDeclaration: 3808,
                        src: "1957:5:37",
                        typeDescriptions: {
                          typeIdentifier:
                            "t_contract$_ITransparentUpgradeableProxy_$3899",
                          typeString: "contract ITransparentUpgradeableProxy",
                        },
                      },
                      id: 3817,
                      isConstant: false,
                      isLValue: false,
                      isPure: false,
                      lValueRequested: false,
                      memberLocation: "1963:11:37",
                      memberName: "changeAdmin",
                      nodeType: "MemberAccess",
                      referencedDeclaration: 3886,
                      src: "1957:17:37",
                      typeDescriptions: {
                        typeIdentifier:
                          "t_function_external_nonpayable$_t_address_$returns$__$",
                        typeString: "function (address) external",
                      },
                    },
                    id: 3819,
                    isConstant: false,
                    isLValue: false,
                    isPure: false,
                    kind: "functionCall",
                    lValueRequested: false,
                    nameLocations: [],
                    names: [],
                    nodeType: "FunctionCall",
                    src: "1957:27:37",
                    tryCall: false,
                    typeDescriptions: {
                      typeIdentifier: "t_tuple$__$",
                      typeString: "tuple()",
                    },
                  },
                  id: 3820,
                  nodeType: "ExpressionStatement",
                  src: "1957:27:37",
                },
              ],
            },
            documentation: {
              id: 3805,
              nodeType: "StructuredDocumentation",
              src: "1674:163:37",
              text: " @dev Changes the admin of `proxy` to `newAdmin`.\n Requirements:\n - This contract must be the current admin of `proxy`.",
            },
            functionSelector: "7eff275e",
            implemented: true,
            kind: "function",
            modifiers: [
              {
                id: 3813,
                kind: "modifierInvocation",
                modifierName: {
                  id: 3812,
                  name: "onlyOwner",
                  nameLocations: ["1937:9:37"],
                  nodeType: "IdentifierPath",
                  referencedDeclaration: 2642,
                  src: "1937:9:37",
                },
                nodeType: "ModifierInvocation",
                src: "1937:9:37",
              },
            ],
            name: "changeProxyAdmin",
            nameLocation: "1851:16:37",
            parameters: {
              id: 3811,
              nodeType: "ParameterList",
              parameters: [
                {
                  constant: false,
                  id: 3808,
                  mutability: "mutable",
                  name: "proxy",
                  nameLocation: "1897:5:37",
                  nodeType: "VariableDeclaration",
                  scope: 3822,
                  src: "1868:34:37",
                  stateVariable: false,
                  storageLocation: "default",
                  typeDescriptions: {
                    typeIdentifier:
                      "t_contract$_ITransparentUpgradeableProxy_$3899",
                    typeString: "contract ITransparentUpgradeableProxy",
                  },
                  typeName: {
                    id: 3807,
                    nodeType: "UserDefinedTypeName",
                    pathNode: {
                      id: 3806,
                      name: "ITransparentUpgradeableProxy",
                      nameLocations: ["1868:28:37"],
                      nodeType: "IdentifierPath",
                      referencedDeclaration: 3899,
                      src: "1868:28:37",
                    },
                    referencedDeclaration: 3899,
                    src: "1868:28:37",
                    typeDescriptions: {
                      typeIdentifier:
                        "t_contract$_ITransparentUpgradeableProxy_$3899",
                      typeString: "contract ITransparentUpgradeableProxy",
                    },
                  },
                  visibility: "internal",
                },
                {
                  constant: false,
                  id: 3810,
                  mutability: "mutable",
                  name: "newAdmin",
                  nameLocation: "1912:8:37",
                  nodeType: "VariableDeclaration",
                  scope: 3822,
                  src: "1904:16:37",
                  stateVariable: false,
                  storageLocation: "default",
                  typeDescriptions: {
                    typeIdentifier: "t_address",
                    typeString: "address",
                  },
                  typeName: {
                    id: 3809,
                    name: "address",
                    nodeType: "ElementaryTypeName",
                    src: "1904:7:37",
                    stateMutability: "nonpayable",
                    typeDescriptions: {
                      typeIdentifier: "t_address",
                      typeString: "address",
                    },
                  },
                  visibility: "internal",
                },
              ],
              src: "1867:54:37",
            },
            returnParameters: {
              id: 3814,
              nodeType: "ParameterList",
              parameters: [],
              src: "1947:0:37",
            },
            scope: 3865,
            stateMutability: "nonpayable",
            virtual: true,
            visibility: "public",
          },
          {
            id: 3840,
            nodeType: "FunctionDefinition",
            src: "2196:150:37",
            nodes: [],
            body: {
              id: 3839,
              nodeType: "Block",
              src: "2298:48:37",
              nodes: [],
              statements: [
                {
                  expression: {
                    arguments: [
                      {
                        id: 3836,
                        name: "implementation",
                        nodeType: "Identifier",
                        overloadedDeclarations: [],
                        referencedDeclaration: 3828,
                        src: "2324:14:37",
                        typeDescriptions: {
                          typeIdentifier: "t_address",
                          typeString: "address",
                        },
                      },
                    ],
                    expression: {
                      argumentTypes: [
                        { typeIdentifier: "t_address", typeString: "address" },
                      ],
                      expression: {
                        id: 3833,
                        name: "proxy",
                        nodeType: "Identifier",
                        overloadedDeclarations: [],
                        referencedDeclaration: 3826,
                        src: "2308:5:37",
                        typeDescriptions: {
                          typeIdentifier:
                            "t_contract$_ITransparentUpgradeableProxy_$3899",
                          typeString: "contract ITransparentUpgradeableProxy",
                        },
                      },
                      id: 3835,
                      isConstant: false,
                      isLValue: false,
                      isPure: false,
                      lValueRequested: false,
                      memberLocation: "2314:9:37",
                      memberName: "upgradeTo",
                      nodeType: "MemberAccess",
                      referencedDeclaration: 3891,
                      src: "2308:15:37",
                      typeDescriptions: {
                        typeIdentifier:
                          "t_function_external_nonpayable$_t_address_$returns$__$",
                        typeString: "function (address) external",
                      },
                    },
                    id: 3837,
                    isConstant: false,
                    isLValue: false,
                    isPure: false,
                    kind: "functionCall",
                    lValueRequested: false,
                    nameLocations: [],
                    names: [],
                    nodeType: "FunctionCall",
                    src: "2308:31:37",
                    tryCall: false,
                    typeDescriptions: {
                      typeIdentifier: "t_tuple$__$",
                      typeString: "tuple()",
                    },
                  },
                  id: 3838,
                  nodeType: "ExpressionStatement",
                  src: "2308:31:37",
                },
              ],
            },
            documentation: {
              id: 3823,
              nodeType: "StructuredDocumentation",
              src: "1997:194:37",
              text: " @dev Upgrades `proxy` to `implementation`. See {TransparentUpgradeableProxy-upgradeTo}.\n Requirements:\n - This contract must be the admin of `proxy`.",
            },
            functionSelector: "99a88ec4",
            implemented: true,
            kind: "function",
            modifiers: [
              {
                id: 3831,
                kind: "modifierInvocation",
                modifierName: {
                  id: 3830,
                  name: "onlyOwner",
                  nameLocations: ["2288:9:37"],
                  nodeType: "IdentifierPath",
                  referencedDeclaration: 2642,
                  src: "2288:9:37",
                },
                nodeType: "ModifierInvocation",
                src: "2288:9:37",
              },
            ],
            name: "upgrade",
            nameLocation: "2205:7:37",
            parameters: {
              id: 3829,
              nodeType: "ParameterList",
              parameters: [
                {
                  constant: false,
                  id: 3826,
                  mutability: "mutable",
                  name: "proxy",
                  nameLocation: "2242:5:37",
                  nodeType: "VariableDeclaration",
                  scope: 3840,
                  src: "2213:34:37",
                  stateVariable: false,
                  storageLocation: "default",
                  typeDescriptions: {
                    typeIdentifier:
                      "t_contract$_ITransparentUpgradeableProxy_$3899",
                    typeString: "contract ITransparentUpgradeableProxy",
                  },
                  typeName: {
                    id: 3825,
                    nodeType: "UserDefinedTypeName",
                    pathNode: {
                      id: 3824,
                      name: "ITransparentUpgradeableProxy",
                      nameLocations: ["2213:28:37"],
                      nodeType: "IdentifierPath",
                      referencedDeclaration: 3899,
                      src: "2213:28:37",
                    },
                    referencedDeclaration: 3899,
                    src: "2213:28:37",
                    typeDescriptions: {
                      typeIdentifier:
                        "t_contract$_ITransparentUpgradeableProxy_$3899",
                      typeString: "contract ITransparentUpgradeableProxy",
                    },
                  },
                  visibility: "internal",
                },
                {
                  constant: false,
                  id: 3828,
                  mutability: "mutable",
                  name: "implementation",
                  nameLocation: "2257:14:37",
                  nodeType: "VariableDeclaration",
                  scope: 3840,
                  src: "2249:22:37",
                  stateVariable: false,
                  storageLocation: "default",
                  typeDescriptions: {
                    typeIdentifier: "t_address",
                    typeString: "address",
                  },
                  typeName: {
                    id: 3827,
                    name: "address",
                    nodeType: "ElementaryTypeName",
                    src: "2249:7:37",
                    stateMutability: "nonpayable",
                    typeDescriptions: {
                      typeIdentifier: "t_address",
                      typeString: "address",
                    },
                  },
                  visibility: "internal",
                },
              ],
              src: "2212:60:37",
            },
            returnParameters: {
              id: 3832,
              nodeType: "ParameterList",
              parameters: [],
              src: "2298:0:37",
            },
            scope: 3865,
            stateMutability: "nonpayable",
            virtual: true,
            visibility: "public",
          },
          {
            id: 3864,
            nodeType: "FunctionDefinition",
            src: "2612:245:37",
            nodes: [],
            body: {
              id: 3863,
              nodeType: "Block",
              src: "2778:79:37",
              nodes: [],
              statements: [
                {
                  expression: {
                    arguments: [
                      {
                        id: 3859,
                        name: "implementation",
                        nodeType: "Identifier",
                        overloadedDeclarations: [],
                        referencedDeclaration: 3846,
                        src: "2829:14:37",
                        typeDescriptions: {
                          typeIdentifier: "t_address",
                          typeString: "address",
                        },
                      },
                      {
                        id: 3860,
                        name: "data",
                        nodeType: "Identifier",
                        overloadedDeclarations: [],
                        referencedDeclaration: 3848,
                        src: "2845:4:37",
                        typeDescriptions: {
                          typeIdentifier: "t_bytes_memory_ptr",
                          typeString: "bytes memory",
                        },
                      },
                    ],
                    expression: {
                      argumentTypes: [
                        { typeIdentifier: "t_address", typeString: "address" },
                        {
                          typeIdentifier: "t_bytes_memory_ptr",
                          typeString: "bytes memory",
                        },
                      ],
                      expression: {
                        argumentTypes: [
                          {
                            typeIdentifier: "t_address",
                            typeString: "address",
                          },
                          {
                            typeIdentifier: "t_bytes_memory_ptr",
                            typeString: "bytes memory",
                          },
                        ],
                        expression: {
                          id: 3853,
                          name: "proxy",
                          nodeType: "Identifier",
                          overloadedDeclarations: [],
                          referencedDeclaration: 3844,
                          src: "2788:5:37",
                          typeDescriptions: {
                            typeIdentifier:
                              "t_contract$_ITransparentUpgradeableProxy_$3899",
                            typeString: "contract ITransparentUpgradeableProxy",
                          },
                        },
                        id: 3855,
                        isConstant: false,
                        isLValue: false,
                        isPure: false,
                        lValueRequested: false,
                        memberLocation: "2794:16:37",
                        memberName: "upgradeToAndCall",
                        nodeType: "MemberAccess",
                        referencedDeclaration: 3898,
                        src: "2788:22:37",
                        typeDescriptions: {
                          typeIdentifier:
                            "t_function_external_payable$_t_address_$_t_bytes_memory_ptr_$returns$__$",
                          typeString:
                            "function (address,bytes memory) payable external",
                        },
                      },
                      id: 3858,
                      isConstant: false,
                      isLValue: false,
                      isPure: false,
                      lValueRequested: false,
                      names: ["value"],
                      nodeType: "FunctionCallOptions",
                      options: [
                        {
                          expression: {
                            id: 3856,
                            name: "msg",
                            nodeType: "Identifier",
                            overloadedDeclarations: [],
                            referencedDeclaration: -15,
                            src: "2818:3:37",
                            typeDescriptions: {
                              typeIdentifier: "t_magic_message",
                              typeString: "msg",
                            },
                          },
                          id: 3857,
                          isConstant: false,
                          isLValue: false,
                          isPure: false,
                          lValueRequested: false,
                          memberLocation: "2822:5:37",
                          memberName: "value",
                          nodeType: "MemberAccess",
                          src: "2818:9:37",
                          typeDescriptions: {
                            typeIdentifier: "t_uint256",
                            typeString: "uint256",
                          },
                        },
                      ],
                      src: "2788:40:37",
                      typeDescriptions: {
                        typeIdentifier:
                          "t_function_external_payable$_t_address_$_t_bytes_memory_ptr_$returns$__$value",
                        typeString:
                          "function (address,bytes memory) payable external",
                      },
                    },
                    id: 3861,
                    isConstant: false,
                    isLValue: false,
                    isPure: false,
                    kind: "functionCall",
                    lValueRequested: false,
                    nameLocations: [],
                    names: [],
                    nodeType: "FunctionCall",
                    src: "2788:62:37",
                    tryCall: false,
                    typeDescriptions: {
                      typeIdentifier: "t_tuple$__$",
                      typeString: "tuple()",
                    },
                  },
                  id: 3862,
                  nodeType: "ExpressionStatement",
                  src: "2788:62:37",
                },
              ],
            },
            documentation: {
              id: 3841,
              nodeType: "StructuredDocumentation",
              src: "2352:255:37",
              text: " @dev Upgrades `proxy` to `implementation` and calls a function on the new implementation. See\n {TransparentUpgradeableProxy-upgradeToAndCall}.\n Requirements:\n - This contract must be the admin of `proxy`.",
            },
            functionSelector: "9623609d",
            implemented: true,
            kind: "function",
            modifiers: [
              {
                id: 3851,
                kind: "modifierInvocation",
                modifierName: {
                  id: 3850,
                  name: "onlyOwner",
                  nameLocations: ["2768:9:37"],
                  nodeType: "IdentifierPath",
                  referencedDeclaration: 2642,
                  src: "2768:9:37",
                },
                nodeType: "ModifierInvocation",
                src: "2768:9:37",
              },
            ],
            name: "upgradeAndCall",
            nameLocation: "2621:14:37",
            parameters: {
              id: 3849,
              nodeType: "ParameterList",
              parameters: [
                {
                  constant: false,
                  id: 3844,
                  mutability: "mutable",
                  name: "proxy",
                  nameLocation: "2674:5:37",
                  nodeType: "VariableDeclaration",
                  scope: 3864,
                  src: "2645:34:37",
                  stateVariable: false,
                  storageLocation: "default",
                  typeDescriptions: {
                    typeIdentifier:
                      "t_contract$_ITransparentUpgradeableProxy_$3899",
                    typeString: "contract ITransparentUpgradeableProxy",
                  },
                  typeName: {
                    id: 3843,
                    nodeType: "UserDefinedTypeName",
                    pathNode: {
                      id: 3842,
                      name: "ITransparentUpgradeableProxy",
                      nameLocations: ["2645:28:37"],
                      nodeType: "IdentifierPath",
                      referencedDeclaration: 3899,
                      src: "2645:28:37",
                    },
                    referencedDeclaration: 3899,
                    src: "2645:28:37",
                    typeDescriptions: {
                      typeIdentifier:
                        "t_contract$_ITransparentUpgradeableProxy_$3899",
                      typeString: "contract ITransparentUpgradeableProxy",
                    },
                  },
                  visibility: "internal",
                },
                {
                  constant: false,
                  id: 3846,
                  mutability: "mutable",
                  name: "implementation",
                  nameLocation: "2697:14:37",
                  nodeType: "VariableDeclaration",
                  scope: 3864,
                  src: "2689:22:37",
                  stateVariable: false,
                  storageLocation: "default",
                  typeDescriptions: {
                    typeIdentifier: "t_address",
                    typeString: "address",
                  },
                  typeName: {
                    id: 3845,
                    name: "address",
                    nodeType: "ElementaryTypeName",
                    src: "2689:7:37",
                    stateMutability: "nonpayable",
                    typeDescriptions: {
                      typeIdentifier: "t_address",
                      typeString: "address",
                    },
                  },
                  visibility: "internal",
                },
                {
                  constant: false,
                  id: 3848,
                  mutability: "mutable",
                  name: "data",
                  nameLocation: "2734:4:37",
                  nodeType: "VariableDeclaration",
                  scope: 3864,
                  src: "2721:17:37",
                  stateVariable: false,
                  storageLocation: "memory",
                  typeDescriptions: {
                    typeIdentifier: "t_bytes_memory_ptr",
                    typeString: "bytes",
                  },
                  typeName: {
                    id: 3847,
                    name: "bytes",
                    nodeType: "ElementaryTypeName",
                    src: "2721:5:37",
                    typeDescriptions: {
                      typeIdentifier: "t_bytes_storage_ptr",
                      typeString: "bytes",
                    },
                  },
                  visibility: "internal",
                },
              ],
              src: "2635:109:37",
            },
            returnParameters: {
              id: 3852,
              nodeType: "ParameterList",
              parameters: [],
              src: "2778:0:37",
            },
            scope: 3865,
            stateMutability: "payable",
            virtual: true,
            visibility: "public",
          },
        ],
        abstract: false,
        baseContracts: [
          {
            baseName: {
              id: 3735,
              name: "Ownable",
              nameLocations: ["473:7:37"],
              nodeType: "IdentifierPath",
              referencedDeclaration: 2723,
              src: "473:7:37",
            },
            id: 3736,
            nodeType: "InheritanceSpecifier",
            src: "473:7:37",
          },
        ],
        canonicalName: "ProxyAdmin",
        contractDependencies: [],
        contractKind: "contract",
        documentation: {
          id: 3734,
          nodeType: "StructuredDocumentation",
          src: "221:228:37",
          text: " @dev This is an auxiliary contract meant to be assigned as the admin of a {TransparentUpgradeableProxy}. For an\n explanation of why you would want to use this see the documentation for {TransparentUpgradeableProxy}.",
        },
        fullyImplemented: true,
        linearizedBaseContracts: [3865, 2723, 80709],
        name: "ProxyAdmin",
        nameLocation: "459:10:37",
        scope: 3866,
        usedErrors: [],
      },
    ],
    license: "MIT",
  },
  id: 37,
} as const;
