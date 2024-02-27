export const StaticMessageIdMultisigIsmFactory = {
  abi: [
    {
      type: "function",
      name: "deploy",
      inputs: [
        { name: "_values", type: "address[]", internalType: "address[]" },
        { name: "_threshold", type: "uint8", internalType: "uint8" },
      ],
      outputs: [{ name: "", type: "address", internalType: "address" }],
      stateMutability: "nonpayable",
    },
    {
      type: "function",
      name: "getAddress",
      inputs: [
        { name: "_values", type: "address[]", internalType: "address[]" },
        { name: "_threshold", type: "uint8", internalType: "uint8" },
      ],
      outputs: [{ name: "", type: "address", internalType: "address" }],
      stateMutability: "view",
    },
    {
      type: "function",
      name: "implementation",
      inputs: [],
      outputs: [{ name: "", type: "address", internalType: "address" }],
      stateMutability: "view",
    },
  ],
  bytecode: {
    object:
      "0x60a060405234801561001057600080fd5b5061001961002a565b6001600160a01b0316608052610067565b60006040516100389061005a565b604051809103906000f080158015610054573d6000803e3d6000fd5b50905090565b610d328061061a83390190565b60805161059261008860003960008181604b015261016e01526105926000f3fe608060405234801561001057600080fd5b50600436106100415760003560e01c80635c60da1b14610046578063d4277ebc14610096578063ed547bf7146100a9575b600080fd5b61006d7f000000000000000000000000000000000000000000000000000000000000000081565b60405173ffffffffffffffffffffffffffffffffffffffff909116815260200160405180910390f35b61006d6100a43660046103d3565b6100bc565b61006d6100b73660046103d3565b6100e6565b60008060006100cc86868661013b565b915091506100da82826101ab565b925050505b9392505050565b60008060006100f686868661013b565b91509150600061010683836101ab565b905073ffffffffffffffffffffffffffffffffffffffff81163b6100da57610130600084846101c7565b979650505050505050565b6000606060008585856040516020016101569392919061045f565b604051602081830303815290604052905060006101937f00000000000000000000000000000000000000000000000000000000000000008361032b565b82516020909301929092209791965090945050505050565b805160208201206000906101bf84826103a1565b949350505050565b600083471015610238576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601d60248201527f437265617465323a20696e73756666696369656e742062616c616e636500000060448201526064015b60405180910390fd5b81516000036102a3576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820181905260248201527f437265617465323a2062797465636f6465206c656e677468206973207a65726f604482015260640161022f565b8282516020840186f5905073ffffffffffffffffffffffffffffffffffffffff81166100df576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601960248201527f437265617465323a204661696c6564206f6e206465706c6f7900000000000000604482015260640161022f565b60607f600b380380600b3d393df3363d3d373d3d3d3d60368038038091363936013d738360601b7f5af43d3d93803e603457fd5bf30000000000000000000000000000000000000084855160405160200161038a9594939291906104cc565b604051602081830303815290604052905092915050565b60006100df8383306000604051836040820152846020820152828152600b8101905060ff815360559020949350505050565b6000806000604084860312156103e857600080fd5b833567ffffffffffffffff8082111561040057600080fd5b818601915086601f83011261041457600080fd5b81358181111561042357600080fd5b8760208260051b850101111561043857600080fd5b6020928301955093505084013560ff8116811461045457600080fd5b809150509250925092565b604080825281018390526000846060830182805b878110156104b457833573ffffffffffffffffffffffffffffffffffffffff811680821461049f578384fd5b84525060209384019390920191600101610473565b5050809250505060ff83166020830152949350505050565b858152600060207fffffffffffffffffffffffffffffffffffffffff0000000000000000000000008716818401527fffffffffffffffffffffffffff0000000000000000000000000000000000000086166034840152845160005b8181101561054357868101830151858201604101528201610527565b506041930192830193909352506061019594505050505056fea26469706673582212206835b6900a7c99306533ee5fb7261b0c3341b0e1236fcfbffa5372fe01e0024464736f6c63430008130033608060405234801561001057600080fd5b50610d12806100206000396000f3fe608060405234801561001057600080fd5b50600436106100415760003560e01c80632e0ed234146100465780636465e69f14610070578063f7e83aee1461008a575b600080fd5b6100596100543660046108b6565b6100ad565b6040516100679291906108f8565b60405180910390f35b610078600581565b60405160ff9091168152602001610067565b61009d61009836600461095c565b6100d8565b6040519015158152602001610067565b606060006100b96102f1565b8060200190518101906100cc9190610a31565b915091505b9250929050565b6000806100e786868686610321565b90506000806100f686866100ad565b9150915060008160ff1611610192576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602c60248201527f4e6f204d756c746973696749534d207468726573686f6c642070726573656e7460448201527f20666f72206d657373616765000000000000000000000000000000000000000060648201526084015b60405180910390fd5b81516000805b8360ff168110156102e05760006101ee876101b48e8e86610479565b8080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525061049392505050565b90505b8383108015610245575085838151811061020d5761020d610b55565b602002602001015173ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1614155b1561025a5761025383610bb3565b92506101f1565b8383106102c3576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152600a60248201527f217468726573686f6c64000000000000000000000000000000000000000000006044820152606401610189565b6102cc83610bb3565b925050806102d990610bb3565b9050610198565b5060019a9950505050505050505050565b60608060203603803580820391506040519250602081018301604052808352602083018183823750919392505050565b600061047061033084846104b9565b61033a87876104dc565b610344888861050c565b61034e898961051c565b61038d88888080601f01602080910402602001604051908101604052809392919081815260200183838082843760009201919091525061053d92505050565b604080517fffffffff0000000000000000000000000000000000000000000000000000000060e097881b811660208084019190915260248301979097527f48595045524c414e45000000000000000000000000000000000000000000000060448301528251808303602d018152604d83018452805190880120606d830152608d8201959095529290951b90921660ad82015260b1808201929092528351808203909201825260d10190925281519101207f19457468657265756d205369676e6564204d6573736167653a0a3332000000006000908152601c91909152603c902090565b95945050505050565b366000610487858585610548565b91509150935093915050565b60008060006104a2858561058d565b915091506104af816105cf565b5090505b92915050565b60006104c9600960058486610beb565b6104d291610c15565b60e01c9392505050565b60008281836104ec826020610c5d565b60ff16926104fc93929190610beb565b61050591610c76565b9392505050565b6000826020836104ec8280610c5d565b60008260408361052d826004610c5d565b60ff16926104c993929190610beb565b805160209091012090565b36600080610557604185610cb2565b610562906044610cc9565b90506000610571604183610cc9565b905061057f8183888a610beb565b935093505050935093915050565b60008082516041036105c35760208301516040840151606085015160001a6105b787828585610785565b945094505050506100d1565b506000905060026100d1565b60008160048111156105e3576105e3610b26565b036105eb5750565b60018160048111156105ff576105ff610b26565b03610666576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601860248201527f45434453413a20696e76616c6964207369676e617475726500000000000000006044820152606401610189565b600281600481111561067a5761067a610b26565b036106e1576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601f60248201527f45434453413a20696e76616c6964207369676e6174757265206c656e677468006044820152606401610189565b60038160048111156106f5576106f5610b26565b03610782576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152602260248201527f45434453413a20696e76616c6964207369676e6174757265202773272076616c60448201527f75650000000000000000000000000000000000000000000000000000000000006064820152608401610189565b50565b6000807f7fffffffffffffffffffffffffffffff5d576e7357a4501ddfe92f46681b20a08311156107bc575060009050600361086b565b6040805160008082526020820180845289905260ff881692820192909252606081018690526080810185905260019060a0016020604051602081039080840390855afa158015610810573d6000803e3d6000fd5b50506040517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0015191505073ffffffffffffffffffffffffffffffffffffffff81166108645760006001925092505061086b565b9150600090505b94509492505050565b60008083601f84011261088657600080fd5b50813567ffffffffffffffff81111561089e57600080fd5b6020830191508360208285010111156100d157600080fd5b600080602083850312156108c957600080fd5b823567ffffffffffffffff8111156108e057600080fd5b6108ec85828601610874565b90969095509350505050565b604080825283519082018190526000906020906060840190828701845b8281101561094757815173ffffffffffffffffffffffffffffffffffffffff1684529284019290840190600101610915565b50505060ff9490941692019190915250919050565b6000806000806040858703121561097257600080fd5b843567ffffffffffffffff8082111561098a57600080fd5b61099688838901610874565b909650945060208701359150808211156109af57600080fd5b506109bc87828801610874565b95989497509550505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052604160045260246000fd5b805173ffffffffffffffffffffffffffffffffffffffff81168114610a1b57600080fd5b919050565b805160ff81168114610a1b57600080fd5b60008060408385031215610a4457600080fd5b825167ffffffffffffffff80821115610a5c57600080fd5b818501915085601f830112610a7057600080fd5b8151602082821115610a8457610a846109c8565b8160051b6040517fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffe0603f83011681018181108682111715610ac757610ac76109c8565b604052928352818301935084810182019289841115610ae557600080fd5b948201945b83861015610b0a57610afb866109f7565b85529482019493820193610aea565b9650610b199050878201610a20565b9450505050509250929050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052602160045260246000fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052603260045260246000fd5b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b60007fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff8203610be457610be4610b84565b5060010190565b60008085851115610bfb57600080fd5b83861115610c0857600080fd5b5050820193919092039150565b7fffffffff000000000000000000000000000000000000000000000000000000008135818116916004851015610c555780818660040360031b1b83161692505b505092915050565b60ff81811683821601908111156104b3576104b3610b84565b803560208310156104b3577fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff602084900360031b1b1692915050565b80820281158282048414176104b3576104b3610b84565b808201808211156104b3576104b3610b8456fea2646970667358221220f1388193c8f63060046d5de774d3e89a05ec036e2df567e67195da1226685bb664736f6c63430008130033",
    sourceMap:
      "1874:221:165:-:0;;;;;;;;;;;;-1:-1:-1;576:23:180;:21;:23::i;:::-;-1:-1:-1;;;;;559:40:180;;;1874:221:165;;1959:134;2019:7;2053:32;;;;;:::i;:::-;;;;;;;;;;;;;;;;;;;;;;;;2038:48;;1959:134;:::o;1874:221::-;;;;;;;;:::o;:::-;;;;;;;;;;;;;;;;;;",
    linkReferences: {},
  },
  deployedBytecode: {
    object:
      "0x608060405234801561001057600080fd5b50600436106100415760003560e01c80635c60da1b14610046578063d4277ebc14610096578063ed547bf7146100a9575b600080fd5b61006d7f000000000000000000000000000000000000000000000000000000000000000081565b60405173ffffffffffffffffffffffffffffffffffffffff909116815260200160405180910390f35b61006d6100a43660046103d3565b6100bc565b61006d6100b73660046103d3565b6100e6565b60008060006100cc86868661013b565b915091506100da82826101ab565b925050505b9392505050565b60008060006100f686868661013b565b91509150600061010683836101ab565b905073ffffffffffffffffffffffffffffffffffffffff81163b6100da57610130600084846101c7565b979650505050505050565b6000606060008585856040516020016101569392919061045f565b604051602081830303815290604052905060006101937f00000000000000000000000000000000000000000000000000000000000000008361032b565b82516020909301929092209791965090945050505050565b805160208201206000906101bf84826103a1565b949350505050565b600083471015610238576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601d60248201527f437265617465323a20696e73756666696369656e742062616c616e636500000060448201526064015b60405180910390fd5b81516000036102a3576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820181905260248201527f437265617465323a2062797465636f6465206c656e677468206973207a65726f604482015260640161022f565b8282516020840186f5905073ffffffffffffffffffffffffffffffffffffffff81166100df576040517f08c379a000000000000000000000000000000000000000000000000000000000815260206004820152601960248201527f437265617465323a204661696c6564206f6e206465706c6f7900000000000000604482015260640161022f565b60607f600b380380600b3d393df3363d3d373d3d3d3d60368038038091363936013d738360601b7f5af43d3d93803e603457fd5bf30000000000000000000000000000000000000084855160405160200161038a9594939291906104cc565b604051602081830303815290604052905092915050565b60006100df8383306000604051836040820152846020820152828152600b8101905060ff815360559020949350505050565b6000806000604084860312156103e857600080fd5b833567ffffffffffffffff8082111561040057600080fd5b818601915086601f83011261041457600080fd5b81358181111561042357600080fd5b8760208260051b850101111561043857600080fd5b6020928301955093505084013560ff8116811461045457600080fd5b809150509250925092565b604080825281018390526000846060830182805b878110156104b457833573ffffffffffffffffffffffffffffffffffffffff811680821461049f578384fd5b84525060209384019390920191600101610473565b5050809250505060ff83166020830152949350505050565b858152600060207fffffffffffffffffffffffffffffffffffffffff0000000000000000000000008716818401527fffffffffffffffffffffffffff0000000000000000000000000000000000000086166034840152845160005b8181101561054357868101830151858201604101528201610527565b506041930192830193909352506061019594505050505056fea26469706673582212206835b6900a7c99306533ee5fb7261b0c3341b0e1236fcfbffa5372fe01e0024464736f6c63430008130033",
    sourceMap:
      "1874:221:165:-:0;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;;443:39:180;;;;;;;;190:42:282;178:55;;;160:74;;148:2;133:18;443:39:180;;;;;;;1827:296;;;;;;:::i;:::-;;:::i;1042:421::-;;;;;;:::i;:::-;;:::i;1827:296::-;1938:7;1958:13;1973:22;1999:71;2029:7;;2050:10;1999:16;:71::i;:::-;1957:113;;;;2087:29;2099:5;2106:9;2087:11;:29::i;:::-;2080:36;;;;1827:296;;;;;;:::o;1042:421::-;1142:7;1162:13;1177:22;1203:71;1233:7;;1254:10;1203:16;:71::i;:::-;1161:113;;;;1284:12;1299:29;1311:5;1318:9;1299:11;:29::i;:::-;1284:44;-1:-1:-1;1702:19:52;;;;1338:98:180;;1390:35;1405:1;1408:5;1415:9;1390:14;:35::i;:::-;1383:42;1452:4;-1:-1:-1;;;;;;;1042:421:180:o;2973:374::-;3090:7;3099:12;3123:22;3159:7;;3168:10;3148:31;;;;;;;;;;:::i;:::-;;;;;;;;;;;;;3123:56;;3189:22;3214:45;3233:14;3249:9;3214:18;:45::i;:::-;3285:20;;;;;;;;;;;;;-1:-1:-1;2973:374:180;;-1:-1:-1;;;;;2973:374:180:o;2435:236::-;2583:20;;;;;;2540:7;;2620:44;2643:5;2583:20;2620:22;:44::i;:::-;2613:51;2435:236;-1:-1:-1;;;;2435:236:180:o;1081:484:54:-;1168:12;1225:6;1200:21;:31;;1192:73;;;;;;;2111:2:282;1192:73:54;;;2093:21:282;2150:2;2130:18;;;2123:30;2189:31;2169:18;;;2162:59;2238:18;;1192:73:54;;;;;;;;;1283:8;:15;1302:1;1283:20;1275:65;;;;;;;2469:2:282;1275:65:54;;;2451:21:282;;;2488:18;;;2481:30;2547:34;2527:18;;;2520:62;2599:18;;1275:65:54;2267:356:282;1275:65:54;1478:4;1467:8;1461:15;1454:4;1444:8;1440:19;1432:6;1424:59;1416:67;-1:-1:-1;1510:18:54;;;1502:56;;;;;;;2830:2:282;1502:56:54;;;2812:21:282;2869:2;2849:18;;;2842:30;2908:27;2888:18;;;2881:55;2953:18;;1502:56:54;2628:349:282;337:343:178;449:12;526:6;558:15;550:24;;592:6;616:9;643;:16;492:181;;;;;;;;;;;;:::i;:::-;;;;;;;;;;;;;473:200;;337:343;;;;:::o;1769:165:54:-;1852:7;1878:49;1893:4;1899:12;1921:4;2278:12;2385:4;2379:11;3604:12;3597:4;3592:3;3588:14;3581:36;3653:4;3646;3641:3;3637:14;3630:28;3683:8;3678:3;3671:21;3776:4;3771:3;3767:14;3754:27;;3887:4;3880:5;3872:20;3930:2;3913:20;;;2177:1772;-1:-1:-1;;;;2177:1772:54:o;245:778:282:-;338:6;346;354;407:2;395:9;386:7;382:23;378:32;375:52;;;423:1;420;413:12;375:52;463:9;450:23;492:18;533:2;525:6;522:14;519:34;;;549:1;546;539:12;519:34;587:6;576:9;572:22;562:32;;632:7;625:4;621:2;617:13;613:27;603:55;;654:1;651;644:12;603:55;694:2;681:16;720:2;712:6;709:14;706:34;;;736:1;733;726:12;706:34;791:7;784:4;774:6;771:1;767:14;763:2;759:23;755:34;752:47;749:67;;;812:1;809;802:12;749:67;843:4;835:13;;;;-1:-1:-1;867:6:282;-1:-1:-1;;908:20:282;;895:34;969:4;958:16;;948:27;;938:55;;989:1;986;979:12;938:55;1012:5;1002:15;;;245:778;;;;;:::o;1028:876::-;1252:2;1264:21;;;1237:18;;1320:22;;;1204:4;1399:6;1373:2;1358:18;;1204:4;;1454:368;1470:6;1465:3;1462:15;1454:368;;;1549:6;1536:20;1590:42;1583:5;1579:54;1666:2;1659:5;1656:13;1646:41;;1683:1;1680;1673:12;1646:41;1700:15;;-1:-1:-1;1738:4:282;1797:15;;;;1762:12;;;;1496:1;1487:11;1454:368;;;1458:3;;1839;1831:11;;;;1892:4;1884:6;1880:17;1873:4;1862:9;1858:20;1851:47;1028:876;;;;;;:::o;2982:821::-;3253:6;3248:3;3241:19;3223:3;3279:2;3323:66;3315:6;3311:79;3306:2;3301:3;3297:12;3290:101;3433:66;3425:6;3421:79;3416:2;3411:3;3407:12;3400:101;3530:6;3524:13;3555:1;3565:134;3579:6;3576:1;3573:13;3565:134;;;3668:14;;;3664:23;;3658:30;3640:11;;;3653:2;3636:20;3629:60;3594:10;;3565:134;;;-1:-1:-1;3758:2:282;3718:16;;3750:11;;;3743:27;;;;-1:-1:-1;3794:2:282;3786:11;;2982:821;-1:-1:-1;;;;;2982:821:282:o",
    linkReferences: {},
    immutableReferences: {
      "24705": [
        { start: 75, length: 32 },
        { start: 366, length: 32 },
      ],
    },
  },
  methodIdentifiers: {
    "deploy(address[],uint8)": "ed547bf7",
    "getAddress(address[],uint8)": "d4277ebc",
    "implementation()": "5c60da1b",
  },
  rawMetadata:
    '{"compiler":{"version":"0.8.19+commit.7dd6d404"},"language":"Solidity","output":{"abi":[{"inputs":[{"internalType":"address[]","name":"_values","type":"address[]"},{"internalType":"uint8","name":"_threshold","type":"uint8"}],"name":"deploy","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"nonpayable","type":"function"},{"inputs":[{"internalType":"address[]","name":"_values","type":"address[]"},{"internalType":"uint8","name":"_threshold","type":"uint8"}],"name":"getAddress","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"},{"inputs":[],"name":"implementation","outputs":[{"internalType":"address","name":"","type":"address"}],"stateMutability":"view","type":"function"}],"devdoc":{"kind":"dev","methods":{"deploy(address[],uint8)":{"details":"Consider sorting addresses to ensure contract reuse","params":{"_threshold":"The threshold value to use","_values":"An array of addresses"},"returns":{"_0":"set The contract address representing this StaticThresholdAddressSet"}},"getAddress(address[],uint8)":{"details":"Consider sorting addresses to ensure contract reuse","params":{"_threshold":"The threshold value to use","_values":"An array of addresses"},"returns":{"_0":"set The contract address representing this StaticThresholdAddressSet"}}},"version":1},"userdoc":{"kind":"user","methods":{"deploy(address[],uint8)":{"notice":"Deploys a StaticThresholdAddressSet contract address for the given values"},"getAddress(address[],uint8)":{"notice":"Returns the StaticThresholdAddressSet contract address for the given values"}},"version":1}},"settings":{"compilationTarget":{"contracts/isms/multisig/StaticMultisigIsm.sol":"StaticMessageIdMultisigIsmFactory"},"evmVersion":"paris","libraries":{},"metadata":{"bytecodeHash":"ipfs"},"optimizer":{"enabled":true,"runs":999999},"remappings":[":@eth-optimism/=../node_modules/@eth-optimism/",":@openzeppelin/=../node_modules/@openzeppelin/",":ds-test/=lib/forge-std/lib/ds-test/src/",":forge-std/=lib/forge-std/src/"]},"sources":{"../node_modules/@openzeppelin/contracts/utils/Address.sol":{"keccak256":"0x006dd67219697fe68d7fbfdea512e7c4cb64a43565ed86171d67e844982da6fa","license":"MIT","urls":["bzz-raw://2455248c8ddd9cc6a7af76a13973cddf222072427e7b0e2a7d1aff345145e931","dweb:/ipfs/QmfYjnjRbWqYpuxurqveE6HtzsY1Xx323J428AKQgtBJZm"]},"../node_modules/@openzeppelin/contracts/utils/Create2.sol":{"keccak256":"0x6e00f269073ffc4350e56b7e8153c9092d5f70bfba423299990514183101ef89","license":"MIT","urls":["bzz-raw://8dc5ff5c204604be7594ce2fa9100a0638f22b5aada6ebec1e165b0f50a48447","dweb:/ipfs/QmfH7uWphPrq8jqwGvyh4pFnLEgEPmLwyGV2Bvx1Acjggh"]},"../node_modules/@openzeppelin/contracts/utils/Strings.sol":{"keccak256":"0x3088eb2868e8d13d89d16670b5f8612c4ab9ff8956272837d8e90106c59c14a0","license":"MIT","urls":["bzz-raw://b81d9ff6559ea5c47fc573e17ece6d9ba5d6839e213e6ebc3b4c5c8fe4199d7f","dweb:/ipfs/QmPCW1bFisUzJkyjroY3yipwfism9RRCigCcK1hbXtVM8n"]},"../node_modules/@openzeppelin/contracts/utils/cryptography/ECDSA.sol":{"keccak256":"0x809bc3edb4bcbef8263fa616c1b60ee0004b50a8a1bfa164d8f57fd31f520c58","license":"MIT","urls":["bzz-raw://8b93a1e39a4a19eba1600b92c96f435442db88cac91e315c8291547a2a7bcfe2","dweb:/ipfs/QmTm34KVe6uZBZwq8dZDNWwPcm24qBJdxqL3rPxBJ4LrMv"]},"../node_modules/@openzeppelin/contracts/utils/math/Math.sol":{"keccak256":"0xe4455ac1eb7fc497bb7402579e7b4d64d928b846fce7d2b6fde06d366f21c2b3","license":"MIT","urls":["bzz-raw://cc8841b3cd48ad125e2f46323c8bad3aa0e88e399ec62acb9e57efa7e7c8058c","dweb:/ipfs/QmSqE4mXHA2BXW58deDbXE8MTcsL5JSKNDbm23sVQxRLPS"]},"../node_modules/@openzeppelin/contracts/utils/math/SignedMath.sol":{"keccak256":"0xf92515413956f529d95977adc9b0567d583c6203fc31ab1c23824c35187e3ddc","license":"MIT","urls":["bzz-raw://c50fcc459e49a9858b6d8ad5f911295cb7c9ab57567845a250bf0153f84a95c7","dweb:/ipfs/QmcEW85JRzvDkQggxiBBLVAasXWdkhEysqypj9EaB6H2g6"]},"contracts/interfaces/IInterchainSecurityModule.sol":{"keccak256":"0xd144a00a15eef18012f8dae28ca1a7d965012f88ff4a07382a172ae6e02811ce","license":"MIT OR Apache-2.0","urls":["bzz-raw://045da4f74529b4c0252743fe3f84e96d7e6e41cf30632011eefbb5b33eaa4cb2","dweb:/ipfs/QmefWSKyXSndAGC38tszRfdXbo4kvSsgcq7oTZz9rQ9n2p"]},"contracts/interfaces/isms/IMultisigIsm.sol":{"keccak256":"0x4815639fb16bc00a611ae978bbff47810dfd76b2b38b4613d7d893a7c35c4d34","license":"MIT OR Apache-2.0","urls":["bzz-raw://c386374a9e1154b027f8e921de9b333f8b6b7f6724e01ccb1d3c3fbca8aca7b4","dweb:/ipfs/QmQjPQFL1DSmqbyFFP49GgfEezzQBjz3c8SbThVADZi1Ui"]},"contracts/isms/libs/MerkleRootMultisigIsmMetadata.sol":{"keccak256":"0xb45298ff24fd4f12f2aeb93f6ab700749ef9952238c7693922bc8e6cca457395","license":"MIT OR Apache-2.0","urls":["bzz-raw://50a248b5d89c078b4d80a282c19292d5ef68770949ff9ec4c76275e6dbba8764","dweb:/ipfs/QmW1PCYZtaQBocy5yz3VPZK1YTSnwH9UWYnzcRVHn5rLR6"]},"contracts/isms/libs/MessageIdMultisigIsmMetadata.sol":{"keccak256":"0x6a5cd7301d1e1faa6c30a988c9f5235eb9b9c6e334b5bcc06c153df79687afb6","license":"MIT OR Apache-2.0","urls":["bzz-raw://af34bcfc9d7043ef1d83b8f79da13f9cefbf634cbb423ea0fc67b431e20a84fb","dweb:/ipfs/QmVoae2thC5eALT6d9h1Jtp6X9WgjvrPT2c4Cw2iXVkRmp"]},"contracts/isms/multisig/AbstractMerkleRootMultisigIsm.sol":{"keccak256":"0xa3ada238a0fbd182110ea62e6abf0f848fa46083d618073e68bd2a3845855f16","license":"MIT OR Apache-2.0","urls":["bzz-raw://fc311c80d2bc2aa46cfe557b27af7fa4cb153977f3d50d1e704799708c3545ab","dweb:/ipfs/Qmebic4t1BqVGc7PmVgDG18uA1kUTjn6wP54yPcoWjN9ea"]},"contracts/isms/multisig/AbstractMessageIdMultisigIsm.sol":{"keccak256":"0x0d7eb241c6da40a3449bc81d02806718784749d16e591bb6f7e3667a683dffdf","license":"MIT OR Apache-2.0","urls":["bzz-raw://f136f852d61b97642c898b527bd23d99e93baf82f8d8f91a74c790e23359b2e1","dweb:/ipfs/QmQiGR6Dg9PEUTbJnA9QQqDpRKQsFwkTw2sGRuqFqNCZE1"]},"contracts/isms/multisig/AbstractMultisigIsm.sol":{"keccak256":"0x42090e87be4d24c0dbb1da731af40f1fcd48bc3748a7bf95051c652713cccbc6","license":"MIT OR Apache-2.0","urls":["bzz-raw://8253ca6b02d70488d2df8c5811277f05a68f5eff6b667c9a9b2bcda235b220ac","dweb:/ipfs/QmXQfvQUTesMvH9U8dCMrqf6SvSmQLTsa2Y1gzPSbaikvh"]},"contracts/isms/multisig/StaticMultisigIsm.sol":{"keccak256":"0x02f3a5f3f65dfa784c9bc4e29ffce28795d3879eba83367ece0b4f1ac3243349","license":"MIT OR Apache-2.0","urls":["bzz-raw://8f7a2b1634c2606a76798b7dee1b03cb3c15ce9b5f6d99b30437967fac56f0ac","dweb:/ipfs/QmeZdrxaanYCx5XxJvJpE4kxYTqAteKhf4T61WCPDB6Cn6"]},"contracts/libs/CheckpointLib.sol":{"keccak256":"0x2ffee95803acbb0fc19dc8c339e7f06d7fab2474daffaf5b9237162a7962b4d4","license":"MIT OR Apache-2.0","urls":["bzz-raw://ccea883a098c5f47f34310a3acf256aadcca0d221f8b686f7fcbf9a64339b4be","dweb:/ipfs/Qmc7faVY4mkWM3cndij2WxrH6wZUQrnS62Fyzjb3s27YhV"]},"contracts/libs/Merkle.sol":{"keccak256":"0x7cfc607a9b1614522237bcc02f647e4bf0ebf5aa22b76c4ac2fbbec74c7d67f9","license":"MIT OR Apache-2.0","urls":["bzz-raw://0b32ebf16293ac2f103fb946853ea8671e9d8d8c1b445356e74ad901eb28f69c","dweb:/ipfs/QmeuRgcNgN764Hj47dMyNQ3wewSqYjc2HTdzQLDofiuuMA"]},"contracts/libs/Message.sol":{"keccak256":"0x5319af5ade07bcb021d6373f0713f24f34431def78cda16fea6755f580fb5e78","license":"MIT OR Apache-2.0","urls":["bzz-raw://f531eecebd22a535398def83b078aff4f72d88940a0e9474ad3244a555f00d53","dweb:/ipfs/QmQjtjpf4qwaM9QVyWBsogRvZeokKESTVdsX3z9mt1wZFa"]},"contracts/libs/MetaProxy.sol":{"keccak256":"0x7e4a543d24b7bd4c8e0a96f03be6e275ba4acdc5bf220a17428afefcaed27b54","license":"CC0-1.0","urls":["bzz-raw://8113f5c60f7ad022a75b64315ddf400c2babd9ddf2fd9e67ddc0709ddf218388","dweb:/ipfs/QmRHrwChnVbvoSBFE3oUGZiUSRVD1DPJP8t9coZ9XRHJSn"]},"contracts/libs/StaticAddressSetFactory.sol":{"keccak256":"0x179dba222921cfcbf68879fa63bb4eb3659eb9c795bd48a20f6934762bd3a3da","license":"MIT OR Apache-2.0","urls":["bzz-raw://d12e36b93413b7c71129ea07ea5ba681adcb7165ee629a5cffb31f9605753209","dweb:/ipfs/QmS8seehj2nTv9EocdNpzoWhjX2GYAatChSNBPbsDz7kc1"]},"contracts/libs/TypeCasts.sol":{"keccak256":"0x7e3608c35e1109fc137cea688760f9c10223b1e7d110be113ad44ff716974ead","license":"MIT OR Apache-2.0","urls":["bzz-raw://6fff310e714caca96e809f5a56884a2e9edf842d928929d6228d6593ad8f3d52","dweb:/ipfs/QmSHQmcZV2GF8CUJjny128rDThPEDvUvrfA5kBs7kAK5GJ"]}},"version":1}',
  metadata: {
    compiler: { version: "0.8.19+commit.7dd6d404" },
    language: "Solidity",
    output: {
      abi: [
        {
          inputs: [
            { internalType: "address[]", name: "_values", type: "address[]" },
            { internalType: "uint8", name: "_threshold", type: "uint8" },
          ],
          stateMutability: "nonpayable",
          type: "function",
          name: "deploy",
          outputs: [{ internalType: "address", name: "", type: "address" }],
        },
        {
          inputs: [
            { internalType: "address[]", name: "_values", type: "address[]" },
            { internalType: "uint8", name: "_threshold", type: "uint8" },
          ],
          stateMutability: "view",
          type: "function",
          name: "getAddress",
          outputs: [{ internalType: "address", name: "", type: "address" }],
        },
        {
          inputs: [],
          stateMutability: "view",
          type: "function",
          name: "implementation",
          outputs: [{ internalType: "address", name: "", type: "address" }],
        },
      ],
      devdoc: {
        kind: "dev",
        methods: {
          "deploy(address[],uint8)": {
            details: "Consider sorting addresses to ensure contract reuse",
            params: {
              _threshold: "The threshold value to use",
              _values: "An array of addresses",
            },
            returns: {
              _0: "set The contract address representing this StaticThresholdAddressSet",
            },
          },
          "getAddress(address[],uint8)": {
            details: "Consider sorting addresses to ensure contract reuse",
            params: {
              _threshold: "The threshold value to use",
              _values: "An array of addresses",
            },
            returns: {
              _0: "set The contract address representing this StaticThresholdAddressSet",
            },
          },
        },
        version: 1,
      },
      userdoc: {
        kind: "user",
        methods: {
          "deploy(address[],uint8)": {
            notice:
              "Deploys a StaticThresholdAddressSet contract address for the given values",
          },
          "getAddress(address[],uint8)": {
            notice:
              "Returns the StaticThresholdAddressSet contract address for the given values",
          },
        },
        version: 1,
      },
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
        "contracts/isms/multisig/StaticMultisigIsm.sol":
          "StaticMessageIdMultisigIsmFactory",
      },
      evmVersion: "paris",
      libraries: {},
    },
    sources: {
      "../node_modules/@openzeppelin/contracts/utils/Address.sol": {
        keccak256:
          "0x006dd67219697fe68d7fbfdea512e7c4cb64a43565ed86171d67e844982da6fa",
        urls: [
          "bzz-raw://2455248c8ddd9cc6a7af76a13973cddf222072427e7b0e2a7d1aff345145e931",
          "dweb:/ipfs/QmfYjnjRbWqYpuxurqveE6HtzsY1Xx323J428AKQgtBJZm",
        ],
        license: "MIT",
      },
      "../node_modules/@openzeppelin/contracts/utils/Create2.sol": {
        keccak256:
          "0x6e00f269073ffc4350e56b7e8153c9092d5f70bfba423299990514183101ef89",
        urls: [
          "bzz-raw://8dc5ff5c204604be7594ce2fa9100a0638f22b5aada6ebec1e165b0f50a48447",
          "dweb:/ipfs/QmfH7uWphPrq8jqwGvyh4pFnLEgEPmLwyGV2Bvx1Acjggh",
        ],
        license: "MIT",
      },
      "../node_modules/@openzeppelin/contracts/utils/Strings.sol": {
        keccak256:
          "0x3088eb2868e8d13d89d16670b5f8612c4ab9ff8956272837d8e90106c59c14a0",
        urls: [
          "bzz-raw://b81d9ff6559ea5c47fc573e17ece6d9ba5d6839e213e6ebc3b4c5c8fe4199d7f",
          "dweb:/ipfs/QmPCW1bFisUzJkyjroY3yipwfism9RRCigCcK1hbXtVM8n",
        ],
        license: "MIT",
      },
      "../node_modules/@openzeppelin/contracts/utils/cryptography/ECDSA.sol": {
        keccak256:
          "0x809bc3edb4bcbef8263fa616c1b60ee0004b50a8a1bfa164d8f57fd31f520c58",
        urls: [
          "bzz-raw://8b93a1e39a4a19eba1600b92c96f435442db88cac91e315c8291547a2a7bcfe2",
          "dweb:/ipfs/QmTm34KVe6uZBZwq8dZDNWwPcm24qBJdxqL3rPxBJ4LrMv",
        ],
        license: "MIT",
      },
      "../node_modules/@openzeppelin/contracts/utils/math/Math.sol": {
        keccak256:
          "0xe4455ac1eb7fc497bb7402579e7b4d64d928b846fce7d2b6fde06d366f21c2b3",
        urls: [
          "bzz-raw://cc8841b3cd48ad125e2f46323c8bad3aa0e88e399ec62acb9e57efa7e7c8058c",
          "dweb:/ipfs/QmSqE4mXHA2BXW58deDbXE8MTcsL5JSKNDbm23sVQxRLPS",
        ],
        license: "MIT",
      },
      "../node_modules/@openzeppelin/contracts/utils/math/SignedMath.sol": {
        keccak256:
          "0xf92515413956f529d95977adc9b0567d583c6203fc31ab1c23824c35187e3ddc",
        urls: [
          "bzz-raw://c50fcc459e49a9858b6d8ad5f911295cb7c9ab57567845a250bf0153f84a95c7",
          "dweb:/ipfs/QmcEW85JRzvDkQggxiBBLVAasXWdkhEysqypj9EaB6H2g6",
        ],
        license: "MIT",
      },
      "contracts/interfaces/IInterchainSecurityModule.sol": {
        keccak256:
          "0xd144a00a15eef18012f8dae28ca1a7d965012f88ff4a07382a172ae6e02811ce",
        urls: [
          "bzz-raw://045da4f74529b4c0252743fe3f84e96d7e6e41cf30632011eefbb5b33eaa4cb2",
          "dweb:/ipfs/QmefWSKyXSndAGC38tszRfdXbo4kvSsgcq7oTZz9rQ9n2p",
        ],
        license: "MIT OR Apache-2.0",
      },
      "contracts/interfaces/isms/IMultisigIsm.sol": {
        keccak256:
          "0x4815639fb16bc00a611ae978bbff47810dfd76b2b38b4613d7d893a7c35c4d34",
        urls: [
          "bzz-raw://c386374a9e1154b027f8e921de9b333f8b6b7f6724e01ccb1d3c3fbca8aca7b4",
          "dweb:/ipfs/QmQjPQFL1DSmqbyFFP49GgfEezzQBjz3c8SbThVADZi1Ui",
        ],
        license: "MIT OR Apache-2.0",
      },
      "contracts/isms/libs/MerkleRootMultisigIsmMetadata.sol": {
        keccak256:
          "0xb45298ff24fd4f12f2aeb93f6ab700749ef9952238c7693922bc8e6cca457395",
        urls: [
          "bzz-raw://50a248b5d89c078b4d80a282c19292d5ef68770949ff9ec4c76275e6dbba8764",
          "dweb:/ipfs/QmW1PCYZtaQBocy5yz3VPZK1YTSnwH9UWYnzcRVHn5rLR6",
        ],
        license: "MIT OR Apache-2.0",
      },
      "contracts/isms/libs/MessageIdMultisigIsmMetadata.sol": {
        keccak256:
          "0x6a5cd7301d1e1faa6c30a988c9f5235eb9b9c6e334b5bcc06c153df79687afb6",
        urls: [
          "bzz-raw://af34bcfc9d7043ef1d83b8f79da13f9cefbf634cbb423ea0fc67b431e20a84fb",
          "dweb:/ipfs/QmVoae2thC5eALT6d9h1Jtp6X9WgjvrPT2c4Cw2iXVkRmp",
        ],
        license: "MIT OR Apache-2.0",
      },
      "contracts/isms/multisig/AbstractMerkleRootMultisigIsm.sol": {
        keccak256:
          "0xa3ada238a0fbd182110ea62e6abf0f848fa46083d618073e68bd2a3845855f16",
        urls: [
          "bzz-raw://fc311c80d2bc2aa46cfe557b27af7fa4cb153977f3d50d1e704799708c3545ab",
          "dweb:/ipfs/Qmebic4t1BqVGc7PmVgDG18uA1kUTjn6wP54yPcoWjN9ea",
        ],
        license: "MIT OR Apache-2.0",
      },
      "contracts/isms/multisig/AbstractMessageIdMultisigIsm.sol": {
        keccak256:
          "0x0d7eb241c6da40a3449bc81d02806718784749d16e591bb6f7e3667a683dffdf",
        urls: [
          "bzz-raw://f136f852d61b97642c898b527bd23d99e93baf82f8d8f91a74c790e23359b2e1",
          "dweb:/ipfs/QmQiGR6Dg9PEUTbJnA9QQqDpRKQsFwkTw2sGRuqFqNCZE1",
        ],
        license: "MIT OR Apache-2.0",
      },
      "contracts/isms/multisig/AbstractMultisigIsm.sol": {
        keccak256:
          "0x42090e87be4d24c0dbb1da731af40f1fcd48bc3748a7bf95051c652713cccbc6",
        urls: [
          "bzz-raw://8253ca6b02d70488d2df8c5811277f05a68f5eff6b667c9a9b2bcda235b220ac",
          "dweb:/ipfs/QmXQfvQUTesMvH9U8dCMrqf6SvSmQLTsa2Y1gzPSbaikvh",
        ],
        license: "MIT OR Apache-2.0",
      },
      "contracts/isms/multisig/StaticMultisigIsm.sol": {
        keccak256:
          "0x02f3a5f3f65dfa784c9bc4e29ffce28795d3879eba83367ece0b4f1ac3243349",
        urls: [
          "bzz-raw://8f7a2b1634c2606a76798b7dee1b03cb3c15ce9b5f6d99b30437967fac56f0ac",
          "dweb:/ipfs/QmeZdrxaanYCx5XxJvJpE4kxYTqAteKhf4T61WCPDB6Cn6",
        ],
        license: "MIT OR Apache-2.0",
      },
      "contracts/libs/CheckpointLib.sol": {
        keccak256:
          "0x2ffee95803acbb0fc19dc8c339e7f06d7fab2474daffaf5b9237162a7962b4d4",
        urls: [
          "bzz-raw://ccea883a098c5f47f34310a3acf256aadcca0d221f8b686f7fcbf9a64339b4be",
          "dweb:/ipfs/Qmc7faVY4mkWM3cndij2WxrH6wZUQrnS62Fyzjb3s27YhV",
        ],
        license: "MIT OR Apache-2.0",
      },
      "contracts/libs/Merkle.sol": {
        keccak256:
          "0x7cfc607a9b1614522237bcc02f647e4bf0ebf5aa22b76c4ac2fbbec74c7d67f9",
        urls: [
          "bzz-raw://0b32ebf16293ac2f103fb946853ea8671e9d8d8c1b445356e74ad901eb28f69c",
          "dweb:/ipfs/QmeuRgcNgN764Hj47dMyNQ3wewSqYjc2HTdzQLDofiuuMA",
        ],
        license: "MIT OR Apache-2.0",
      },
      "contracts/libs/Message.sol": {
        keccak256:
          "0x5319af5ade07bcb021d6373f0713f24f34431def78cda16fea6755f580fb5e78",
        urls: [
          "bzz-raw://f531eecebd22a535398def83b078aff4f72d88940a0e9474ad3244a555f00d53",
          "dweb:/ipfs/QmQjtjpf4qwaM9QVyWBsogRvZeokKESTVdsX3z9mt1wZFa",
        ],
        license: "MIT OR Apache-2.0",
      },
      "contracts/libs/MetaProxy.sol": {
        keccak256:
          "0x7e4a543d24b7bd4c8e0a96f03be6e275ba4acdc5bf220a17428afefcaed27b54",
        urls: [
          "bzz-raw://8113f5c60f7ad022a75b64315ddf400c2babd9ddf2fd9e67ddc0709ddf218388",
          "dweb:/ipfs/QmRHrwChnVbvoSBFE3oUGZiUSRVD1DPJP8t9coZ9XRHJSn",
        ],
        license: "CC0-1.0",
      },
      "contracts/libs/StaticAddressSetFactory.sol": {
        keccak256:
          "0x179dba222921cfcbf68879fa63bb4eb3659eb9c795bd48a20f6934762bd3a3da",
        urls: [
          "bzz-raw://d12e36b93413b7c71129ea07ea5ba681adcb7165ee629a5cffb31f9605753209",
          "dweb:/ipfs/QmS8seehj2nTv9EocdNpzoWhjX2GYAatChSNBPbsDz7kc1",
        ],
        license: "MIT OR Apache-2.0",
      },
      "contracts/libs/TypeCasts.sol": {
        keccak256:
          "0x7e3608c35e1109fc137cea688760f9c10223b1e7d110be113ad44ff716974ead",
        urls: [
          "bzz-raw://6fff310e714caca96e809f5a56884a2e9edf842d928929d6228d6593ad8f3d52",
          "dweb:/ipfs/QmSHQmcZV2GF8CUJjny128rDThPEDvUvrfA5kBs7kAK5GJ",
        ],
        license: "MIT OR Apache-2.0",
      },
    },
    version: 1,
  },
  ast: {
    absolutePath: "contracts/isms/multisig/StaticMultisigIsm.sol",
    id: 22612,
    exportedSymbols: {
      AbstractMerkleRootMultisigIsm: [22302],
      AbstractMessageIdMultisigIsm: [22380],
      AbstractMetaProxyMultisigIsm: [22563],
      AbstractMultisigIsm: [22521],
      MetaProxy: [24653],
      StaticMerkleRootMultisigIsm: [22569],
      StaticMerkleRootMultisigIsmFactory: [22593],
      StaticMessageIdMultisigIsm: [22575],
      StaticMessageIdMultisigIsmFactory: [22611],
      StaticThresholdAddressSetFactory: [24855],
    },
    nodeType: "SourceUnit",
    src: "46:2050:165",
    nodes: [
      {
        id: 22523,
        nodeType: "PragmaDirective",
        src: "46:24:165",
        nodes: [],
        literals: ["solidity", ">=", "0.8", ".0"],
      },
      {
        id: 22525,
        nodeType: "ImportDirective",
        src: "117:62:165",
        nodes: [],
        absolutePath: "contracts/isms/multisig/AbstractMultisigIsm.sol",
        file: "./AbstractMultisigIsm.sol",
        nameLocation: "-1:-1:-1",
        scope: 22612,
        sourceUnit: 22522,
        symbolAliases: [
          {
            foreign: {
              id: 22524,
              name: "AbstractMultisigIsm",
              nodeType: "Identifier",
              overloadedDeclarations: [],
              referencedDeclaration: 22521,
              src: "125:19:165",
              typeDescriptions: {},
            },
            nameLocation: "-1:-1:-1",
          },
        ],
        unitAlias: "",
      },
      {
        id: 22527,
        nodeType: "ImportDirective",
        src: "180:82:165",
        nodes: [],
        absolutePath:
          "contracts/isms/multisig/AbstractMerkleRootMultisigIsm.sol",
        file: "./AbstractMerkleRootMultisigIsm.sol",
        nameLocation: "-1:-1:-1",
        scope: 22612,
        sourceUnit: 22303,
        symbolAliases: [
          {
            foreign: {
              id: 22526,
              name: "AbstractMerkleRootMultisigIsm",
              nodeType: "Identifier",
              overloadedDeclarations: [],
              referencedDeclaration: 22302,
              src: "188:29:165",
              typeDescriptions: {},
            },
            nameLocation: "-1:-1:-1",
          },
        ],
        unitAlias: "",
      },
      {
        id: 22529,
        nodeType: "ImportDirective",
        src: "263:80:165",
        nodes: [],
        absolutePath:
          "contracts/isms/multisig/AbstractMessageIdMultisigIsm.sol",
        file: "./AbstractMessageIdMultisigIsm.sol",
        nameLocation: "-1:-1:-1",
        scope: 22612,
        sourceUnit: 22381,
        symbolAliases: [
          {
            foreign: {
              id: 22528,
              name: "AbstractMessageIdMultisigIsm",
              nodeType: "Identifier",
              overloadedDeclarations: [],
              referencedDeclaration: 22380,
              src: "271:28:165",
              typeDescriptions: {},
            },
            nameLocation: "-1:-1:-1",
          },
        ],
        unitAlias: "",
      },
      {
        id: 22531,
        nodeType: "ImportDirective",
        src: "344:51:165",
        nodes: [],
        absolutePath: "contracts/libs/MetaProxy.sol",
        file: "../../libs/MetaProxy.sol",
        nameLocation: "-1:-1:-1",
        scope: 22612,
        sourceUnit: 24654,
        symbolAliases: [
          {
            foreign: {
              id: 22530,
              name: "MetaProxy",
              nodeType: "Identifier",
              overloadedDeclarations: [],
              referencedDeclaration: 24653,
              src: "352:9:165",
              typeDescriptions: {},
            },
            nameLocation: "-1:-1:-1",
          },
        ],
        unitAlias: "",
      },
      {
        id: 22533,
        nodeType: "ImportDirective",
        src: "396:88:165",
        nodes: [],
        absolutePath: "contracts/libs/StaticAddressSetFactory.sol",
        file: "../../libs/StaticAddressSetFactory.sol",
        nameLocation: "-1:-1:-1",
        scope: 22612,
        sourceUnit: 24908,
        symbolAliases: [
          {
            foreign: {
              id: 22532,
              name: "StaticThresholdAddressSetFactory",
              nodeType: "Identifier",
              overloadedDeclarations: [],
              referencedDeclaration: 24855,
              src: "404:32:165",
              typeDescriptions: {},
            },
            nameLocation: "-1:-1:-1",
          },
        ],
        unitAlias: "",
      },
      {
        id: 22563,
        nodeType: "ContractDefinition",
        src: "631:326:165",
        nodes: [
          {
            id: 22562,
            nodeType: "FunctionDefinition",
            src: "762:193:165",
            nodes: [],
            body: {
              id: 22561,
              nodeType: "Block",
              src: "879:76:165",
              nodes: [],
              statements: [
                {
                  expression: {
                    arguments: [
                      {
                        arguments: [],
                        expression: {
                          argumentTypes: [],
                          expression: {
                            id: 22550,
                            name: "MetaProxy",
                            nodeType: "Identifier",
                            overloadedDeclarations: [],
                            referencedDeclaration: 24653,
                            src: "907:9:165",
                            typeDescriptions: {
                              typeIdentifier:
                                "t_type$_t_contract$_MetaProxy_$24653_$",
                              typeString: "type(library MetaProxy)",
                            },
                          },
                          id: 22551,
                          isConstant: false,
                          isLValue: false,
                          isPure: false,
                          lValueRequested: false,
                          memberLocation: "917:8:165",
                          memberName: "metadata",
                          nodeType: "MemberAccess",
                          referencedDeclaration: 24652,
                          src: "907:18:165",
                          typeDescriptions: {
                            typeIdentifier:
                              "t_function_internal_pure$__$returns$_t_bytes_memory_ptr_$",
                            typeString:
                              "function () pure returns (bytes memory)",
                          },
                        },
                        id: 22552,
                        isConstant: false,
                        isLValue: false,
                        isPure: false,
                        kind: "functionCall",
                        lValueRequested: false,
                        nameLocations: [],
                        names: [],
                        nodeType: "FunctionCall",
                        src: "907:20:165",
                        tryCall: false,
                        typeDescriptions: {
                          typeIdentifier: "t_bytes_memory_ptr",
                          typeString: "bytes memory",
                        },
                      },
                      {
                        components: [
                          {
                            baseExpression: {
                              id: 22554,
                              isConstant: false,
                              isLValue: false,
                              isPure: true,
                              lValueRequested: false,
                              nodeType: "ElementaryTypeNameExpression",
                              src: "930:7:165",
                              typeDescriptions: {
                                typeIdentifier: "t_type$_t_address_$",
                                typeString: "type(address)",
                              },
                              typeName: {
                                id: 22553,
                                name: "address",
                                nodeType: "ElementaryTypeName",
                                src: "930:7:165",
                                typeDescriptions: {},
                              },
                            },
                            id: 22555,
                            isConstant: false,
                            isLValue: false,
                            isPure: true,
                            lValueRequested: false,
                            nodeType: "IndexAccess",
                            src: "930:9:165",
                            typeDescriptions: {
                              typeIdentifier:
                                "t_type$_t_array$_t_address_$dyn_memory_ptr_$",
                              typeString: "type(address[] memory)",
                            },
                          },
                          {
                            id: 22557,
                            isConstant: false,
                            isLValue: false,
                            isPure: true,
                            lValueRequested: false,
                            nodeType: "ElementaryTypeNameExpression",
                            src: "941:5:165",
                            typeDescriptions: {
                              typeIdentifier: "t_type$_t_uint8_$",
                              typeString: "type(uint8)",
                            },
                            typeName: {
                              id: 22556,
                              name: "uint8",
                              nodeType: "ElementaryTypeName",
                              src: "941:5:165",
                              typeDescriptions: {},
                            },
                          },
                        ],
                        id: 22558,
                        isConstant: false,
                        isInlineArray: false,
                        isLValue: false,
                        isPure: true,
                        lValueRequested: false,
                        nodeType: "TupleExpression",
                        src: "929:18:165",
                        typeDescriptions: {
                          typeIdentifier:
                            "t_tuple$_t_type$_t_array$_t_address_$dyn_memory_ptr_$_$_t_type$_t_uint8_$_$",
                          typeString:
                            "tuple(type(address[] memory),type(uint8))",
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
                          typeIdentifier:
                            "t_tuple$_t_type$_t_array$_t_address_$dyn_memory_ptr_$_$_t_type$_t_uint8_$_$",
                          typeString:
                            "tuple(type(address[] memory),type(uint8))",
                        },
                      ],
                      expression: {
                        id: 22548,
                        name: "abi",
                        nodeType: "Identifier",
                        overloadedDeclarations: [],
                        referencedDeclaration: -1,
                        src: "896:3:165",
                        typeDescriptions: {
                          typeIdentifier: "t_magic_abi",
                          typeString: "abi",
                        },
                      },
                      id: 22549,
                      isConstant: false,
                      isLValue: false,
                      isPure: true,
                      lValueRequested: false,
                      memberLocation: "900:6:165",
                      memberName: "decode",
                      nodeType: "MemberAccess",
                      src: "896:10:165",
                      typeDescriptions: {
                        typeIdentifier:
                          "t_function_abidecode_pure$__$returns$__$",
                        typeString: "function () pure",
                      },
                    },
                    id: 22559,
                    isConstant: false,
                    isLValue: false,
                    isPure: false,
                    kind: "functionCall",
                    lValueRequested: false,
                    nameLocations: [],
                    names: [],
                    nodeType: "FunctionCall",
                    src: "896:52:165",
                    tryCall: false,
                    typeDescriptions: {
                      typeIdentifier:
                        "t_tuple$_t_array$_t_address_$dyn_memory_ptr_$_t_uint8_$",
                      typeString: "tuple(address[] memory,uint8)",
                    },
                  },
                  functionReturnParameters: 22547,
                  id: 22560,
                  nodeType: "Return",
                  src: "889:59:165",
                },
              ],
            },
            baseFunctions: [22406],
            documentation: {
              id: 22537,
              nodeType: "StructuredDocumentation",
              src: "707:50:165",
              text: " @inheritdoc AbstractMultisigIsm",
            },
            functionSelector: "2e0ed234",
            implemented: true,
            kind: "function",
            modifiers: [],
            name: "validatorsAndThreshold",
            nameLocation: "771:22:165",
            overrides: {
              id: 22541,
              nodeType: "OverrideSpecifier",
              overrides: [],
              src: "836:8:165",
            },
            parameters: {
              id: 22540,
              nodeType: "ParameterList",
              parameters: [
                {
                  constant: false,
                  id: 22539,
                  mutability: "mutable",
                  name: "",
                  nameLocation: "-1:-1:-1",
                  nodeType: "VariableDeclaration",
                  scope: 22562,
                  src: "803:14:165",
                  stateVariable: false,
                  storageLocation: "calldata",
                  typeDescriptions: {
                    typeIdentifier: "t_bytes_calldata_ptr",
                    typeString: "bytes",
                  },
                  typeName: {
                    id: 22538,
                    name: "bytes",
                    nodeType: "ElementaryTypeName",
                    src: "803:5:165",
                    typeDescriptions: {
                      typeIdentifier: "t_bytes_storage_ptr",
                      typeString: "bytes",
                    },
                  },
                  visibility: "internal",
                },
              ],
              src: "793:30:165",
            },
            returnParameters: {
              id: 22547,
              nodeType: "ParameterList",
              parameters: [
                {
                  constant: false,
                  id: 22544,
                  mutability: "mutable",
                  name: "",
                  nameLocation: "-1:-1:-1",
                  nodeType: "VariableDeclaration",
                  scope: 22562,
                  src: "854:16:165",
                  stateVariable: false,
                  storageLocation: "memory",
                  typeDescriptions: {
                    typeIdentifier: "t_array$_t_address_$dyn_memory_ptr",
                    typeString: "address[]",
                  },
                  typeName: {
                    baseType: {
                      id: 22542,
                      name: "address",
                      nodeType: "ElementaryTypeName",
                      src: "854:7:165",
                      stateMutability: "nonpayable",
                      typeDescriptions: {
                        typeIdentifier: "t_address",
                        typeString: "address",
                      },
                    },
                    id: 22543,
                    nodeType: "ArrayTypeName",
                    src: "854:9:165",
                    typeDescriptions: {
                      typeIdentifier: "t_array$_t_address_$dyn_storage_ptr",
                      typeString: "address[]",
                    },
                  },
                  visibility: "internal",
                },
                {
                  constant: false,
                  id: 22546,
                  mutability: "mutable",
                  name: "",
                  nameLocation: "-1:-1:-1",
                  nodeType: "VariableDeclaration",
                  scope: 22562,
                  src: "872:5:165",
                  stateVariable: false,
                  storageLocation: "default",
                  typeDescriptions: {
                    typeIdentifier: "t_uint8",
                    typeString: "uint8",
                  },
                  typeName: {
                    id: 22545,
                    name: "uint8",
                    nodeType: "ElementaryTypeName",
                    src: "872:5:165",
                    typeDescriptions: {
                      typeIdentifier: "t_uint8",
                      typeString: "uint8",
                    },
                  },
                  visibility: "internal",
                },
              ],
              src: "853:25:165",
            },
            scope: 22563,
            stateMutability: "pure",
            virtual: false,
            visibility: "public",
          },
        ],
        abstract: true,
        baseContracts: [
          {
            baseName: {
              id: 22535,
              name: "AbstractMultisigIsm",
              nameLocations: ["681:19:165"],
              nodeType: "IdentifierPath",
              referencedDeclaration: 22521,
              src: "681:19:165",
            },
            id: 22536,
            nodeType: "InheritanceSpecifier",
            src: "681:19:165",
          },
        ],
        canonicalName: "AbstractMetaProxyMultisigIsm",
        contractDependencies: [],
        contractKind: "contract",
        documentation: {
          id: 22534,
          nodeType: "StructuredDocumentation",
          src: "486:144:165",
          text: " @title AbstractMetaProxyMultisigIsm\n @notice Manages per-domain m-of-n Validator set that is used\n to verify interchain messages.",
        },
        fullyImplemented: false,
        linearizedBaseContracts: [22563, 22521, 21152, 20729],
        name: "AbstractMetaProxyMultisigIsm",
        nameLocation: "649:28:165",
        scope: 22612,
        usedErrors: [],
      },
      {
        id: 22569,
        nodeType: "ContractDefinition",
        src: "1209:110:165",
        nodes: [],
        abstract: false,
        baseContracts: [
          {
            baseName: {
              id: 22565,
              name: "AbstractMerkleRootMultisigIsm",
              nameLocations: ["1253:29:165"],
              nodeType: "IdentifierPath",
              referencedDeclaration: 22302,
              src: "1253:29:165",
            },
            id: 22566,
            nodeType: "InheritanceSpecifier",
            src: "1253:29:165",
          },
          {
            baseName: {
              id: 22567,
              name: "AbstractMetaProxyMultisigIsm",
              nameLocations: ["1288:28:165"],
              nodeType: "IdentifierPath",
              referencedDeclaration: 22563,
              src: "1288:28:165",
            },
            id: 22568,
            nodeType: "InheritanceSpecifier",
            src: "1288:28:165",
          },
        ],
        canonicalName: "StaticMerkleRootMultisigIsm",
        contractDependencies: [],
        contractKind: "contract",
        documentation: {
          id: 22564,
          nodeType: "StructuredDocumentation",
          src: "995:213:165",
          text: " @title StaticMerkleRootMultisigIsm\n @notice Manages per-domain m-of-n validator set that is used\n to verify interchain messages using a merkle root signature quorum\n and merkle proof of inclusion.",
        },
        fullyImplemented: true,
        linearizedBaseContracts: [22569, 22563, 22302, 22521, 21152, 20729],
        name: "StaticMerkleRootMultisigIsm",
        nameLocation: "1218:27:165",
        scope: 22612,
        usedErrors: [],
      },
      {
        id: 22575,
        nodeType: "ContractDefinition",
        src: "1500:108:165",
        nodes: [],
        abstract: false,
        baseContracts: [
          {
            baseName: {
              id: 22571,
              name: "AbstractMessageIdMultisigIsm",
              nameLocations: ["1543:28:165"],
              nodeType: "IdentifierPath",
              referencedDeclaration: 22380,
              src: "1543:28:165",
            },
            id: 22572,
            nodeType: "InheritanceSpecifier",
            src: "1543:28:165",
          },
          {
            baseName: {
              id: 22573,
              name: "AbstractMetaProxyMultisigIsm",
              nameLocations: ["1577:28:165"],
              nodeType: "IdentifierPath",
              referencedDeclaration: 22563,
              src: "1577:28:165",
            },
            id: 22574,
            nodeType: "InheritanceSpecifier",
            src: "1577:28:165",
          },
        ],
        canonicalName: "StaticMessageIdMultisigIsm",
        contractDependencies: [],
        contractKind: "contract",
        documentation: {
          id: 22570,
          nodeType: "StructuredDocumentation",
          src: "1321:178:165",
          text: " @title StaticMessageIdMultisigIsm\n @notice Manages per-domain m-of-n validator set that is used\n to verify interchain messages using a message ID signature quorum.",
        },
        fullyImplemented: true,
        linearizedBaseContracts: [22575, 22563, 22380, 22521, 21152, 20729],
        name: "StaticMessageIdMultisigIsm",
        nameLocation: "1509:26:165",
        scope: 22612,
        usedErrors: [],
      },
      {
        id: 22593,
        nodeType: "ContractDefinition",
        src: "1645:227:165",
        nodes: [
          {
            id: 22592,
            nodeType: "FunctionDefinition",
            src: "1735:135:165",
            nodes: [],
            body: {
              id: 22591,
              nodeType: "Block",
              src: "1804:66:165",
              nodes: [],
              statements: [
                {
                  expression: {
                    arguments: [
                      {
                        arguments: [],
                        expression: {
                          argumentTypes: [],
                          id: 22587,
                          isConstant: false,
                          isLValue: false,
                          isPure: false,
                          lValueRequested: false,
                          nodeType: "NewExpression",
                          src: "1829:31:165",
                          typeDescriptions: {
                            typeIdentifier:
                              "t_function_creation_nonpayable$__$returns$_t_contract$_StaticMerkleRootMultisigIsm_$22569_$",
                            typeString:
                              "function () returns (contract StaticMerkleRootMultisigIsm)",
                          },
                          typeName: {
                            id: 22586,
                            nodeType: "UserDefinedTypeName",
                            pathNode: {
                              id: 22585,
                              name: "StaticMerkleRootMultisigIsm",
                              nameLocations: ["1833:27:165"],
                              nodeType: "IdentifierPath",
                              referencedDeclaration: 22569,
                              src: "1833:27:165",
                            },
                            referencedDeclaration: 22569,
                            src: "1833:27:165",
                            typeDescriptions: {
                              typeIdentifier:
                                "t_contract$_StaticMerkleRootMultisigIsm_$22569",
                              typeString:
                                "contract StaticMerkleRootMultisigIsm",
                            },
                          },
                        },
                        id: 22588,
                        isConstant: false,
                        isLValue: false,
                        isPure: false,
                        kind: "functionCall",
                        lValueRequested: false,
                        nameLocations: [],
                        names: [],
                        nodeType: "FunctionCall",
                        src: "1829:33:165",
                        tryCall: false,
                        typeDescriptions: {
                          typeIdentifier:
                            "t_contract$_StaticMerkleRootMultisigIsm_$22569",
                          typeString: "contract StaticMerkleRootMultisigIsm",
                        },
                      },
                    ],
                    expression: {
                      argumentTypes: [
                        {
                          typeIdentifier:
                            "t_contract$_StaticMerkleRootMultisigIsm_$22569",
                          typeString: "contract StaticMerkleRootMultisigIsm",
                        },
                      ],
                      id: 22584,
                      isConstant: false,
                      isLValue: false,
                      isPure: true,
                      lValueRequested: false,
                      nodeType: "ElementaryTypeNameExpression",
                      src: "1821:7:165",
                      typeDescriptions: {
                        typeIdentifier: "t_type$_t_address_$",
                        typeString: "type(address)",
                      },
                      typeName: {
                        id: 22583,
                        name: "address",
                        nodeType: "ElementaryTypeName",
                        src: "1821:7:165",
                        typeDescriptions: {},
                      },
                    },
                    id: 22589,
                    isConstant: false,
                    isLValue: false,
                    isPure: false,
                    kind: "typeConversion",
                    lValueRequested: false,
                    nameLocations: [],
                    names: [],
                    nodeType: "FunctionCall",
                    src: "1821:42:165",
                    tryCall: false,
                    typeDescriptions: {
                      typeIdentifier: "t_address",
                      typeString: "address",
                    },
                  },
                  functionReturnParameters: 22582,
                  id: 22590,
                  nodeType: "Return",
                  src: "1814:49:165",
                },
              ],
            },
            baseFunctions: [24719],
            implemented: true,
            kind: "function",
            modifiers: [],
            name: "_deployImplementation",
            nameLocation: "1744:21:165",
            overrides: {
              id: 22579,
              nodeType: "OverrideSpecifier",
              overrides: [],
              src: "1777:8:165",
            },
            parameters: {
              id: 22578,
              nodeType: "ParameterList",
              parameters: [],
              src: "1765:2:165",
            },
            returnParameters: {
              id: 22582,
              nodeType: "ParameterList",
              parameters: [
                {
                  constant: false,
                  id: 22581,
                  mutability: "mutable",
                  name: "",
                  nameLocation: "-1:-1:-1",
                  nodeType: "VariableDeclaration",
                  scope: 22592,
                  src: "1795:7:165",
                  stateVariable: false,
                  storageLocation: "default",
                  typeDescriptions: {
                    typeIdentifier: "t_address",
                    typeString: "address",
                  },
                  typeName: {
                    id: 22580,
                    name: "address",
                    nodeType: "ElementaryTypeName",
                    src: "1795:7:165",
                    stateMutability: "nonpayable",
                    typeDescriptions: {
                      typeIdentifier: "t_address",
                      typeString: "address",
                    },
                  },
                  visibility: "internal",
                },
              ],
              src: "1794:9:165",
            },
            scope: 22593,
            stateMutability: "nonpayable",
            virtual: false,
            visibility: "internal",
          },
        ],
        abstract: false,
        baseContracts: [
          {
            baseName: {
              id: 22576,
              name: "StaticThresholdAddressSetFactory",
              nameLocations: ["1696:32:165"],
              nodeType: "IdentifierPath",
              referencedDeclaration: 24855,
              src: "1696:32:165",
            },
            id: 22577,
            nodeType: "InheritanceSpecifier",
            src: "1696:32:165",
          },
        ],
        canonicalName: "StaticMerkleRootMultisigIsmFactory",
        contractDependencies: [22569],
        contractKind: "contract",
        fullyImplemented: true,
        linearizedBaseContracts: [22593, 24855],
        name: "StaticMerkleRootMultisigIsmFactory",
        nameLocation: "1654:34:165",
        scope: 22612,
        usedErrors: [],
      },
      {
        id: 22611,
        nodeType: "ContractDefinition",
        src: "1874:221:165",
        nodes: [
          {
            id: 22610,
            nodeType: "FunctionDefinition",
            src: "1959:134:165",
            nodes: [],
            body: {
              id: 22609,
              nodeType: "Block",
              src: "2028:65:165",
              nodes: [],
              statements: [
                {
                  expression: {
                    arguments: [
                      {
                        arguments: [],
                        expression: {
                          argumentTypes: [],
                          id: 22605,
                          isConstant: false,
                          isLValue: false,
                          isPure: false,
                          lValueRequested: false,
                          nodeType: "NewExpression",
                          src: "2053:30:165",
                          typeDescriptions: {
                            typeIdentifier:
                              "t_function_creation_nonpayable$__$returns$_t_contract$_StaticMessageIdMultisigIsm_$22575_$",
                            typeString:
                              "function () returns (contract StaticMessageIdMultisigIsm)",
                          },
                          typeName: {
                            id: 22604,
                            nodeType: "UserDefinedTypeName",
                            pathNode: {
                              id: 22603,
                              name: "StaticMessageIdMultisigIsm",
                              nameLocations: ["2057:26:165"],
                              nodeType: "IdentifierPath",
                              referencedDeclaration: 22575,
                              src: "2057:26:165",
                            },
                            referencedDeclaration: 22575,
                            src: "2057:26:165",
                            typeDescriptions: {
                              typeIdentifier:
                                "t_contract$_StaticMessageIdMultisigIsm_$22575",
                              typeString: "contract StaticMessageIdMultisigIsm",
                            },
                          },
                        },
                        id: 22606,
                        isConstant: false,
                        isLValue: false,
                        isPure: false,
                        kind: "functionCall",
                        lValueRequested: false,
                        nameLocations: [],
                        names: [],
                        nodeType: "FunctionCall",
                        src: "2053:32:165",
                        tryCall: false,
                        typeDescriptions: {
                          typeIdentifier:
                            "t_contract$_StaticMessageIdMultisigIsm_$22575",
                          typeString: "contract StaticMessageIdMultisigIsm",
                        },
                      },
                    ],
                    expression: {
                      argumentTypes: [
                        {
                          typeIdentifier:
                            "t_contract$_StaticMessageIdMultisigIsm_$22575",
                          typeString: "contract StaticMessageIdMultisigIsm",
                        },
                      ],
                      id: 22602,
                      isConstant: false,
                      isLValue: false,
                      isPure: true,
                      lValueRequested: false,
                      nodeType: "ElementaryTypeNameExpression",
                      src: "2045:7:165",
                      typeDescriptions: {
                        typeIdentifier: "t_type$_t_address_$",
                        typeString: "type(address)",
                      },
                      typeName: {
                        id: 22601,
                        name: "address",
                        nodeType: "ElementaryTypeName",
                        src: "2045:7:165",
                        typeDescriptions: {},
                      },
                    },
                    id: 22607,
                    isConstant: false,
                    isLValue: false,
                    isPure: false,
                    kind: "typeConversion",
                    lValueRequested: false,
                    nameLocations: [],
                    names: [],
                    nodeType: "FunctionCall",
                    src: "2045:41:165",
                    tryCall: false,
                    typeDescriptions: {
                      typeIdentifier: "t_address",
                      typeString: "address",
                    },
                  },
                  functionReturnParameters: 22600,
                  id: 22608,
                  nodeType: "Return",
                  src: "2038:48:165",
                },
              ],
            },
            baseFunctions: [24719],
            implemented: true,
            kind: "function",
            modifiers: [],
            name: "_deployImplementation",
            nameLocation: "1968:21:165",
            overrides: {
              id: 22597,
              nodeType: "OverrideSpecifier",
              overrides: [],
              src: "2001:8:165",
            },
            parameters: {
              id: 22596,
              nodeType: "ParameterList",
              parameters: [],
              src: "1989:2:165",
            },
            returnParameters: {
              id: 22600,
              nodeType: "ParameterList",
              parameters: [
                {
                  constant: false,
                  id: 22599,
                  mutability: "mutable",
                  name: "",
                  nameLocation: "-1:-1:-1",
                  nodeType: "VariableDeclaration",
                  scope: 22610,
                  src: "2019:7:165",
                  stateVariable: false,
                  storageLocation: "default",
                  typeDescriptions: {
                    typeIdentifier: "t_address",
                    typeString: "address",
                  },
                  typeName: {
                    id: 22598,
                    name: "address",
                    nodeType: "ElementaryTypeName",
                    src: "2019:7:165",
                    stateMutability: "nonpayable",
                    typeDescriptions: {
                      typeIdentifier: "t_address",
                      typeString: "address",
                    },
                  },
                  visibility: "internal",
                },
              ],
              src: "2018:9:165",
            },
            scope: 22611,
            stateMutability: "nonpayable",
            virtual: false,
            visibility: "internal",
          },
        ],
        abstract: false,
        baseContracts: [
          {
            baseName: {
              id: 22594,
              name: "StaticThresholdAddressSetFactory",
              nameLocations: ["1920:32:165"],
              nodeType: "IdentifierPath",
              referencedDeclaration: 24855,
              src: "1920:32:165",
            },
            id: 22595,
            nodeType: "InheritanceSpecifier",
            src: "1920:32:165",
          },
        ],
        canonicalName: "StaticMessageIdMultisigIsmFactory",
        contractDependencies: [22575],
        contractKind: "contract",
        fullyImplemented: true,
        linearizedBaseContracts: [22611, 24855],
        name: "StaticMessageIdMultisigIsmFactory",
        nameLocation: "1883:33:165",
        scope: 22612,
        usedErrors: [],
      },
    ],
    license: "MIT OR Apache-2.0",
  },
  id: 165,
} as const;
