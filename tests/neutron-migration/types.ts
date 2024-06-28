import { CosmWasmClient } from '@cosmjs/cosmwasm-stargate';
import { StargateClient } from '@cosmjs/stargate';
import { Any } from 'cosmjs-types/google/protobuf/any';

export type ClientSet = {
  wasm: CosmWasmClient;
  stargate: StargateClient;
};

export type Member = {
  address: string;
  pubkey?: Any;
  client: ClientSet;
};
